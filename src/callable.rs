use std::fmt;

use crate::interpreter::Interpreter;
use crate::parser::StmtFunction;
use crate::value::{Er, Value};

pub trait Callable: fmt::Debug {
    fn artiy(&self) -> usize;
    fn call(&self, _: &mut Interpreter, _: Vec<Value>) -> Result<Value, Er>;
}

#[derive(Debug, Clone)]
pub struct Function {
    pub declaration: StmtFunction,
}

impl Callable for Function {
    fn artiy(&self) -> usize {
        self.declaration.params.len()
    }
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, Er> {
        let mut temp = std::mem::take(&mut interpreter.envs);
        interpreter.envs.push_default();
        for (param, arg) in self.declaration.params.iter().zip(args.iter()) {
            interpreter.envs.define(param, arg.clone());
        }
        let ret = interpreter.eval_block(&self.declaration.body);
        interpreter.envs.pop()?;
        std::mem::swap(&mut interpreter.envs, &mut temp);
        ret
    }
}
