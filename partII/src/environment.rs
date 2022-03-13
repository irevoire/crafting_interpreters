use std::collections::HashMap;

use crate::{error::RuntimeError, token::Token, value::Value};

#[derive(Default, Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enclose(&mut self, enclosing: Self) {
        self.enclosing = Some(Box::new(enclosing));
    }

    pub fn enclosed_by(&mut self, mut enclosing: Self) {
        std::mem::swap(self, &mut enclosing);
        self.enclosing = Some(Box::new(enclosing));
    }

    /// Reverse a `enclosed_by`
    pub fn declose(&mut self) {
        let inner = *self.enclosing.take().unwrap();
        // drop the old env
        let _ = std::mem::replace(self, inner);
    }

    pub fn destroy(self) -> Option<Environment> {
        self.enclosing.map(|env| *env)
    }

    pub fn pop(mut self) -> Environment {
        self.enclosing = None;
        self
    }

    pub fn define(&mut self, name: impl AsRef<str>, value: impl Into<Value>) {
        self.values.insert(name.as_ref().to_string(), value.into());
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        *self.get_mut(name)? = value;
        Ok(())
    }

    pub fn globals(&self) -> &Self {
        if self.enclosing.is_some() {
            self.enclosing.as_ref().unwrap().globals()
        } else {
            self
        }
    }

    pub fn globals_mut(&mut self) -> &mut Self {
        if self.enclosing.is_some() {
            self.enclosing.as_mut().unwrap().globals_mut()
        } else {
            self
        }
    }

    pub fn get(&self, name: impl AsRef<str>) -> Result<&Value, RuntimeError> {
        let name = name.as_ref();
        Ok(self
            .values
            .get(name)
            .or_else(|| {
                self.enclosing
                    .as_ref()
                    .map(|env| env.get(name).ok())
                    .flatten()
            })
            .ok_or(anyhow::anyhow!("Undefined variable `{}`.", name))?)
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

    pub fn get_at(&self, distance: usize, name: impl AsRef<str>) -> Result<&Value, RuntimeError> {
        if distance == 0 {
            self.get(name)
        } else {
            self.get_at(distance - 1, name)
        }
    }

    pub fn get_at_mut(
        &mut self,
        distance: usize,
        name: &Token,
    ) -> Result<&mut Value, RuntimeError> {
        if distance == 0 {
            self.get_mut(name)
        } else {
            self.get_at_mut(distance - 1, name)
        }
    }
}
