use phf::phf_map;
use std::{fmt::Display, iter::Peekable, str::Chars};

pub mod token;

use self::token::{Token, TokenType};
use crate::lox;

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    source_chars: Vec<char>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.clone(),
            tokens: Vec::new(),
            source_chars: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            Box::new(""),
            self.line,
        ));

        &self.tokens
    }

    fn scan_token(&mut self) {
        if let Some(c) = self.advance() {
            match c {
                '(' => self.add_empty_token(TokenType::LeftParen),
                ')' => self.add_empty_token(TokenType::RightParen),
                '{' => self.add_empty_token(TokenType::LeftBrace),
                '}' => self.add_empty_token(TokenType::RightBrace),
                ',' => self.add_empty_token(TokenType::Comma),
                '.' => self.add_empty_token(TokenType::Dot),
                '-' => self.add_empty_token(TokenType::Minus),
                '+' => self.add_empty_token(TokenType::Plus),
                ';' => self.add_empty_token(TokenType::Semicolon),
                '*' => self.add_empty_token(TokenType::Star),
                '!' => {
                    let token = if self.is_match('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_empty_token(token)
                }
                '=' => {
                    let token = if self.is_match('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_empty_token(token)
                }
                '<' => {
                    let token = if self.is_match('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    self.add_empty_token(token);
                }
                '>' => {
                    let token = if self.is_match('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    self.add_empty_token(token)
                }
                '/' => {
                    if self.is_match('/') {
                        while *self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_empty_token(TokenType::Slash);
                    }
                }
                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,
                '"' => self.parse_string(),
                _ => {
                    if c.is_numeric() {
                        self.parse_number()
                    } else if c.is_alphabetic() {
                        self.parse_identifier()
                    } else {
                        lox::error(self.line, "Unexpected character.")
                    }
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source_chars.len()
    }

    fn advance(&mut self) -> Option<&char> {
        let c = self.source_chars.get(self.current);
        self.current += 1;
        c
    }

    fn is_match(&mut self, expected: char) -> bool {
        if let Some(c) = self.source_chars.get(self.current) {
            if *c != expected {
                return false;
            }
            self.current += 1;
            return true;
        }
        false
    }

    fn peek(&self) -> &char {
        self.source_chars.get(self.current).unwrap_or(&'\0')
    }

    fn peek_next(&self) -> &char {
        self.source_chars.get(self.current + 1).unwrap_or(&'\0')
    }

    fn parse_string(&mut self) {
        while *self.peek() != '"' && !self.is_at_end() {
            if *self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            lox::error(self.line, "Unterminated string.");
            return;
        }

        // Consume closing '"'
        self.advance();

        let value = self
            .source
            .get(self.start + 1..self.current - 1)
            .unwrap()
            .to_string();

        self.add_token(TokenType::String, Box::new(value));
    }

    fn parse_number(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }

        // Look for fractional part
        if *self.peek() == '.' && self.peek_next().is_numeric() {
            // Consume '.'
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        let s = self.source.get(self.start..self.current).unwrap();
        self.add_token(TokenType::Number, Box::new(s.parse::<f64>().unwrap()))
    }

    fn parse_identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text = self.source.get(self.start..self.current).unwrap();
        let token_type = match KEYWORDS.get(text) {
            Some(t) => t,
            None => &TokenType::Identifier,
        };
        self.add_empty_token(*token_type);
    }

    fn add_empty_token(&mut self, token_type: TokenType) {
        self.add_token(token_type, Box::new(""));
    }

    fn add_token(&mut self, token_type: TokenType, literal: Box<dyn Display>) {
        let text = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .to_string();

        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }
}
