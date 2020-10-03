#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Text(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Self::Text(s) => s.clone(),
            Self::Number(f) => f.to_string(),
            Self::Boolean(b) => b.to_string(),
            Self::Nil => "null".to_string(),
        }
    }
}
