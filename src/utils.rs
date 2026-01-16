use colored::Colorize;
use std::{fs, io::ErrorKind, process};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: u32,
    pub start_column: u32,
    pub end_column: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn unspan(self) -> T {
        self.node
    }
}

pub fn read_file(path: String) -> String {
    match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(err) => {
            let message = match err.kind() {
                ErrorKind::NotFound => {
                    format!("File '{}' not found", path)
                }
                ErrorKind::PermissionDenied => {
                    format!("Permission denied: Cannot read file '{}'", path)
                }
                ErrorKind::IsADirectory => {
                    format!("'{}' is a directory, not a file", path)
                }
                ErrorKind::InvalidData => {
                    format!("File '{}' contains invalid UTF-8", path)
                }
                _ => {
                    format!("Failed to read file '{}': {}", path, err.kind())
                }
            };
            throw_error(&message, 1)
        }
    }
}

pub fn throw_error(message: &str, code: i32) -> ! {
    eprintln!("{}: {}", "Error".bright_red(), message);
    process::exit(code);
}

pub fn print_success(msg: &str) {
    println!("{} {}", "✓".bright_green().bold(), msg);
}

pub fn print_info(msg: &str) {
    println!("{} {}", "→".bright_blue().bold(), msg);
}

pub fn print_warning(msg: &str) {
    println!("{} {}", "⚠".bright_yellow().bold(), msg);
}
