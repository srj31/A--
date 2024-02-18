use std::env;

mod log;

use log::log_message::print_error_msg;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => execute_file(&args[1]),
        _ => print_error_msg("Usage: amm [File]"),
    }
}

fn execute_file(file: &str) {
    todo!()
}
