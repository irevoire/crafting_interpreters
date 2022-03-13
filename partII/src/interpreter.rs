use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{
    callable::Callable,
    class::Class,
    environment::Environment,
    error::RuntimeError,
    expr::Expr,
    native_functions,
    stmt::Stmt,
    token::{Token, TokenType},
    value::Value,
};
use anyhow::anyhow;

type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug, Clone, Default)]
pub struct Interpreter {
    pub env: Environment,
    pub locals: HashMap<*const Expr, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self::default();

        interpreter.define(String::from("clock"), native_functions::Clock::value());
        interpreter.define(
            String::from("readLines"),
            native_functions::ReadLines::value(),
        );
        interpreter.define(
            String::from("parseInt"),
            native_functions::ParseInt::value(),
        );

        interpreter
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) -> std::result::Result<(), RuntimeError> {
        for stmt in stmts {
            stmt.evaluate(self)?;
        }
        Ok(())
    }

    fn lookup_variable(&mut self, name: &Token, expr: &Expr) -> Result<&Value> {
        if let Some(distance) = self.locals.get(&(expr as *const Expr)) {
            self.get_at(*distance, name)
        } else {
            self.globals().get(name)
        }
    }

    pub fn resolve(&mut self, expr: &Expr, depth: usize) {
        self.locals.insert(expr, depth);
    }
}

impl Deref for Interpreter {
    type Target = Environment;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

impl DerefMut for Interpreter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.env
    }
}

impl Stmt {
    pub fn evaluate(&self, interpreter: &mut Interpreter) -> Result<()> {
        match self {
            Stmt::Block(stmts) => {
                let previous_env = std::mem::take(&mut interpreter.env);
                interpreter.enclose(previous_env);

                for stmt in stmts.iter() {
                    match stmt.evaluate(interpreter) {
                        Ok(_) => (),
                        Err(e) => {
                            interpreter.env =
                                std::mem::take(&mut interpreter.env).destroy().unwrap();
                            return Err(e);
                        }
                    }
                }
                interpreter.env = std::mem::take(&mut interpreter.env).destroy().unwrap();
            }
            Stmt::Class {
                name,
                superclass,
                methods,
            } => {
                let superclass = superclass
                    .as_ref()
                    .map(|superclass| -> Result<_> {
                        let superclass = superclass.evaluate(interpreter)?;
                        match superclass {
                            Value::Class(class) => Ok(class),
                            _ => Err(anyhow!("Superclass must be a class"))?,
                        }
                    })
                    .transpose()?;
                interpreter.define(name.lexeme.clone(), Value::Nil);

                if let Some(_) = superclass {
                    interpreter.env.enclosed_by(Environment::new());
                    interpreter.define("super", superclass.clone().unwrap());
                }

                let methods = methods
                    .iter()
                    .map(|method| (method.name.lexeme.clone(), method.clone()))
                    .collect();

                let class = Class::new(name.lexeme.clone(), methods, superclass.clone());
                if let Some(_) = superclass {
                    interpreter.declose();
                }
                interpreter.assign(name, class.into())?;
            }
            Stmt::Expression(expr) => drop(expr.evaluate(interpreter)?),
            Stmt::Function(fun) => fun.evaluate(interpreter)?,
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if condition.evaluate(interpreter)?.is_truthy() {
                    then_branch.evaluate(interpreter)?;
                } else if let Some(else_branch) = else_branch {
                    else_branch.evaluate(interpreter)?;
                }
            }
            Stmt::Print(expr) => println!("{}", expr.evaluate(interpreter)?),
            Stmt::Return { value, .. } => {
                let value = value
                    .as_ref()
                    .unwrap_or(&Expr::default())
                    .evaluate(interpreter)?;
                return Err(RuntimeError::Return(value));
            }
            Stmt::While { condition, body } => {
                while condition.evaluate(interpreter)?.is_truthy() {
                    body.evaluate(interpreter)?;
                }
            }
            Stmt::Var { name, initializer } => {
                let value = initializer
                    .as_ref()
                    .unwrap_or(&Expr::Literal { value: Value::Nil })
                    .evaluate(interpreter)?;
                interpreter.define(name.lexeme.clone(), value);
            }
        }
        Ok(())
    }
}

impl Expr {
    pub fn evaluate(&self, interpreter: &mut Interpreter) -> Result<Value> {
        match self {
            Expr::Assign { name, value } => {
                let value = value.evaluate(interpreter)?;
                interpreter.assign(&name, value.clone())?;
                Ok(value)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let (left, right) = (left.evaluate(interpreter)?, right.evaluate(interpreter)?);

                match operator.ty {
                    TokenType::Slash => Ok((left.number()? / right.number()?).into()),
                    TokenType::Star => Ok((left.number()? * right.number()?).into()),
                    TokenType::Minus => Ok((left.number()? - right.number()?).into()),
                    TokenType::Plus if left.is_string() || right.is_string() => {
                        Ok((left.to_string() + &right.to_string()).into())
                    }
                    TokenType::Plus if left.is_number() => {
                        Ok((left.number()? + right.number()?).into())
                    }
                    TokenType::Plus => Err(anyhow!(
                        "Operator `+` can only be applied to `string` or `number`"
                    ))?,
                    TokenType::Greater => Ok((left.number()? > right.number()?).into()),
                    TokenType::GreaterEqual => Ok((left.number()? >= right.number()?).into()),
                    TokenType::Less => Ok((left.number()? < right.number()?).into()),
                    TokenType::LessEqual => Ok((left.number()? <= right.number()?).into()),
                    TokenType::BangEqual => Ok((left == right).into()),
                    TokenType::EqualEqual => Ok((left == right).into()),
                    _ => unreachable!(),
                }
            }
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                let mut callee = callee.evaluate(interpreter)?;

                let arguments = arguments
                    .into_iter()
                    .map(|arg| arg.evaluate(interpreter))
                    .collect::<Result<Vec<_>>>()?;

                callee.call(interpreter, arguments)
            }
            Expr::Get { object, name } => {
                let object = object.evaluate(interpreter)?;
                match object {
                    Value::Instance(object) => object.get(name),
                    _ => Err(anyhow!("Only object have properties."))?,
                }
            }
            Expr::Grouping { expression } => expression.evaluate(interpreter),
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(interpreter)?;

                if operator.ty == TokenType::Or {
                    if left.is_truthy() {
                        return Ok(left);
                    }
                } else {
                    if left.is_falsy() {
                        return Ok(left);
                    }
                }

                right.evaluate(interpreter)
            }
            Expr::Set {
                object,
                name,
                value,
            } => {
                let object = object.evaluate(interpreter)?;
                match object {
                    Value::Instance(mut object) => {
                        let value = value.evaluate(interpreter)?;
                        object.set(name, value.clone());
                        Ok(value)
                    }
                    _ => Err(anyhow!("Only instances have fields."))?,
                }
            }
            Expr::Super { keyword, method } => {
                println!("HERE");
                let distance = interpreter.locals.get(&(self as *const Expr)).unwrap();
                println!("distance: {}", distance);
                println!("{:#?}", interpreter.env);
                // let superclass = interpreter.get_at(*distance, keyword)?.clone().class()?;
                let superclass = interpreter.get(keyword)?.clone().class()?;
                println!("HERE2");
                let object = interpreter
                    .env
                    .get_at(*distance - 1, "this")?
                    .clone()
                    .instance()?;

                println!("HOARE");
                if let Some(method) = superclass.find_method(method) {
                    Ok(method.bind(object).into())
                } else {
                    Err(anyhow!("Can't use `super` in a class with no superclass"))?
                }
            }
            Expr::This { keyword } => Ok(interpreter.lookup_variable(keyword, self)?.clone()),
            Expr::Unary { operator, right } => match operator.ty {
                TokenType::Bang => Ok((right.evaluate(interpreter)?.is_falsy()).into()),
                TokenType::Minus => right.evaluate(interpreter)?.map_number(|n| -n),
                _ => unreachable!(),
            },
            Expr::Variable { name } => Ok(interpreter.lookup_variable(name, self)?.clone()),
        }
    }
}
