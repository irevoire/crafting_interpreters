use anyhow::anyhow;

use std::{fmt::Display, rc::Rc};

use crate::{
    callable::{Callable, Function},
    class::Class,
    error::RuntimeError,
    instance::Instance,
};

#[derive(Debug, Clone)]
pub enum Value {
    Callable(Rc<dyn Callable>),
    Class(Class),
    Instance(Instance),
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl Default for Value {
    fn default() -> Self {
        Self::Nil
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Callable(left), Self::Callable(right)) => Rc::as_ptr(left) == Rc::as_ptr(right),
            (Self::String(left), Self::String(right)) => left == right,
            (Self::Number(left), Self::Number(right)) => left == right,
            (Self::Bool(left), Self::Bool(right)) => left == right,
            (Self::Nil, Self::Nil) => true,
            _ => false,
        }
    }
}

impl Eq for Value {}

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

    pub fn map_number(self, mut f: impl FnMut(f64) -> f64) -> Result<Self, RuntimeError> {
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

    pub fn number(self) -> Result<f64, RuntimeError> {
        match self {
            Self::Number(n) => Ok(n),
            _ => Err(anyhow!("Expected `number` but instead got {:?}", self))?,
        }
    }

    pub fn string(self) -> Result<String, RuntimeError> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err(anyhow!("Expected `string` but instead got {:?}", self))?,
        }
    }

    pub fn bool(self) -> Result<bool, RuntimeError> {
        match self {
            Self::Bool(b) => Ok(b),
            _ => Err(anyhow!("Expected `bool` but instead got {:?}", self))?,
        }
    }

    pub fn class(self) -> Result<Class, RuntimeError> {
        match self {
            Self::Class(class) => Ok(class),
            _ => Err(anyhow!("Expected `class` but instead got {:?}", self))?,
        }
    }

    pub fn instance(self) -> Result<Instance, RuntimeError> {
        match self {
            Self::Instance(instance) => Ok(instance),
            _ => Err(anyhow!("Expected `instance` but instead got {:?}", self))?,
        }
    }

    pub fn nil(self) -> Result<(), RuntimeError> {
        match self {
            Self::Nil => Ok(()),
            _ => Err(anyhow!("Expected `nil` but instead got {:?}", self))?,
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

impl From<Class> for Value {
    fn from(class: Class) -> Self {
        Self::Class(class)
    }
}

impl From<Function> for Value {
    fn from(fun: Function) -> Self {
        Self::Callable(Rc::new(fun))
    }
}

impl From<Instance> for Value {
    fn from(instance: Instance) -> Self {
        Self::Instance(instance)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Callable { .. } => write!(f, "fun"),
            Self::Class { .. } => write!(f, "class"),
            Self::Instance(i) => write!(f, "{}", i),
            Self::String(s) => write!(f, "{}", s),
            Self::Number(n) => write!(f, "{}", n),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Nil => write!(f, "nil"),
        }
    }
}
