use crate::{environment::Environment, expr::Expr, stmt::Stmt, token::TokenType, value::Value};
use anyhow::anyhow;

#[derive(Debug, Clone, Default)]
pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), anyhow::Error> {
        for stmt in stmts {
            stmt.evaluate(&mut self.env)?;
        }
        Ok(())
    }
}

impl Stmt {
    pub fn evaluate(self, env: &mut Environment) -> Result<(), anyhow::Error> {
        match self {
            Stmt::Block(stmts) => {
                let previous_env = std::mem::take(env);
                env.enclose(previous_env);

                for stmt in stmts {
                    match stmt.evaluate(env) {
                        Ok(_) => (),
                        Err(e) => {
                            *env = std::mem::take(env).destroy().unwrap();
                            return Err(e);
                        }
                    }
                }
                *env = std::mem::take(env).destroy().unwrap();
            }
            Stmt::Expression(expr) => drop(expr.evaluate(env)),
            Stmt::Print(expr) => println!("{}", expr.evaluate(env)?),
            Stmt::Var { name, initializer } => {
                let value = initializer
                    .unwrap_or(Expr::Literal { value: Value::Nil })
                    .evaluate(env)?;
                env.define(name.lexeme, value);
            }
        }
        Ok(())
    }
}

impl Expr {
    pub fn evaluate(self, env: &mut Environment) -> Result<Value, anyhow::Error> {
        match self {
            Expr::Assign { name, value } => {
                let value = value.evaluate(env)?;
                env.assign(&name, value.clone())?;
                Ok(value)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let (left, right) = (left.evaluate(env)?, right.evaluate(env)?);

                match operator.ty {
                    TokenType::Slash => Ok((left.number()? / right.number()?).into()),
                    TokenType::Star => Ok((left.number()? * right.number()?).into()),
                    TokenType::Minus => Ok((left.number()? - right.number()?).into()),
                    TokenType::Plus if left.is_number() => {
                        Ok((left.number()? + right.number()?).into())
                    }
                    TokenType::Plus if left.is_string() => {
                        Ok((left.string()? + &right.string()?).into())
                    }
                    TokenType::Plus => Err(anyhow!(
                        "Operator `+` can only be applied to `string` or `number`"
                    )),
                    TokenType::Greater => Ok((left.number()? > right.number()?).into()),
                    TokenType::GreaterEqual => Ok((left.number()? >= right.number()?).into()),
                    TokenType::Less => Ok((left.number()? < right.number()?).into()),
                    TokenType::LessEqual => Ok((left.number()? <= right.number()?).into()),
                    TokenType::BangEqual => Ok((left == right).into()),
                    TokenType::EqualEqual => Ok((left == right).into()),
                    _ => unreachable!(),
                }
            }
            Expr::Grouping { expression } => expression.evaluate(env),
            Expr::Literal { value } => Ok(value),
            Expr::Unary { operator, right } => match operator.ty {
                TokenType::Bang => Ok((right.evaluate(env)?.is_falsy()).into()),
                TokenType::Minus => right.evaluate(env)?.map_number(|n| -n),
                _ => unreachable!(),
            },
            Expr::Variable { name } => env.get(&name).cloned(),
        }
    }
}
