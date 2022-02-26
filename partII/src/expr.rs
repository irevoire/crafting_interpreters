use std::fmt::Display;

use crate::{token::Token, value::Value};

#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Value,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn logical(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn group(expr: Expr) -> Self {
        Self::Grouping {
            expression: Box::new(expr),
        }
    }

    pub fn literal(value: impl Into<Value>) -> Self {
        Self::Literal {
            value: value.into(),
        }
    }

    pub fn unary(operator: Token, right: Expr) -> Self {
        Self::Unary {
            operator,
            right: Box::new(right),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assign { name, .. } => write!(f, "assign {}", name.lexeme),
            Self::Binary { operator, .. } => write!(f, "{}", operator.lexeme),
            Self::Grouping { .. } => write!(f, "grouping"),
            Self::Logical { operator, .. } => write!(f, "{}", operator.lexeme),
            Self::Literal { value } => write!(f, "{}", value),
            Self::Unary { operator, .. } => write!(f, "{}", operator.lexeme),
            Expr::Variable { name } => write!(f, "{}", name),
        }
    }
}
