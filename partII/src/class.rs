use std::{collections::HashMap, fmt::Display};

use crate::{
    callable::{Callable, Function},
    instance::Instance,
};

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub methods: HashMap<String, Function>,
    pub superclass: Option<Box<Class>>,
}

impl Class {
    pub fn new(
        name: String,
        methods: HashMap<String, Function>,
        superclass: Option<Class>,
    ) -> Self {
        Class {
            name,
            methods,
            superclass: superclass.map(Box::new),
        }
    }

    pub fn find_method(&self, name: &str) -> Option<&Function> {
        self.methods.get(name)
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Callable for Class {
    fn call(
        &mut self,
        interpreter: &mut crate::interpreter::Interpreter,
        arguments: Vec<crate::value::Value>,
    ) -> Result<crate::value::Value, crate::error::RuntimeError> {
        let instance = Instance::new(self.clone());
        if let Some(initializer) = self.find_method("init") {
            initializer
                .bind(instance.clone())
                .call(interpreter, arguments)?;
        }

        Ok(instance.into())
    }

    fn arity(&self) -> usize {
        self.find_method("init").map(Callable::arity).unwrap_or(0)
    }
}
