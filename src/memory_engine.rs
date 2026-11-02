use std::collections::{HashMap, VecDeque};
use crate::engine::Engine;
use crate::errors::EngineError;
use crate::types::Value;

#[derive(Default)]
pub struct MemoryEngine {
    storage: HashMap<String, Value>,
}

impl MemoryEngine {
    pub fn new() -> Self {
        return MemoryEngine::default();
    }
}

impl Engine for MemoryEngine {
    fn flush(&mut self) -> Result<(), EngineError> {
        return Ok(self.storage.clear());
    }

    fn put(&mut self, key: String, value: Value) -> Result<Option<Value>, EngineError> {
        return Ok(self.storage.insert(key, value));
    }

    fn l_push(&mut self, key: String, value: String) -> Result<(), EngineError> {
        let entry = self.storage.entry(key).or_insert_with(|| Value::List(VecDeque::new()));
        if let Value::List(list) = entry {
            list.push_front(value);
            Ok(())
        } else {
            Err(EngineError::WrongType)
        }
    }

    fn l_pop(&mut self, key: String) -> Result<Option<String>, EngineError> {
        match self.storage.get_mut(&key) {
            None => Ok(None),
            Some(Value::List(list)) => {
                let value = list.pop_front();
                if list.is_empty() {
                    self.storage.remove(&key);
                }
                Ok(value)
            }
            _ => {
                Err(EngineError::WrongType)
            }
        }
    }

    fn get(&self, key: &str) -> Result<Option<&Value>, EngineError> {
        return Ok(self.storage.get(key));
    }

    fn exist(&self, key: &str) -> bool {
        return self.storage.contains_key(key);
    }

    fn del(&mut self, key: &str) -> Result<Option<Value>, EngineError> {
        return Ok(self.storage.remove(key));
    }

    fn print(&self) {
        for key in self.storage.keys() {
            println!("{} -> {:?}", key, self.storage.get(key));
        }
    }

    fn matcher_get(&self, matcher: &str) -> Vec<String> {
        if matcher.eq("*") {
            return self.storage.keys().cloned().collect();
        }
        let mut vec: Vec<String> = Vec::new();
        if !matcher.contains("*") {
            if self.exist(matcher) {
                vec.push(matcher.to_string());
            }
            return vec;
        }
        let to_find = matcher.replace("*", "");
        if matcher.starts_with("*") && matcher.ends_with("*") {
            for key in self.storage.keys() {
                if key.contains(&to_find) {
                    vec.push(key.clone());
                }
            }
            return vec;
        }
        if matcher.starts_with("*") {
            for key in self.storage.keys() {
                if key.ends_with(&to_find) {
                    vec.push(key.clone());
                }
            }
            return vec;
        }
        if matcher.ends_with("*") {
            for key in self.storage.keys() {
                if key.starts_with(&to_find) {
                    vec.push(key.clone());
                }
            }
            return vec;
        }
        return vec;
    }

    fn size(&self) -> usize {
        return self.storage.len();
    }
}
