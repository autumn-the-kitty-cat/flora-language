#![allow(unused)]

use std::path::Path;

mod commands;
mod interpreter;
mod util;

fn main() {
    // Get compilation settings from the user command. This includes things like input and output
    // file destinations.
    let settings = commands::flags::parse_flags();

    // Get the simple tokens from the file set by the user
    let input_file_name = settings.input_file.unwrap();
    let simple_tokens = interpreter::simple_tokenize::tokenize_file(input_file_name);
    println!("Simple Tokens: [");
    for token in &simple_tokens {
        println!("\t{:?}", token);
    }
    println!("]");
    let advanced_tokens = interpreter::advanced_tokenize::advance_tokens(simple_tokens);
    println!("Advanced Tokens: [");
    for token in advanced_tokens {
        println!("\t{:?}", token);
    }
    println!("]");
}
