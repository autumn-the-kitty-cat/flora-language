use crate::commands::compilation_settings::CompilationSettings;
use crate::util::{error, warning};

pub fn parse_flags() -> CompilationSettings {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    let mut compilation_settings = CompilationSettings::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                help_message();
                std::process::exit(0);
            }
            "-o" => {
                if i + 1 >= args.len() {
                    error("No output file specified after -o flag!".to_string());
                }

                if compilation_settings.output_file.is_none() {
                    let file_name = args[i + 1].clone();
                    if !file_name.chars().all(|x| x.is_alphanumeric() || x == '.') {
                        error("Output files must be alphanumeric!".to_string());
                    }

                    compilation_settings.output_file = Some(file_name);
                } else {
                    warning("The output file is already set!".to_string());
                }
                i += 1;
            }
            input_file_path => {
                if compilation_settings.input_file.is_none() {
                    if !input_file_path
                        .chars()
                        .all(|x| x.is_alphanumeric() || x == '.')
                    {
                        error("Output files must be alphanumeric!".to_string());
                    }

                    compilation_settings.input_file = Some(input_file_path.to_string());
                } else {
                    warning("The output file is already set!".to_string());
                }
                i += 1;
            }
        }
        i += 1;
    }

    if compilation_settings.input_file.is_none() {
        error("No input files specified!".to_string());
    }

    if compilation_settings.output_file.is_none() {
        error("No output file specified!".to_string());
    }

    compilation_settings
}

fn help_message() {
    println!(
        "
Flora Compiler
------------------------------
flora <command> [arguments...]

-h | --help   \t\t Print this message
-o [file_name]\t\t Set the output file
"
    );
}
