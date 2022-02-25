use crate::{expr::Expr, stmt::Stmt, token::TokenType, value::Value};
use anyhow::anyhow;

pub fn interpret(stmts: Vec<Stmt>) -> Result<(), anyhow::Error> {
    for stmt in stmts {
        stmt.evaluate()?;
    }
    Ok(())
}

impl Stmt {
    pub fn evaluate(self) -> Result<(), anyhow::Error> {
        match self {
            Stmt::Expression(expr) => drop(expr.evaluate()),
            Stmt::Print(expr) => println!("{}", expr.evaluate()?),
        }
        Ok(())
    }
}

impl Expr {
    pub fn evaluate(self) -> Result<Value, anyhow::Error> {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let (left, right) = (left.evaluate()?, right.evaluate()?);

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
            Expr::Grouping { expression } => expression.evaluate(),
            Expr::Literal { value } => Ok(value),
            Expr::Unary { operator, right } => match operator.ty {
                TokenType::Bang => Ok((right.evaluate()?.is_falsy()).into()),
                TokenType::Minus => right.evaluate()?.map_number(|n| -n),
                _ => unreachable!(),
            },
        }
    }
}
