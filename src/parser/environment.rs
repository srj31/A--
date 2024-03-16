use std::collections;

use crate::{lexer::token::Token, log};

use super::interpreter::Object;

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: collections::HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            values: collections::HashMap::new(),
        }
    }

    pub fn new_enclosed(env: Environment) -> Environment {
        Environment {
            enclosing: Some(Box::new(env)),
            values: collections::HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Object {
        match self.values.get(&name.lexeme) {
            Some(v) => v.clone(),
            None => match self.enclosing {
                Some(ref e) => e.get(name),
                None => Object::Nil,
            },
        }
    }

    pub fn assign(&mut self, var: &Token, expr: Object) {
        match self.values.get_mut(&var.lexeme) {
            Some(v) => *v = expr,
            None => {
                if let Some(ref mut e) = self.enclosing {
                    return e.assign(var, expr);
                }
                log::log_message::print_code_error(
                    var.line,
                    &format!("Undefined variable '{}'", var.lexeme),
                );
            }
        }
    }
}
