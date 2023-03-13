mod directory;
mod table;
mod codemetadata;

use std::env;
use std::collections;

use codemetadata::FileMetadata;
use codemetadata::CodeMetadata;
use table::Table;

#[derive(Debug)]
struct Cli {
    path: String
}

fn main() {
    let path: String = env::args().nth(1).expect("no path given");
    let cli: Cli = Cli{ path };

    let files: Vec<String> = directory::get_files(&cli.path);
    let lines: collections::HashMap<String, FileMetadata> = CodeMetadata::get_metadatas(&files);
    let mut rows: Vec<collections::HashMap<String, String>> = vec![];
    lines.keys().into_iter().for_each(|extention: &String| {
        let mut row: collections::HashMap<String, String> = collections::HashMap::<String, String>::new();
        row.insert(String::from("Extention"), extention.to_string());
        row.insert(String::from("Files"), format!("{}", lines[extention].files));
        row.insert(String::from("Lines"), format!("{}", lines[extention].lines));
        row.insert(String::from("LPF"), format!("{}", lines[extention].lines / lines[extention].files));
        rows.push(row);
    });
    let headers: Vec<String> = vec![
        String::from("Extention"),
        String::from("Files"),
        String::from("Lines"),
        String::from("LPF")
    ];

    let table: Table = Table::new();
    table.print_with_headers(&rows, &headers);
}
