use std::{collections::HashMap, fmt::Display};

use crate::{class::Class, error::RuntimeError, token::Token, value::Value};

use anyhow::anyhow;

#[derive(Debug)]
pub struct Instance {
    class: Class,
    fields: HashMap<String, Value>,
}

impl Instance {
    pub fn new(class: Class) -> Self {
        Self {
            class,
            fields: HashMap::default(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<&Value, RuntimeError> {
        Ok(self
            .fields
            .get(&name.lexeme)
            .ok_or(anyhow!("Undefined property `{}`.", name.lexeme))?)
    }

    pub fn set(&mut self, name: &Token, value: Value) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class.name)
    }
}
