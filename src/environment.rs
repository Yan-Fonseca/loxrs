use std::collections::HashMap;

use crate::expr::Expr;
use crate::token::Token;

pub struct Environment {
    pub values: HashMap<String, Option<Expr>>
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Option<Expr>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Expr, String> {
        let value = self.values.get(&name.get_lexeme());

        match value {
            Some(expr) => {
                match expr.clone() {
                    Some(expression_value) => {
                        Ok(expression_value)
                    },
                    None => Err(format!("[ERROR] Variable {} is not defined!", name.get_lexeme())),
                }
            },
            None => Err(format!("[ERROR] {} is not defined!", name.get_lexeme())),
        }
    }

    pub fn assign(&mut self, name: String, data: Option<Expr>) -> Result<(),String> {
        let value = self.values.get(&name);

        match value {
            Some(_) => {
                self.values.insert(name, data);
                return Ok(());
            },
            None => Err(format!("[ERROR] {} is not defined!", name)),
        }
    }
}