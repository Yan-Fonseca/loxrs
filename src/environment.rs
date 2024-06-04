use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone, Debug)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<String, Option<Expr>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_with_enclosing(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn set_enclosing_environment(&mut self, environment: Option<Rc<RefCell<Environment>>>) {
        self.enclosing = environment;
    }

    pub fn get_enclosing(&self) -> Option<Rc<RefCell<Environment>>> {
        self.enclosing.clone()
    }

    pub fn define(&mut self, name: String, value: Option<Expr>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Option<Expr>, String> {
        if let Some(value) = self.values.get(&name.get_lexeme()) {
            return Ok(value.clone());
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow().get(name);
        }

        Err(format!("[ERROR] {} is not defined!", name.get_lexeme()))
    }

    pub fn assign(&mut self, name: String, data: Option<Expr>) -> Result<(), String> {
        if self.values.contains_key(&name) {
            self.values.insert(name, data);
            return Ok(());
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow_mut().assign(name, data);
        }

        Err(format!("[ERROR] {} is not defined!", name))
    }
}
