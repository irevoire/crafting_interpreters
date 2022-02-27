use std::{
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::ensure;

use crate::{callable::Callable, environment::Environment, value::Value};

#[derive(Debug)]
pub struct Clock {}

impl Clock {
    pub fn value() -> Value {
        let clock = Rc::new(Self {}) as Rc<dyn Callable>;
        clock.into()
    }
}

impl Callable for Clock {
    fn call(&self, _env: &mut Environment, arguments: Vec<Value>) -> anyhow::Result<Value> {
        ensure!(arguments.is_empty(), "`clock` expect no argument.");

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
