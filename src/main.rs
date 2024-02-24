use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process::exit;

mod lexer;
mod log;
mod parser;

use log::log_message::print_code_error;
use log::log_message::print_error_msg;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => execute_file(&args[1]),
        1 => execute_prompt(),
        _ => print_error_msg("Usage: amm [File]"),
    }
}

fn execute_file(file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(contents) => execute(contents),
        Err(_) => print_error_msg("Unable to read the file"),
    }
}

fn execute_prompt() {
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) if (is_end(&line)) => exit(0),
            Ok(_) => execute(line),
            Err(_) => print_error_msg("Unable to read the line"),
        }
    }
}

fn execute(contents: String) {
    let mut scanner = lexer::scanner::Scanner::new(&contents);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn is_end(line: &str) -> bool {
    line.trim().is_empty()
}

fn error(line_number: u32, message: &str) {
    print_code_error(line_number, message);
}
