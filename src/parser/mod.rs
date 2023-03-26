use crate::scanner::{Token, TokenType};
use std::{
    fmt::{Display, Error},
    rc::Rc,
};

mod expr;
pub use expr::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug, Clone)]
struct ParseError {
    message: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse Error: {}", self.message)
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn binary<F>(&mut self, match_expr: F, token_types: &[TokenType]) -> Expr
    where
        F: Fn(&mut Self) -> Expr,
    {
        let mut expr = match_expr(self);

        while self.is_match(token_types) {
            // let operator = { self.previous() };
            let operator = self.tokens.get(self.current - 1).unwrap();
            let right = match_expr(self);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }
        expr
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        self.binary(
            Self::comparison,
            &[TokenType::BangEqual, TokenType::EqualEqual],
        )
    }

    fn comparison(&mut self) -> Expr {
        self.binary(
            Self::term,
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
        )
    }

    fn term(&mut self) -> Expr {
        self.binary(Self::factor, &[TokenType::Minus, TokenType::Plus])
    }

    fn factor(&mut self) -> Expr {
        self.binary(Self::unary, &[TokenType::Slash, TokenType::Star])
    }

    fn unary(&mut self) -> Expr {
        while self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary {
                operator: operator.clone(),
                right: Box::new(right),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.is_match(&[TokenType::False]) {
            Expr::Literal {
                value: Some(Rc::new(false)),
            }
        } else if self.is_match(&[TokenType::True]) {
            Expr::Literal {
                value: Some(Rc::new(true)),
            }
        } else if self.is_match(&[TokenType::Nil]) {
            Expr::Literal { value: None }
        } else if self.is_match(&[TokenType::Number, TokenType::String]) {
            Expr::Literal {
                value: self.previous().literal.clone(),
            }
        } else if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            Expr::Grouping {
                expression: Box::new(expr),
            }
        } else {
            panic!("Invalid syntax")
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if !self.check(token_type) {
            return Err(ParseError {
                message: message.to_string(),
            });
        }

        Ok(self.advance())
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(*t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if let Some(t) = self.peek() {
            return matches!(t.token_type, token_type);
        }
        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(t) => matches!(t.token_type, TokenType::Eof),
            None => true,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&mut self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn error(&self, token: &Token, message: &str) {}
}
