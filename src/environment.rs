use std::collections::HashMap;

use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, Option<Expr>>
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn set_enclosing_environment(&mut self, environment: Option<Environment>) {
        if let Some(env) = environment {
            let reference = Box::new(env);
            self.enclosing = Some(reference);
        }
        self.enclosing = None;
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

        match value {
            Some(expr) => {
                match expr.clone() {
                    Some(expression_value) => {
                        return Ok(expression_value);
                    },
                    None => return Err(format!("[ERROR] Variable {} is not defined!", name.get_lexeme())),
                }
            },
            None => {},
        }

        match enclosing_value {
            Some(reference) => {
                let enclosing = *reference;
                let val = enclosing.get(name.clone());
                
                match val {
                    Ok(result_value) => {
                        return Ok(result_value);
                    },
                    Err(e) => return Err(e),
                }
            },
            None => return Err(format!("[ERROR] {} is not defined!", name.get_lexeme())),
        }
    }

    pub fn assign(&mut self, name: String, data: Option<Expr>) -> Result<(),String> {
        let value = self.values.get(&name);

        let enclosing_value = self.get_enclosing();

        match value {
            Some(_) => {
                self.values.insert(name, data);
                return Ok(());
            },
            None => {},
        }

        match enclosing_value {
            Some(reference) => {
                let mut enclosing = *reference;
                let res = enclosing.assign(name.clone(), data.clone());

                if let Ok(_) = res {
                    return Ok(());
                }
            },
            None => {},
        }

        Err(format!("[ERROR] {} is not defined!", name))
    }
}