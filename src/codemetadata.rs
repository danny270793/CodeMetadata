use std::path;
use std::fs;
use std::io;
use std::io::BufRead;
use std::str;
use std::time;
use std::collections;

pub struct CodeMetadata {}

#[derive(Debug)]
pub struct FileMetadata {
    pub files: usize,
    pub lines: usize,
    pub blank_lines: usize,
    pub bytes: u64,
    pub longest_file: usize,
    pub modified: time::SystemTime
}

// Metadata of a single file, extention is empty when the file must be ignored
pub struct SingleFileMetadata {
    pub extention: String,
    pub lines: usize,
    pub blank_lines: usize,
    pub bytes: u64,
    pub modified: time::SystemTime
}

impl CodeMetadata {
    pub fn get_metadatas(files: &Vec<String>) -> collections::HashMap<String, FileMetadata> {
        let mut hash_map: collections::HashMap<String, FileMetadata> = collections::HashMap::<String, FileMetadata>::new();
        files.iter().for_each(|file: &String| {
            let metadata: SingleFileMetadata = CodeMetadata::get_metadata(file);
            if metadata.extention == "" {
                return;
            }
            match hash_map.get_mut(&metadata.extention) {
                Some(value) => {
                    value.files += 1;
                    value.lines += metadata.lines;
                    value.blank_lines += metadata.blank_lines;
                    value.bytes += metadata.bytes;
                    value.longest_file = value.longest_file.max(metadata.lines);
                    value.modified = value.modified.max(metadata.modified);
                },
                None => {
                    hash_map.insert(metadata.extention, FileMetadata {
                        files: 1,
                        lines: metadata.lines,
                        blank_lines: metadata.blank_lines,
                        bytes: metadata.bytes,
                        longest_file: metadata.lines,
                        modified: metadata.modified
                    });
                },
            }
        });
        hash_map
    }

    pub fn get_metadata(file: &String) -> SingleFileMetadata {
        let path_parts: Vec<&str> = file.split(path::MAIN_SEPARATOR.to_string().as_str()).collect();
        let filename: &str = path_parts[path_parts.len() - 1];

        let file_parts: Vec<&str> = filename.split(".").collect();
        let input: fs::File = fs::File::open(file.as_str()).expect("can't open file");
        let file_info: Option<fs::Metadata> = input.metadata().ok();
        let buffered: io::BufReader<fs::File> = io::BufReader::new(input);
        let mut lines: usize = 0;
        let mut blank_lines: usize = 0;
        let mut is_utf8: bool = true;
        buffered.lines().for_each(|line: Result<String, io::Error>| {
            lines += 1;
            match line {
                Ok(line) => {
                    if line.trim().is_empty() {
                        blank_lines += 1;
                    }
                },
                Err(error) =>  {
                    if error.to_string() == "stream did not contain valid UTF-8" {
                        is_utf8 = false;
                    }
                },
            }
        });

        let bytes: u64 = file_info.as_ref().map(|info: &fs::Metadata| info.len()).unwrap_or(0);
        let modified: time::SystemTime = file_info.as_ref()
            .and_then(|info: &fs::Metadata| info.modified().ok())
            .unwrap_or(time::UNIX_EPOCH);

        let extention: String = if filename.contains(".") && is_utf8 {
            String::from(file_parts[file_parts.len() - 1])
        } else {
            String::new()
        };
        SingleFileMetadata { extention, lines, blank_lines, bytes, modified }
    }

    // 1536 -> "1.5 KB"
    pub fn format_bytes(bytes: u64) -> String {
        let units: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
        let mut value: f64 = bytes as f64;
        let mut unit: usize = 0;
        while value >= 1024.0 && unit < units.len() - 1 {
            value /= 1024.0;
            unit += 1;
        }
        if unit == 0 {
            format!("{} {}", bytes, units[unit])
        } else {
            format!("{:.1} {}", value, units[unit])
        }
    }

    // YYYY-MM-DD in UTC, no date library needed (civil from days algorithm)
    pub fn format_date(time: time::SystemTime) -> String {
        let days: i64 = time.duration_since(time::UNIX_EPOCH).map(|d: time::Duration| (d.as_secs() / 86400) as i64).unwrap_or(0);
        let z: i64 = days + 719468;
        let era: i64 = z.div_euclid(146097);
        let doe: i64 = z.rem_euclid(146097);
        let yoe: i64 = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let doy: i64 = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp: i64 = (5 * doy + 2) / 153;
        let day: i64 = doy - (153 * mp + 2) / 5 + 1;
        let month: i64 = if mp < 10 { mp + 3 } else { mp - 9 };
        let year: i64 = yoe + era * 400 + if month <= 2 { 1 } else { 0 };
        format!("{:04}-{:02}-{:02}", year, month, day)
    }
}
