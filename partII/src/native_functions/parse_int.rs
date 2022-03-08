use std::rc::Rc;

use anyhow::anyhow;

use crate::{callable::Callable, error::RuntimeError, interpreter::Interpreter, value::Value};

#[derive(Debug)]
pub struct ParseInt {}

impl ParseInt {
    pub fn value() -> Value {
        let parse_int = Rc::new(Self {}) as Rc<dyn Callable>;
        parse_int.into()
    }
}

impl Callable for ParseInt {
    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        if arguments.len() != 1 {
            return Err(anyhow!("`ParseInt` expect one string argument."))?;
        }

        let int = arguments[0]
            .clone()
            .string()?
            .parse::<f64>()
            .map_err(|e| anyhow!("`ParseInt` was not able to parse {}. {}", arguments[0], e))?;

        Ok(int.into())
    }

    fn arity(&self) -> usize {
        1
    }
}
