use std::collections::{HashMap, VecDeque};

use crate::value::Value;

#[derive(Debug, Clone)]
pub struct EnvStack {
    envs: VecDeque<Environment>,
}

impl Default for EnvStack {
    fn default() -> Self {
        Self {
            envs: VecDeque::from(vec![Default::default()]),
        }
    }
}

impl EnvStack {
    pub fn push_default(&mut self) {
        self.envs.push_back(Default::default())
    }

    pub fn push(&mut self, env: Environment) {
        self.envs.push_back(env)
    }

    pub fn pop(&mut self) -> Result<(), usize> {
        self.envs.pop_back().map(|_| ()).ok_or(45)
    }

    pub fn define(&mut self, name: &str, value: Value) {
        if let Some(env) = self.envs.back_mut() {
            env.define(name, value)
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), usize> {
        for env in self.envs.iter_mut().rev() {
            if env.get(name).is_ok() {
                env.assign(name, value).ok();
                return Ok(());
            }
        }

        Err(46)
    }

    pub fn get(&self, name: &str) -> Result<&Value, usize> {
        for env in self.envs.iter().rev() {
            if let Ok(val) = env.get(name) {
                return Ok(val);
            }
        }

        Err(47)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), usize> {
        if self.values.get(name).is_some() {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(43)
        }
    }

    pub fn get(&self, name: &str) -> Result<&Value, usize> {
        self.values.get(name).ok_or(34)
    }
}
