use std::collections::{HashMap, VecDeque};

use crate::globals;
use crate::value::{Er, Value};

#[derive(Debug, Clone)]
pub struct EnvStack {
    envs: VecDeque<Environment>,
}

impl Default for EnvStack {
    fn default() -> Self {
        let mut env_stack = EnvStack {
            envs: VecDeque::default(),
        };
        env_stack.push_default();
        env_stack.define(
            "clock",
            Value::Callable(std::rc::Rc::new(globals::Clock {})),
        );
        env_stack
    }
}

impl EnvStack {
    pub fn push_default(&mut self) {
        self.envs.push_back(Default::default())
    }

    pub fn push(&mut self, env: Environment) {
        self.envs.push_back(env)
    }

    pub fn pop(&mut self) -> Result<(), Er> {
        self.envs.pop_back().map(|_| ()).ok_or(Er::Code(45))
    }

    pub fn define(&mut self, name: &str, value: Value) {
        if let Some(env) = self.envs.back_mut() {
            env.define(name, value)
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), Er> {
        for env in self.envs.iter_mut().rev() {
            if env.get(name).is_ok() {
                env.assign(name, value).ok();
                return Ok(());
            }
        }

        Err(Er::Code(46))
    }

    pub fn get(&self, name: &str) -> Result<&Value, Er> {
        for env in self.envs.iter().rev() {
            if let Ok(val) = env.get(name) {
                return Ok(val);
            }
        }

        Err(Er::Code(47))
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

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), Er> {
        if self.values.get(name).is_some() {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(Er::Code(43))
        }
    }

    pub fn get(&self, name: &str) -> Result<&Value, usize> {
        self.values.get(name).ok_or(34)
    }
}
