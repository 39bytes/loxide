use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::scanner::Token;

macro_rules! parenthesize {
    ( $name:expr, $($e:expr), *) => {{
        let mut result = String::from("(");
        result.push_str(&$name.to_string());
        $(
            result.push(' ');
            result.push_str(&$e.to_string());
        )*
        result.push(')');
        result
    }};
}

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Option<Rc<dyn Display>>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => parenthesize!(operator.lexeme, left, right),
            Expr::Grouping { expression } => parenthesize!("group", expression),
            Expr::Literal { value } => match value {
                Some(v) => v.to_string(),
                None => String::from("null"),
            },
            Expr::Unary { operator, right } => parenthesize!(operator.lexeme, right),
        };
        write!(f, "{text}")
    }
}
