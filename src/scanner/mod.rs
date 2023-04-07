use phf::phf_map;
use std::{fmt::Display, rc::Rc};

mod token;

pub use self::token::{Token, TokenType};
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

struct ScanError {
    message: String,
}

impl Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scan Error: {}", self.message)
    }
}

pub struct Scanner {
    source: String,
    source_chars: Vec<char>,
    tokens: Vec<Token>,
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
            match self.scan_token() {
                Ok(Some(token)) => self.tokens.push(token),
                Ok(None) => (),
                Err(e) => {
                    lox::error(self.line, &e.message);
                }
            }
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line));

        &self.tokens
    }

    fn scan_token(&mut self) -> Result<Option<Token>, ScanError> {
        if let Some(c) = self.advance() {
            match c {
                '(' => Ok(Some(self.make_empty_token(TokenType::LeftParen))),
                ')' => Ok(Some(self.make_empty_token(TokenType::RightParen))),
                '{' => Ok(Some(self.make_empty_token(TokenType::LeftBrace))),
                '}' => Ok(Some(self.make_empty_token(TokenType::RightBrace))),
                ',' => Ok(Some(self.make_empty_token(TokenType::Comma))),
                '.' => Ok(Some(self.make_empty_token(TokenType::Dot))),
                '-' => Ok(Some(self.make_empty_token(TokenType::Minus))),
                '+' => Ok(Some(self.make_empty_token(TokenType::Plus))),
                ';' => Ok(Some(self.make_empty_token(TokenType::Semicolon))),
                '*' => Ok(Some(self.make_empty_token(TokenType::Star))),
                '!' => {
                    let token = if self.is_match('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    Ok(Some(self.make_empty_token(token)))
                }
                '=' => {
                    let token = if self.is_match('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    Ok(Some(self.make_empty_token(token)))
                }
                '<' => {
                    let token = if self.is_match('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    Ok(Some(self.make_empty_token(token)))
                }
                '>' => {
                    let token = if self.is_match('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    Ok(Some(self.make_empty_token(token)))
                }
                '/' => {
                    if self.is_match('/') {
                        while *self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                        Ok(None)
                    } else {
                        Ok(Some(self.make_empty_token(TokenType::Slash)))
                    }
                }
                ' ' | '\r' | '\t' => Ok(None),
                '\n' => {
                    self.line += 1;
                    Ok(None)
                }
                '"' => Ok(Some(self.parse_string()?)),
                _ => {
                    if c.is_numeric() {
                        Ok(Some(self.parse_number()))
                    } else if c.is_alphabetic() {
                        Ok(Some(self.parse_identifier()))
                    } else {
                        Err(ScanError {
                            message: format!("Unexpected character: {}", c),
                        })
                    }
                }
            }
        } else {
            Ok(None)
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

    // Checks if the next character is the expected one and consumes it if it is.
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

    fn parse_string(&mut self) -> Result<Token, ScanError> {
        while *self.peek() != '"' && !self.is_at_end() {
            if *self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        // Consume closing '"'
        self.advance();

        if let Some(value) = self.source.get(self.start + 1..self.current - 1) {
            Ok(self.make_token(TokenType::String, Some(Rc::new(value.to_string()))))
        } else {
            Err(ScanError {
                message: "Unterminated string.".to_string(),
            })
        }
    }

    fn parse_number(&mut self) -> Token {
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
        self.make_token(TokenType::Number, Some(Rc::new(s.parse::<f64>().unwrap())))
    }

    fn parse_identifier(&mut self) -> Token {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = self.source.get(self.start..self.current).unwrap();
        let token_type = match KEYWORDS.get(text) {
            Some(t) => t,
            None => &TokenType::Identifier,
        };

        self.make_empty_token(*token_type)
    }

    fn make_empty_token(&mut self, token_type: TokenType) -> Token {
        self.make_token(token_type, None)
    }

    fn make_token(&mut self, token_type: TokenType, literal: Option<Rc<dyn Display>>) -> Token {
        let text = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .to_string();

        Token::new(token_type, text, literal, self.line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_tokens() {
        let mut scanner = Scanner::new("-123 * 45.67".to_string());
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens[0],
            Token::new(TokenType::Minus, "-".to_string(), None, 1)
        );
        assert_eq!(
            tokens[1],
            Token::new(
                TokenType::Number,
                "123".to_string(),
                Some(Rc::new(123.0)),
                1
            )
        );
        assert_eq!(
            tokens[2],
            Token::new(TokenType::Star, "*".to_string(), None, 1)
        );
        assert_eq!(
            tokens[3],
            Token::new(
                TokenType::Number,
                "45.67".to_string(),
                Some(Rc::new(45.67)),
                1
            )
        );
        assert_eq!(
            tokens[4],
            Token::new(TokenType::Eof, "".to_string(), None, 1)
        );
    }

    #[test]
    fn test_scan_string() {
        let mut scanner = Scanner::new("\"hello\"".to_string());
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens[0],
            Token::new(
                TokenType::String,
                "\"hello\"".to_string(),
                Some(Rc::new("hello".to_string())),
                1
            )
        );
        assert_eq!(
            tokens[1],
            Token::new(TokenType::Eof, "".to_string(), None, 1)
        );
    }

    // Test for scanning with comments
    #[test]
    fn test_scan_tokens_with_comments() {
        let mut scanner = Scanner::new("1 + 2 // 3 + 4".to_string());
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens[0],
            Token::new(TokenType::Number, "1".to_string(), Some(Rc::new(1.0)), 1)
        );
        assert_eq!(
            tokens[1],
            Token::new(TokenType::Plus, "+".to_string(), None, 1)
        );
        assert_eq!(
            tokens[2],
            Token::new(TokenType::Number, "2".to_string(), Some(Rc::new(2.0)), 1)
        );
        assert_eq!(
            tokens[3],
            Token::new(TokenType::Eof, "".to_string(), None, 1)
        );
    }
}
