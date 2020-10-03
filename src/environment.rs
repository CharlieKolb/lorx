use std::collections::HashMap;

use crate::value::Value;

#[derive(Debug, Clone, Default)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Result<&Value, usize> {
        self.values.get(name).ok_or(34)
    }
}