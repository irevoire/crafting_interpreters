use std::fmt::Display;

use crate::{token::Token, value::Value};

#[derive(Debug, Clone, PartialEq, Eq)]
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
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Get {
        object: Box<Expr>,
        name: Token,
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
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
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

    pub fn unwrap_variable(&self) -> &Token {
        match self {
            Self::Variable { name } => name,
            expr => panic!("Called unwrap variable on a {:#?}.", expr),
        }
    }
}

impl Default for Expr {
    fn default() -> Self {
        Expr::Literal {
            value: Value::default(),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assign { name, .. } => write!(f, "assign {}", name.lexeme),
            Self::Binary { operator, .. } => write!(f, "{}", operator.lexeme),
            Self::Call { .. } => write!(f, "call"),
            Self::Get { .. } | Self::Set { .. } => write!(f, "."),
            Self::Grouping { .. } => write!(f, "grouping"),
            Self::Logical { operator, .. } => write!(f, "{}", operator.lexeme),
            Self::Literal { value } => write!(f, "{}", value),
            Expr::Variable { name } => write!(f, "{}", name),
            Expr::Super { .. } => write!(f, "super"),
            Self::Unary { operator, .. } => write!(f, "{}", operator.lexeme),
            Expr::This { .. } => write!(f, "this"),
        }
    }
}
