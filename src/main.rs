mod directory;
mod table;
mod codemetadata;
mod command;

use std::collections;
use std::time;

use command::Command;
use codemetadata::{FileMetadata, SingleFileMetadata, CodeMetadata};
use table::Table;

fn main() {
    let command: Command = Command::parse().unwrap();
    let mut start: time::Instant = time::Instant::now();
    if command.action == "analyze" {
        println!("[] getting files");
        let files: Vec<String> = directory::get_files(&command.arguments["path"]);
        println!("took {:?}\n[] computing metadata for {} files", start.elapsed(), files.len());
        start = time::Instant::now();

        let lines: collections::HashMap<String, FileMetadata> = CodeMetadata::get_metadatas(&files);
        println!("took {:?}\n[] parsing result", start.elapsed());
        start = time::Instant::now();

        let mut rows: Vec<collections::HashMap<String, String>> = vec![];
        lines.keys().into_iter().for_each(|extention: &String| {
            let mut row: collections::HashMap<String, String> = collections::HashMap::<String, String>::new();
            row.insert(String::from("Extention"), extention.to_string());
            row.insert(String::from("Files"), format!("{}", lines[extention].files));
            row.insert(String::from("Lines"), format!("{}", lines[extention].lines));
            row.insert(String::from("LPF"), format!("{}", lines[extention].lines / lines[extention].files));
            row.insert(String::from("Blank"), format!("{}", lines[extention].blank_lines));
            row.insert(String::from("Size"), CodeMetadata::format_bytes(lines[extention].bytes));
            row.insert(String::from("Longest"), format!("{}", lines[extention].longest_file));
            row.insert(String::from("Modified"), CodeMetadata::format_date(lines[extention].modified));
            rows.push(row);
        });
        let headers: Vec<String> = vec![
            String::from("Extention"),
            String::from("Files"),
            String::from("Lines"),
            String::from("LPF"),
            String::from("Blank"),
            String::from("Size"),
            String::from("Longest"),
            String::from("Modified")
        ];
    
        println!("took {:?}\n[] sorting result", start.elapsed());
        start = time::Instant::now();
        rows.sort_by_key(|row: &collections::HashMap<String, String>| row["Files"].parse::<usize>().unwrap());
    
        println!("took {:?}\n[] printing table", start.elapsed());
        let table: Table = Table::new();
        table.print_with_headers(&rows, &headers);
    } else if command.action == "show" {
        let files: Vec<String> = directory::get_files(&command.arguments["path"]);
        let text: String = format!(".{}", &command.arguments["extention"]);
        let files_by_extention: Vec<String> = files.into_iter().filter(|file: &String| {
            let metadata: SingleFileMetadata = CodeMetadata::get_metadata(file);
            file.ends_with(&text) && metadata.extention != ""
        }).collect::<Vec<String>>();
        files_by_extention.iter().for_each(|f| println!("{}", f));
    } else {
        println!("invalid action {}", command.action);
    }
}
