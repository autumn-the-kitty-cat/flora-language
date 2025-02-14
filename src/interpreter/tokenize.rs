use std::{fs, path::Path};

enum SimpleTokens {
    Operator(String),
    Identifier(String),
    Number(String),
}

pub fn tokenize_file(file_path_string: String) {
    let mut file_path = Path::new(&file_path_string);
    let contents = fs::read_to_string(file_path).unwrap();
    let lines = contents
        .lines()
        .flat_map(|line| (line.to_string() + "\n").chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();
    println!("{:#?}",lines);
}
