use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: String,
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

    pub fn group(expr: Expr) -> Self {
        Self::Grouping {
            expression: Box::new(expr),
        }
    }

    pub fn literal(value: String) -> Self {
        Self::Literal { value }
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
            Self::Binary { operator, .. } => write!(f, "{}", operator.lexeme),
            Self::Grouping { .. } => write!(f, "grouping"),
            Self::Literal { value } => write!(f, "{}", value),
            Self::Unary { operator, .. } => write!(f, "{}", operator.lexeme),
        }
    }
}
