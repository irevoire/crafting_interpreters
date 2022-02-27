use std::rc::Rc;

use crate::{
    environment::Environment, error::RuntimeError, stmt::Stmt, token::Token, value::Value,
};

use anyhow::anyhow;

pub trait Callable: std::fmt::Debug {
    fn call(&self, env: &mut Environment, arguments: Vec<Value>) -> Result<Value, RuntimeError>;
    fn arity(&self) -> usize;
}

impl Callable for Value {
    fn call(&self, env: &mut Environment, arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        match self {
            Self::Callable(fun) if fun.arity() != arguments.len() => Err(anyhow!(
                "Expected {} arguments but got {}.",
                fun.arity(),
                arguments.len()
            ))?,
            Self::Callable(fun) => fun.call(env, arguments),
            _ => Err(anyhow!("Can only call functions or classes."))?,
        }
    }

    fn arity(&self) -> usize {
        match self {
            Self::Callable(fun) => fun.arity(),
            _ => panic!("Called arity on a non function value"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl Function {
    pub fn evaluate(&self, env: &mut Environment) -> Result<(), RuntimeError> {
        let fun = Rc::new(self.clone()) as Rc<dyn Callable>;
        env.define(self.name.lexeme.to_string(), fun.into());
        Ok(())
    }
}

impl Callable for Function {
    fn call(&self, env: &mut Environment, arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if self.params.len() != arguments.len() {
            return Err(anyhow!(
                "Expected {} arguments but got {}.",
                self.params.len(),
                arguments.len()
            ))?;
        }

        let previous_env = std::mem::take(env);
        env.enclose(previous_env);

        for (param, arg) in self.params.iter().zip(arguments) {
            env.define(param.lexeme.to_string(), arg);
        }

        let result = match Stmt::Block(self.body.clone()).evaluate(env) {
            Ok(()) => Ok(Value::Nil),
            Err(RuntimeError::Return(value)) => Ok(value),
            Err(e) => Err(e),
        };

        *env = std::mem::take(env).destroy().unwrap();

        result
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}
