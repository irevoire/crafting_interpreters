use std::{
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::anyhow;

use crate::{callable::Callable, error::RuntimeError, interpreter::Interpreter, value::Value};

#[derive(Debug)]
pub struct Clock {}

impl Clock {
    pub fn value() -> Value {
        let clock = Rc::new(Self {}) as Rc<dyn Callable>;
        clock.into()
    }
}

impl Callable for Clock {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        if !arguments.is_empty() {
            return Err(anyhow!("`clock` expect no argument."))?;
        }

        let start = SystemTime::now();
        let timestamp = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Ok(timestamp.as_secs_f64().into())
    }

    fn arity(&self) -> usize {
        0
    }
}
