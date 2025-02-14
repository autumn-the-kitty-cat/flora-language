#![allow(unused)]

use std::path::Path;

mod commands;
mod interpreter;
mod util;

fn main() {
    let settings = commands::flags::parse_flags();
    let input_file_name = settings.input_file.unwrap();
    interpreter::tokenize::tokenize_file(input_file_name);
}
