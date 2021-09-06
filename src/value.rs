use std::rc::Rc;

use crate::callable::Callable;

#[derive(Debug, Clone)]
pub enum Er {
    Code(usize),
    Return(Value),
}

#[derive(Debug, Clone)]
pub enum Value {
    Text(String),
    Number(f64),
    Boolean(bool),
    Nil,
    Callable(Rc<dyn Callable>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Self::Text(s) => s.clone(),
            Self::Number(f) => f.to_string(),
            Self::Boolean(b) => b.to_string(),
            Self::Nil => "null".to_string(),
            Self::Callable(_) => "Callable".to_owned(),
        }
    }
}
