use colored::Colorize;

pub fn warning(text: String) {
    eprintln!("{} {}", "[WARNING]".yellow().bold(), text);
}

pub fn error(text: String) {
    eprintln!("{} {}", "[FATAL ERROR]".red().bold(), text);
    std::process::exit(255);
}
