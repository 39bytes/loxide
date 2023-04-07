use std::fmt::Debug;
use std::mem::discriminant;
use std::{fmt::Display, rc::Rc};

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Rc<dyn Display>>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Rc<dyn Display>>,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = &self.literal {
            write!(f, "{} {} {}", self.token_type, self.lexeme, s)
        } else {
            write!(f, "{} {} ", self.token_type, self.lexeme)
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        discriminant(&self.token_type) == discriminant(&other.token_type)
            && self.lexeme == other.lexeme
    }
}

#[derive(Clone, Copy, Debug, strum_macros::Display)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
