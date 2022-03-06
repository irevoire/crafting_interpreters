use std::rc::Rc;

use crate::{callable::Function, expr::Expr, token::Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Rc<Vec<Stmt>>),
    Class {
        name: Token,
        superclass: Option<Expr>,
        methods: Vec<Function>,
    },
    Expression(Expr),
    Function(Function),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print(Expr),
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}

impl Default for Stmt {
    fn default() -> Self {
        Stmt::Expression(Expr::default())
    }
}
