use std::collections;

pub struct Table {
    border: String,
    ceil: String,
    line: String
}

impl Table {
    pub fn new() -> Table {
        Table {
            border: String::from("+"),
            ceil: String::from("-"),
            line: String::from("|"),
        }
    }
    pub fn print_with_headers(&self, rows: &Vec<collections::HashMap<String, String>>, headers: &Vec<String>) {
        let mut sizes: collections::HashMap<String, usize> = collections::HashMap::new();
        rows.iter().for_each(|row: &collections::HashMap<String, String>| {
            for (key, value) in row.iter() {
                let last_value: Option<&usize> = sizes.get(key);
                match last_value {
                    Some(last_value) => {
                        if value.len() > *last_value {
                            sizes.insert(key.to_string(), value.len());
                        }
                    }
                    None => {
                        if key.to_string().len() > value.len() {
                            sizes.insert(key.to_string(), key.to_string().len());
                        } else {
                            sizes.insert(key.to_string(), value.len());
                        }
                    }
                }
            }
        });

        let mut headers_map: collections::HashMap<String, String> = collections::HashMap::new();
        headers.iter().for_each(|header: &String| {
            headers_map.insert(header.to_string(), header.to_string());
        });
        self.print_separator(&headers, &sizes);
        self.print_line(&headers, &sizes, &headers_map);
        rows.iter().for_each(|row: &collections::HashMap<String, String>| {
            self.print_separator(&headers, &sizes);
            self.print_line(&headers, &sizes, row);
        });
        self.print_separator(&headers, &sizes);
    }

    pub fn print(&self, rows: &Vec<collections::HashMap<String, String>>) {
        let headers: Vec<String> = self.get_headers_from(&rows);
        self.print_with_headers(&rows, &headers);
    }

    fn get_headers_from(&self, rows: &Vec<collections::HashMap<String, String>>) -> Vec<String> {
        let mut headers: Vec<String> = Vec::new();
        rows.iter().for_each(|row: &collections::HashMap<String, String>| {
            for (key, _) in row.iter() {
                if !headers.contains(key) {
                    headers.push(key.to_string());
                }
            }
        });
        headers
    }

    fn print_line(&self, headers: &Vec<String>, sizes: &collections::HashMap<String, usize>, row: &collections::HashMap<String, String>) {
        print!("{}", self.line);
        headers.iter().for_each(|header| {
            let size: Option<&usize> = sizes.get(header);
            print!(" {} ", row[header]);
            match size {
                None => {}
                Some(size) => {
                    for _ in 0..(*size - row[header].len()) {
                        print!(" ");
                    }
                }
            }
            print!("{}", self.line);
        });
        println!("");
    }

    fn print_separator(&self, headers: &Vec<String>, sizes: &collections::HashMap<String, usize>) {
        print!("{}", self.border);
        headers.iter().for_each(|header| {
            let size: Option<&usize> = sizes.get(header);
            match size {
                None => {}
                Some(size) => {
                    for _ in 0..(*size + 2) {
                        print!("{}", self.ceil);
                    }
                }
            }
            print!("{}", self.border);
        });
        println!("");
    }
}
