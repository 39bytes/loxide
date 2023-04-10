use std::{any::Any, rc::Rc};

use crate::parser::{Expr, RuntimeError};

pub fn interpret(expr: Expr) -> Result<(), RuntimeError> {
    let val = expr.interpret()?;
    println!("{}", stringify(val));
    Ok(())
}

fn stringify(val: Option<Rc<dyn Any>>) -> String {
    match val {
        Some(val) => {
            if let Some(val) = val.downcast_ref::<f64>() {
                let text = val.to_string();
                return match text.strip_suffix(".0") {
                    Some(t) => t.to_string(),
                    None => text,
                };
            }

            if let Some(val) = val.downcast_ref::<bool>() {
                return val.to_string();
            }

            if let Some(val) = val.downcast_ref::<String>() {
                return val.to_string();
            }

            "Object does not have string representation".to_string()
        }
        None => "nil".to_string(),
    }
}
