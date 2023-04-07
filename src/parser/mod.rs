use crate::{
    lox,
    scanner::{Token, TokenType},
};
use std::{
    fmt::{Display, Error},
    mem::discriminant,
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

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression().ok()
    }

    fn binary<F>(&mut self, match_expr: F, token_types: &[TokenType]) -> Result<Expr, ParseError>
    where
        F: Fn(&mut Self) -> Result<Expr, ParseError>,
    {
        let mut expr = match_expr(self)?;

        while self.is_match(token_types) {
            let operator = self.previous().clone();
            let right = match_expr(self)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        self.binary(
            Self::comparison,
            &[TokenType::BangEqual, TokenType::EqualEqual],
        )
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
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

    fn term(&mut self) -> Result<Expr, ParseError> {
        self.binary(Self::factor, &[TokenType::Minus, TokenType::Plus])
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        self.binary(Self::unary, &[TokenType::Slash, TokenType::Star])
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.is_match(&[TokenType::False]) {
            Ok(Expr::Literal {
                value: Some(Rc::new(false)),
            })
        } else if self.is_match(&[TokenType::True]) {
            Ok(Expr::Literal {
                value: Some(Rc::new(true)),
            })
        } else if self.is_match(&[TokenType::Nil]) {
            Ok(Expr::Literal { value: None })
        } else if self.is_match(&[TokenType::Number, TokenType::String]) {
            Ok(Expr::Literal {
                value: self.previous().literal.clone(),
            })
        } else if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;

            Ok(Expr::Grouping {
                expression: Box::new(expr),
            })
        } else {
            Err(self.error(self.peek().unwrap(), "Expect expression."))
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if !self.check(token_type) {
            return Err(self.error(self.peek().unwrap(), message));
        }

        Ok(self.advance())
    }

    fn error(&self, token: &Token, message: &str) -> ParseError {
        lox::error(token.line, message);
        ParseError {
            message: message.to_string(),
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if let TokenType::Semicolon = self.previous().token_type {
                return;
            }

            match self.peek().unwrap().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => self.advance(),
            };
        }
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if let Some(t) = self.peek() {
            return discriminant(&t.token_type) == discriminant(&token_type);
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

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}
