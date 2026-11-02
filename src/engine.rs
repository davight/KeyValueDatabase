use crate::errors::EngineError;
use crate::types::Value;

pub trait Engine {
    fn flush(&mut self) -> Result<(), EngineError>;

    fn put(&mut self, key: String, value: Value) -> Result<Option<Value>, EngineError>;

    fn l_push(&mut self, key: String, value: String) -> Result<(), EngineError>;

    fn l_pop(&mut self, key: String) -> Result<Option<String>, EngineError>;

    fn get(&self, key: &str) -> Result<Option<&Value>, EngineError>;

    fn exist(&self, key: &str) -> bool;

    fn del(&mut self, key: &str) -> Result<Option<Value>, EngineError>;

    fn print(&self);

    fn matcher_get(&self, matcher: &str) -> Vec<String>;

    fn size(&self) -> usize;
}
