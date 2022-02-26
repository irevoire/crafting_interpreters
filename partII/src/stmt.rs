use crate::{expr::Expr, token::Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print(Expr),
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
}
