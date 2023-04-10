use std::{any::Any, fmt::Display, rc::Rc};

use crate::scanner::{Token, TokenType};

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
        value: Option<Rc<dyn Any>>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn interpret(self) -> Result<Option<Rc<dyn Any>>, RuntimeError> {
        match self {
            Expr::Literal { value } => Ok(value),
            Expr::Grouping { expression } => expression.interpret(),
            Expr::Unary { operator, right } => match operator.token_type {
                TokenType::Bang => {
                    let val = right.interpret()?;
                    Ok(Some(Rc::new(!Expr::is_truthy(val))))
                }
                TokenType::Minus => {
                    let val =
                        (*right).try_convert::<f64>(&operator, "Operand must be a number.")?;
                    Ok(Some(Rc::new(-val)))
                }
                _ => Err(RuntimeError {
                    message: "Invalid unary operator.".to_string(),
                    token: operator,
                }),
            },
            Expr::Binary {
                left,
                operator,
                right,
            } => match operator.token_type {
                TokenType::Minus
                | TokenType::Slash
                | TokenType::Star
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    let (l, r) = (
                        (*left).try_convert::<f64>(&operator, "Operands must be numbers.")?,
                        (*right).try_convert::<f64>(&operator, "Operands must be numbers.")?,
                    );

                    match operator.token_type {
                        TokenType::Minus => Ok(Some(Rc::new(l - r))),
                        TokenType::Slash => Ok(Some(Rc::new(l / r))),
                        TokenType::Star => Ok(Some(Rc::new(l * r))),
                        TokenType::Greater => Ok(Some(Rc::new(l > r))),
                        TokenType::GreaterEqual => Ok(Some(Rc::new(l >= r))),
                        TokenType::Less => Ok(Some(Rc::new(l < r))),
                        TokenType::LessEqual => Ok(Some(Rc::new(l <= r))),
                        _ => unreachable!(),
                    }
                }
                TokenType::Plus => {
                    let err = RuntimeError {
                        message: "Operands must be two numbers or two strings.".to_string(),
                        token: operator,
                    };
                    let left = left.interpret()?.ok_or(err.clone())?;
                    let right = right.interpret()?.ok_or(err.clone())?;

                    if let (Some(l), Some(r)) =
                        (left.downcast_ref::<f64>(), right.downcast_ref::<f64>())
                    {
                        return Ok(Some(Rc::new(l + r)));
                    }

                    if let (Some(l), Some(r)) = (
                        left.downcast_ref::<String>(),
                        right.downcast_ref::<String>(),
                    ) {
                        return Ok(Some(Rc::new(l.clone() + r)));
                    }

                    Err(err)
                }
                TokenType::EqualEqual => {
                    let left = left.interpret()?;
                    let right = right.interpret()?;
                    Ok(Some(Rc::new(Expr::equals(left, right))))
                }
                TokenType::BangEqual => {
                    let left = left.interpret()?;
                    let right = right.interpret()?;
                    Ok(Some(Rc::new(!Expr::equals(left, right))))
                }
                _ => Err(RuntimeError {
                    message: "Invalid binary operator.".to_string(),
                    token: operator,
                }),
            },
        }
    }

    fn is_truthy(obj: Option<Rc<dyn Any>>) -> bool {
        match obj {
            Some(v) => match v.downcast_ref::<bool>() {
                Some(val) => *val,
                None => true,
            },
            None => false,
        }
    }

    fn equals(a: Option<Rc<dyn Any>>, b: Option<Rc<dyn Any>>) -> bool {
        if a.is_none() && b.is_none() {
            return true;
        }
        if a.is_none() {
            return false;
        }

        let a = a.unwrap();
        let b = b.unwrap();

        if let (Some(a), Some(b)) = (a.downcast_ref::<f64>(), b.downcast_ref::<f64>()) {
            return a == b;
        }
        if let (Some(a), Some(b)) = (a.downcast_ref::<bool>(), b.downcast_ref::<bool>()) {
            return a == b;
        }
        if let (Some(a), Some(b)) = (a.downcast_ref::<String>(), b.downcast_ref::<String>()) {
            return a == b;
        }

        false
    }

    fn try_convert<T>(self, token: &Token, message: &str) -> Result<T, RuntimeError>
    where
        T: 'static + Copy,
    {
        let val = self.interpret()?;
        match val {
            Some(v) => match v.downcast_ref::<T>() {
                Some(val) => Ok(*val),
                None => Err(RuntimeError {
                    message: message.to_string(),
                    token: token.clone(),
                }),
            },
            None => Err(RuntimeError {
                message: message.to_string(),
                token: token.clone(),
            }),
        }
    }
}

#[derive(Clone)]
pub struct RuntimeError {
    token: Token,
    message: String,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} \n[line {}]", self.message, self.token.line)
    }
}

// impl Display for Expr {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let text = match self {
//             Expr::Binary {
//                 left,
//                 operator,
//                 right,
//             } => parenthesize!(operator.lexeme, left, right),
//             Expr::Grouping { expression } => parenthesize!("group", expression),
//             Expr::Literal { value } => match value {
//                 Some(v) => v.to_string(),
//                 None => String::from("null"),
//             },
//             Expr::Unary { operator, right } => parenthesize!(operator.lexeme, right),
//         };
//         write!(f, "{text}")
//     }
// }
