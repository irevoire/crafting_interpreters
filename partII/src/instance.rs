use std::{collections::HashMap, fmt::Display, rc::Rc};

use crate::{class::Class, error::RuntimeError, token::Token, value::Value};

use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct Instance {
    class: Class,
    // we want to keep the same env between all calls on a same instance
    fields: Rc<HashMap<String, Value>>,
}

impl Instance {
    pub fn new(class: Class) -> Self {
        Self {
            class,
            fields: Rc::default(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
        if let Some(field) = self.fields.get(&name.lexeme) {
            Ok(field.clone())
        } else if let Some(method) = self.class.find_method(&name.lexeme) {
            Ok(method.bind(self.clone()).into())
        } else {
            Err(anyhow!("Undefined property `{}`.", name.lexeme))?
        }
    }

    pub fn set(&mut self, name: &Token, value: Value) {
        unsafe { Rc::get_mut_unchecked(&mut self.fields) }.insert(name.lexeme.clone(), value);
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class.name)
    }
}
