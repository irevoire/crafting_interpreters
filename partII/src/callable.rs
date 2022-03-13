use std::rc::Rc;

use crate::{
    environment::Environment,
    error::RuntimeError,
    instance::Instance,
    interpreter::Interpreter,
    stmt::Stmt,
    token::{Token, TokenType},
    value::Value,
};

use anyhow::anyhow;

pub trait Callable: std::fmt::Debug {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError>;
    fn arity(&self) -> usize;
}

impl Callable for Value {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        match self {
            Self::Callable(fun) if fun.arity() != arguments.len() => Err(anyhow!(
                "Expected {} arguments but got {}.",
                fun.arity(),
                arguments.len()
            ))?,
            Self::Callable(fun) => unsafe {
                Rc::get_mut_unchecked(fun).call(interpreter, arguments)
            },
            // Should I copy paste the code above?
            Self::Class(class) => Value::Callable(Rc::new(class.clone()) as Rc<dyn Callable>)
                .call(interpreter, arguments),
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

    pub is_initializer: bool,
    pub closure: Option<Environment>,
}

impl Function {
    pub fn evaluate(&self, env: &mut Environment) -> Result<(), RuntimeError> {
        let value = self.clone().with_environment(env.clone()).to_value();
        env.define(self.name.lexeme.to_string(), value);
        Ok(())
    }

    pub fn with_environment(self, env: Environment) -> Self {
        Self {
            closure: Some(env),
            ..self
        }
    }

    pub fn to_value(&self) -> Value {
        (Rc::new(self.clone()) as Rc<dyn Callable>).into()
    }

    pub fn bind(&self, instance: Instance) -> Self {
        let mut environment = Environment::new();
        environment.define("this", instance);
        Self {
            name: self.name.clone(),
            params: self.params.clone(),
            body: self.body.clone(),
            is_initializer: self.is_initializer,
            closure: Some(environment),
        }
    }
}

impl Callable for Function {
    fn call(
        &mut self,
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

        // we create a new interpreter just for the execution of this function
        let mut local_interpreter = Interpreter::new();

        // we extract the global env and the locals from the original interpreter
        std::mem::swap(&mut local_interpreter.locals, &mut interpreter.locals);
        std::mem::swap(local_interpreter.globals_mut(), interpreter.globals_mut());

        let is_closure = self.closure.is_some();

        // extract the env of the closure
        let mut env = Environment::new();
        let closure = self.closure.as_mut().unwrap_or(&mut env);
        let closure = std::mem::take(closure);

        local_interpreter.enclosed_by(closure);

        for (param, arg) in self.params.iter().zip(arguments) {
            local_interpreter.define(param.lexeme.to_string(), arg);
        }

        let result = match Stmt::Block(self.body.clone()).evaluate(&mut local_interpreter) {
            Ok(()) => Ok(Value::Nil),
            Err(RuntimeError::Return(value)) => Ok(value),
            Err(e) => Err(e),
        };

        // we restore the global env and the locals
        std::mem::swap(local_interpreter.globals_mut(), interpreter.globals_mut());
        std::mem::swap(&mut local_interpreter.locals, &mut interpreter.locals);

        // restore the env of the closure
        let env = local_interpreter.env.pop();

        if is_closure {
            self.closure = Some(env);
        }

        let result = result?;

        if self.is_initializer && self.closure.is_some() {
            Ok(self
                .closure
                .as_ref()
                .unwrap()
                .get_at(
                    0,
                    &Token {
                        ty: TokenType::This,
                        lexeme: "this".to_string(),
                        line: 0,
                    },
                )
                .unwrap()
                .clone())
        } else {
            Ok(result)
        }
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}
