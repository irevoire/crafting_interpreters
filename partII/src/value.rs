use anyhow::anyhow;

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Number(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Nil => write!(f, "nil"),
        }
    }
}
