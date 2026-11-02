use crate::engine::Engine;
use crate::errors::EngineError;
use crate::errors::EngineError::ReadOnly;
use crate::types::Value;

pub struct ReadOnlyEngine<E: Engine> {
    engine: E,
}

impl <E: Engine> ReadOnlyEngine<E> {
    pub fn new(inner: E) -> Self {
        ReadOnlyEngine { engine: inner }
    }
}

impl <E: Engine> Engine for ReadOnlyEngine<E> {
    fn flush(&mut self) -> Result<(), EngineError> {
        Err(ReadOnly)
    }

    fn put(&mut self, _key: String, _value: Value) -> Result<Option<Value>, EngineError> {
        Err(ReadOnly)
    }

    fn l_push(&mut self, _key: String, _value: String) -> Result<(), EngineError> {
        Err(ReadOnly)
    }

    fn l_pop(&mut self, _key: String) -> Result<Option<String>, EngineError> {
        Err(ReadOnly)
    }

    fn get(&self, _key: &str) -> Result<Option<&Value>, EngineError> {
        Err(ReadOnly)
    }

    fn exist(&self, key: &str) -> bool {
        self.engine.exist(key)
    }

    fn del(&mut self, _key: &str) -> Result<Option<Value>, EngineError> {
        Err(ReadOnly)
    }

    fn print(&self) {
        self.engine.print()
    }

    fn matcher_get(&self, matcher: &str) -> Vec<String> {
        self.engine.matcher_get(matcher)
    }

    fn size(&self) -> usize {
        self.engine.size()
    }
}
