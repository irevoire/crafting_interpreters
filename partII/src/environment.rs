use std::collections::HashMap;

use crate::{error::RuntimeError, token::Token, value::Value};

#[derive(Default, Debug, Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enclose(&mut self, enclosing: Self) {
        self.enclosing = Some(Box::new(enclosing));
    }

    pub fn destroy(self) -> Option<Environment> {
        self.enclosing.map(|env| *env)
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        *self.get_mut(name)? = value;
        Ok(())
    }

    pub fn get(&self, name: &Token) -> Result<&Value, RuntimeError> {
        Ok(self
            .values
            .get(&name.lexeme)
            .or_else(|| {
                self.enclosing
                    .as_ref()
                    .map(|env| env.get(name).ok())
                    .flatten()
            })
            .ok_or(anyhow::anyhow!("Undefined variable `{}`.", name.lexeme))?)
    }

    pub fn get_mut(&mut self, name: &Token) -> Result<&mut Value, RuntimeError> {
        Ok(self
            .values
            .get_mut(&name.lexeme)
            .or_else(|| {
                self.enclosing
                    .as_mut()
                    .map(|env| env.get_mut(name).ok())
                    .flatten()
            })
            .ok_or(anyhow::anyhow!("Undefined variable `{}`.", name.lexeme))?)
    }
}
