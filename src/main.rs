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
use parser::interpreter::Interpreter;

struct Rox {
    interpreter: Interpreter,
}

impl Rox {
    fn new() -> Rox {
        Rox {
            interpreter: Interpreter::new(),
        }
    }

    fn run(&mut self) {
        let args: Vec<String> = env::args().collect();

        match args.len() {
            2 => self.execute_file(&args[1]),
            1 => self.execute_prompt(),
            _ => print_error_msg("Usage: amm [File]"),
        }
    }

    fn execute_file(&mut self, file_path: &str) {
        match fs::read_to_string(file_path) {
            Ok(contents) => self.execute(contents),
            Err(_) => print_error_msg("Unable to read the file"),
        }
    }

    fn execute_prompt(&mut self) {
        loop {
            print!("> ");
            let _ = io::stdout().flush();
            let mut line = String::new();
            match std::io::stdin().read_line(&mut line) {
                Ok(_) if (self.is_end(&line)) => exit(0),
                Ok(_) => self.execute(line),
                Err(_) => print_error_msg("Unable to read the line"),
            }
        }
    }

    fn execute(&mut self, contents: String) {
        let mut scanner = lexer::scanner::Scanner::new(&contents);
        let tokens = scanner.scan_tokens().clone();

        let mut parser = parser::parser::Parser::new(tokens);
        let stmts = parser.parse();

        self.interpreter.interpret(&stmts);
    }

    fn is_end(&self, line: &str) -> bool {
        line.trim().is_empty()
    }
}

fn main() {
    let mut rox = Rox::new();
    rox.run();
}

fn error(line_number: u32, message: &str) {
    print_code_error(line_number, message);
}
