use lox::*;
use std::env;
use std::process::exit;

mod interpreter;
mod lox;
mod parser;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt().expect("Shell error"),
        2 => run_file(&args[1]).expect("Error reading source file."),
        _ => {
            println!("Usage: loxide [script]");
            exit(64)
        }
    }
}
