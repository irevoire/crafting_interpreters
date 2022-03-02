use std::fmt::Display;

use crate::{callable::Callable, instance::Instance};

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
}

impl Class {
    pub fn new(name: String) -> Self {
        Class { name }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Callable for Class {
    fn call(
        &self,
        _interpreter: &mut crate::interpreter::Interpreter,
        _arguments: Vec<crate::value::Value>,
    ) -> Result<crate::value::Value, crate::error::RuntimeError> {
        Ok(Instance::new(self.clone()).into())
    }

    fn arity(&self) -> usize {
        0
    }
}
