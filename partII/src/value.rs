use anyhow::anyhow;

use std::{fmt::Display, rc::Rc};

use crate::callable::Callable;

#[derive(Debug, Clone)]
pub enum Value {
    Callable(Rc<dyn Callable>),
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Callable { .. }, _) | (_, Self::Callable { .. }) => {
                // TODO: we should check if it point to the exact same location
                panic!("You can't compare functions")
            }
            (Self::String(left), Self::String(right)) => left == right,
            (Self::Number(left), Self::Number(right)) => left == right,
            (Self::Bool(left), Self::Bool(right)) => left == right,
            (Self::Nil, Self::Nil) => true,
            _ => false,
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        !self.is_falsy()
    }

    pub fn is_falsy(&self) -> bool {
        matches!(self, Self::Bool(false) | Self::Nil)
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }

    pub fn map_number(self, mut f: impl FnMut(f64) -> f64) -> Result<Self, anyhow::Error> {
        Ok(Self::Number(f(self.number()?)))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_callable(&self) -> bool {
        matches!(self, Self::Callable { .. })
    }

    pub fn number(self) -> Result<f64, anyhow::Error> {
        match self {
            Self::Number(n) => Ok(n),
            _ => Err(anyhow!("Expected `number` but instead got {:?}", self)),
        }
    }

    pub fn string(self) -> Result<String, anyhow::Error> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err(anyhow!("Expected `string` but instead got {:?}", self)),
        }
    }

    pub fn bool(self) -> Result<bool, anyhow::Error> {
        match self {
            Self::Bool(b) => Ok(b),
            _ => Err(anyhow!("Expected `bool` but instead got {:?}", self)),
        }
    }

    pub fn nil(self) -> Result<(), anyhow::Error> {
        match self {
            Self::Nil => Ok(()),
            _ => Err(anyhow!("Expected `nil` but instead got {:?}", self)),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Self::Number(f)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<Rc<dyn Callable>> for Value {
    fn from(fun: Rc<dyn Callable>) -> Self {
        Self::Callable(fun)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Callable { .. } => write!(f, "fun"),
            Self::String(s) => write!(f, "{}", s),
            Self::Number(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Nil => write!(f, "nil"),
        }
    }
}
