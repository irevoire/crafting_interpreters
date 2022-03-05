use std::rc::Rc;

use crate::{
    environment::Environment, error::RuntimeError, interpreter::Interpreter, stmt::Stmt,
    token::Token, value::Value,
};

use anyhow::anyhow;

pub trait Callable: std::fmt::Debug {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError>;
    fn arity(&self) -> usize;
}

impl Callable for Value {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        match self {
            Self::Callable(fun) if fun.arity() != arguments.len() => Err(anyhow!(
                "Expected {} arguments but got {}.",
                fun.arity(),
                arguments.len()
            ))?,
            Self::Callable(fun) => fun.call(interpreter, arguments),
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
    pub body: Rc<Vec<Stmt>>,
}

impl Function {
    pub fn evaluate(&self, env: &mut Environment) -> Result<(), RuntimeError> {
        env.define(self.name.lexeme.to_string(), self.to_value());
        Ok(())
    }

    pub fn to_value(&self) -> Value {
        (Rc::new(self.clone()) as Rc<dyn Callable>).into()
    }
}

impl Callable for Function {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        if self.params.len() != arguments.len() {
            return Err(anyhow!(
                "Expected {} arguments but got {}.",
                self.params.len(),
                arguments.len()
            ))?;
        }

        let previous_env = std::mem::take(&mut interpreter.env);
        interpreter.env.enclose(previous_env);

        for (param, arg) in self.params.iter().zip(arguments) {
            interpreter.define(param.lexeme.to_string(), arg);
        }

        let result = match Stmt::Block(self.body.clone()).evaluate(interpreter) {
            Ok(()) => Ok(Value::Nil),
            Err(RuntimeError::Return(value)) => Ok(value),
            Err(e) => Err(e),
        };

        interpreter.env = std::mem::take(&mut interpreter.env).destroy().unwrap();

        result
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}
