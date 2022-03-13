use std::rc::Rc;

use anyhow::anyhow;

use crate::{callable::Callable, error::RuntimeError, interpreter::Interpreter, value::Value};

#[derive(Debug)]
pub struct ReadLines {}

impl ReadLines {
    pub fn value() -> Value {
        let read_lines = Rc::new(Self {}) as Rc<dyn Callable>;
        read_lines.into()
    }
}

impl Callable for ReadLines {
    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        if !arguments.is_empty() {
            return Err(anyhow!("`ReadLines` expect no argument."))?;
        }

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut chars = line.chars();
        chars.next_back();
        let line = chars.as_str();

        if line.is_empty() {
            Ok(Value::Nil)
        } else {
            Ok(line.into())
        }
    }

    fn arity(&self) -> usize {
        0
    }
}
