use crate::callable::Callable;
use crate::interpreter::Interpreter;
use crate::value::{Er, Value};

#[derive(Debug)]
pub struct Clock {}

impl Callable for Clock {
    fn artiy(&self) -> usize {
        0
    }
    fn call(&self, _: &mut Interpreter, _: Vec<Value>) -> Result<Value, Er> {
        Ok(Value::Number(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("error")
                .as_millis() as f64,
        ))
    }
}
