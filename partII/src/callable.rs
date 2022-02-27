use crate::{environment::Environment, value::Value};

use anyhow::{anyhow, Result};

pub trait Callable: std::fmt::Debug {
    fn call(&self, env: &mut Environment, arguments: Vec<Value>) -> Result<Value>;
    fn arity(&self) -> usize;
}

impl Callable for Value {
    fn call(&self, env: &mut Environment, arguments: Vec<Value>) -> Result<Value> {
        match self {
            Self::Callable(fun) if fun.arity() != arguments.len() => Err(anyhow!(
                "Expected {} arguments but got {}.",
                fun.arity(),
                arguments.len()
            )),
            Self::Callable(fun) => fun.call(env, arguments),
            _ => Err(anyhow!("Can only call functions or classes.")),
        }
    }

    fn arity(&self) -> usize {
        match self {
            Self::Callable(fun) => fun.arity(),
            _ => panic!("Called arity on a non function value"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub arity: usize,
}

impl Callable for Function {
    fn call(&self, _env: &mut Environment, arguments: Vec<Value>) -> Result<Value> {
        if self.arity != arguments.len() {
            return Err(anyhow!(
                "Expected {} arguments but got {}.",
                self.arity,
                arguments.len()
            ));
        }

        todo!()
    }

    fn arity(&self) -> usize {
        self.arity
    }
}
