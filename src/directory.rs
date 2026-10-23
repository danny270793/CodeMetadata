use std::fs;
use std::path;
use std::io;
use std::process;

pub fn get_files(path: &String) -> Vec<String> {
    // Inside a git repository only tracked and not ignored files are analyzed
    if path::Path::new(path).join(".git").exists() {
        if let Some(files) = get_git_files(path) {
            return files;
        }
    }

    let mut files: Vec<String> = Vec::<String>::new();
    fs::read_dir(path).unwrap().into_iter().for_each(|file: Result<fs::DirEntry, io::Error>| {
        let path: path::PathBuf = file.unwrap().path();
        add_entry(path.display().to_string(), &mut files);
    });

    files
}

// Returns None when git is not available or fails, so the caller can walk the folder instead
fn get_git_files(path: &String) -> Option<Vec<String>> {
    let output: process::Output = process::Command::new("git")
        .args(["-C", path, "ls-files", "-z", "--cached", "--others", "--exclude-standard"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let mut files: Vec<String> = Vec::<String>::new();
    output.stdout.split(|byte: &u8| *byte == 0).filter(|relative: &&[u8]| !relative.is_empty()).for_each(|relative: &[u8]| {
        // Untracked nested repositories are listed as "folder/"
        let relative: String = String::from_utf8_lossy(relative).trim_end_matches('/').to_string();
        let full_path: path::PathBuf = path::Path::new(path).join(relative);
        add_entry(full_path.display().to_string(), &mut files);
    });

    Some(files)
}

fn add_entry(full_path: String, files: &mut Vec<String>) {
    // Broken symlinks (e.g. Flutter's ephemeral plugin links) and files deleted but still tracked can't be stat'ed, skip them
    let metadata: fs::Metadata = match fs::metadata(&full_path) {
        Ok(metadata) => metadata,
        Err(_) => return,
    };

    let link: Result<path::PathBuf, io::Error> = fs::read_link(&full_path);
    if !link.is_ok() && metadata.is_dir() {
        let mut sub_files: Vec<String> = get_files(&full_path);
        files.append(&mut sub_files);
    }
    if metadata.is_file() {
        files.push(full_path);
    }
}
