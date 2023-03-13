use std::fs;
use std::path;
use std::io;

pub fn get_files(path: &String) -> Vec<String> {
    let mut files: Vec<String> = Vec::<String>::new();
    fs::read_dir(path).unwrap().into_iter().for_each(|file: Result<fs::DirEntry, io::Error>| {
        let path: path::PathBuf = file.unwrap().path();
        let full_path: String = path.display().to_string();
        let metadata: fs::Metadata = fs::metadata(&full_path).unwrap();

        let link: Result<path::PathBuf, io::Error> = fs::read_link(&full_path);
        if !link.is_ok() && metadata.is_dir() {
            let mut sub_files: Vec<String> = get_files(&full_path);
            files.append(&mut sub_files);
        }
        if metadata.is_file() {
            files.push(full_path);
        }
    });
    
    files
}
