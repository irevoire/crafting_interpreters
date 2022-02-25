use std::collections::HashMap;

use crate::{token::Token, value::Value};

#[derive(Default, Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<&Value, anyhow::Error> {
        self.values
            .get(&name.lexeme)
            .ok_or(anyhow::anyhow!("Undefined variable `{}`.", name.lexeme))
    }

    pub fn get_mut(&mut self, name: &Token) -> Result<&mut Value, anyhow::Error> {
        self.values
            .get_mut(&name.lexeme)
            .ok_or(anyhow::anyhow!("Undefined variable `{}`.", name.lexeme))
    }
}
