use std::fs::read_to_string;
use std::io;
use std::io::Write;

use crate::scanner::scanner::Scanner;

pub fn run_file(path: &str) -> io::Result<()> {
    let source = read_to_string(path)?;
    run(source);

    Ok(())
}

pub fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();

    loop {
        let mut line = String::new();

        print!("> ");
        io::stdout().flush()?;
        // EOF if bytes = 0
        let bytes = stdin.read_line(&mut line)?;

        line = line.trim().to_string();

        if bytes == 0 || line == "exit" {
            break;
        }

        run(line);
    }

    Ok(())
}

pub fn run(source: String) {
    let mut sc = Scanner::new(source);
    for token in sc.scan_tokens() {
        println!("{}", token);
    }
}

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
}
