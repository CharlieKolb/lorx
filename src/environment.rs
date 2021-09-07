use std::collections::{HashMap, VecDeque};

use crate::value::Er;

#[derive(Debug, Clone)]
pub struct EnvStack<T: std::fmt::Debug + Clone + PartialEq + Default> {
    envs: VecDeque<Environment<T>>,
}

impl<T: Clone + PartialEq + Default + std::fmt::Debug> Default for EnvStack<T> {
    fn default() -> Self {
        EnvStack::<T> {
            envs: VecDeque::default(),
        }
       
    }
}

impl<T: Clone + PartialEq + Default + std::fmt::Debug> EnvStack<T> {
    pub fn with_globals(globals: &Vec<(String, T)>) -> Self {
        let mut env_stack = EnvStack::<T> {
            envs: VecDeque::default(),
        };
        env_stack.push_default();
        for (name, val) in globals {
            env_stack.define(
                name,
                val.clone(),
            );
        }
        env_stack
    }
    pub fn push_default(&mut self) {
        self.envs.push_back(Default::default())
    }

    pub fn push(&mut self, env: Environment<T>) {
        self.envs.push_back(env)
    }

    pub fn pop(&mut self) -> Result<(), Er> {
        self.envs.pop_back().map(|_| ()).ok_or(Er::Code(45))
    }

    pub fn define(&mut self, name: &str, value: T) {
        if let Some(env) = self.envs.back_mut() {
            env.define(name, value)
        }
    }

    pub fn assign(&mut self, name: &str, value: T) -> Result<(), Er> {
        for env in self.envs.iter_mut().rev() {
            if env.get(name).is_ok() {
                env.assign(name, value).ok();
                return Ok(());
            }
        }

        Err(Er::Code(46))
    }

    pub fn get(&self, name: &str) -> Result<&T, Er> {
        for env in self.envs.iter().rev() {
            if let Ok(val) = env.get(name) {
                return Ok(val);
            }
        }

        Err(Er::Code(47))
    }

    pub fn resolve_depth(&self, name: &str) -> Option<usize> {
        for i in (0..self.envs.len()).rev() {
            if self.envs[i].get(name).is_ok() {
                return Some(i);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Default)]
pub struct Environment<T: std::fmt::Debug + Clone + Default> {
    values: HashMap<String, T>,
}

impl<T: std::fmt::Debug + Clone + Default> Environment<T> {
    pub fn define(&mut self, name: &str, value: T) {
        self.values.insert(name.to_string(), value);
    }

    pub fn assign(&mut self, name: &str, value: T) -> Result<(), Er> {
        if self.values.get(name).is_some() {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(Er::Code(43))
        }
    }

    pub fn get(&self, name: &str) -> Result<&T, usize> {
        self.values.get(name).ok_or(34)
    }
}
