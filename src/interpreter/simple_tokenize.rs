use super::tokens::SimpleTokens;
use crate::util::error;
use std::{fs, path::Path};

pub fn tokenize_file(file_path_string: String) -> Vec<SimpleTokens> {
    let mut file_path = Path::new(&file_path_string);
    let contents = fs::read_to_string(file_path);
    if contents.is_err() {
        error("Input file does not exist!".to_string());
    }
    let characters = contents
        .unwrap()
        .lines()
        .flat_map(|line| (line.to_string() + "\n").chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();

    let mut token_start_index = 0;
    let mut token_end_index = 0;
    let mut char_index = 0;
    let mut tokens = vec![];
    while token_end_index < characters.len() {
        if characters[token_end_index] == '/' && characters[token_end_index + 1] == '/' {
            while characters[token_end_index] != '\n' {
                token_end_index += 1;
            }
            token_start_index = token_end_index;
        }
        if characters[token_end_index].is_alphabetic() || characters[token_end_index] == '_' {
            while characters[token_end_index].is_alphanumeric()
                || characters[token_end_index] == '_'
            {
                token_end_index += 1;
            }
            let token = characters[token_start_index..token_end_index]
                .iter()
                .collect::<String>();
            tokens.push(SimpleTokens::Identifier(token));
            token_start_index = token_end_index;
        } else if characters[token_end_index].is_numeric() {
            while characters[token_end_index].is_numeric()
                || characters[token_end_index] == '.'
                || characters[token_end_index] == '_'
            {
                token_end_index += 1;
            }

            let token = characters[token_start_index..token_end_index]
                .iter()
                .collect::<String>();
            tokens.push(SimpleTokens::Number(token));
            token_start_index = token_end_index;
        } else if characters[token_end_index].is_ascii_punctuation() {
            while characters[token_end_index].is_ascii_punctuation() {
                token_end_index += 1;
            }

            let token = characters[token_start_index..token_end_index]
                .iter()
                .collect::<String>();
            tokens.push(SimpleTokens::Operator(token));
            token_start_index = token_end_index;
        } else {
            token_start_index += 1;
            token_end_index += 1;
        }
    }

    tokens
}
