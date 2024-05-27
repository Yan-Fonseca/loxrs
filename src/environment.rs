use std::collections::HashMap;

use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, Option<Expr>>
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn set_enclosing_environment(&mut self, environment: Environment) {
        let reference = Box::new(environment);
        self.enclosing = Some(reference);
    }

    fn get_enclosing(&self) -> Option<Box<Environment>> {
        self.enclosing.clone()
    }

    pub fn define(&mut self, name: String, value: Option<Expr>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Expr, String> {
        let value = self.values.get(&name.get_lexeme());

        let enclosing_value = self.get_enclosing();

        match enclosing_value {
            Some(reference) => {
                let enclosing = *reference;
                let val = enclosing.get(name.clone());
                
                if let Ok(result_value) = val {
                    return Ok(result_value);
                }
            },
            None => {},
        }

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