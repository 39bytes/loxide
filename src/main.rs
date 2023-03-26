use lox::*;
use parser::Expr;
use scanner::{Token, TokenType};
use std::env;
use std::process::exit;
use std::rc::Rc;

use parser::Parser;
use scanner::Scanner;

mod lox;
mod parser;
mod scanner;

fn main() {
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus, String::from("-"), None, 1),
            right: Box::new(Expr::Literal {
                value: Some(Rc::new(123)),
            }),
        }),
        operator: Token::new(TokenType::Star, String::from("*"), None, 1),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Some(Rc::new(45.67)),
            }),
        }),
    };

    println!("{}", expression);
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
