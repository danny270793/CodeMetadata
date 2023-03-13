use std::path;
use std::fs;
use std::io;
use std::io::BufRead;
use std::str;
use std::collections;

pub struct CodeMetadata {}

#[derive(Debug)]
pub struct FileMetadata {
    pub files: usize,
    pub lines: usize
}

impl CodeMetadata {
    pub fn get_metadatas(files: &Vec<String>) -> collections::HashMap<String, FileMetadata> {
        let mut hash_map: collections::HashMap<String, FileMetadata> = collections::HashMap::<String, FileMetadata>::new();
        files.iter().for_each(|file: &String| {
            let metadata: (String, usize)  = CodeMetadata::get_metadata(file);
            if hash_map.contains_key(&metadata.0) {
                let rows: Option<&FileMetadata> = hash_map.get(&metadata.0);
                match rows {
                    Some(value) => {
                        let file_metadata: FileMetadata = FileMetadata {
                            files: value.files + 1,
                            lines: value.lines + metadata.1
                        };
                        hash_map.insert(metadata.0, file_metadata);
                    },
                    None => {},
                }
            } else {
                let file_metadata: FileMetadata = FileMetadata {
                    files: 1,
                    lines: metadata.1
                };
                hash_map.insert(metadata.0, file_metadata);
            }
        });
        hash_map
    }

    pub fn get_metadata(file: &String) -> (String, usize) {
        let path_parts: Vec<&str> = file.split(path::MAIN_SEPARATOR.to_string().as_str()).collect();
        let filename: &str = path_parts[path_parts.len() - 1];

        let file_parts: Vec<&str> = filename.split(".").collect();
        let input: fs::File = fs::File::open(file.as_str()).expect("can't open file");
        let buffered: io::BufReader<fs::File> = io::BufReader::new(input);
        let line_count: usize = buffered.lines().count();
    
        if !filename.contains(".") {
            return (String::from(""), line_count);
        }
    
        return (String::from(file_parts[file_parts.len() - 1]), line_count);
    }
}
