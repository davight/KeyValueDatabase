use std::collections::{HashMap, VecDeque};

use crate::errors::EngineError;
use crate::errors::EngineError::WrongType;
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

#[derive(Default)]
pub struct MemoryEngine {
    storage: HashMap<String, Value>,
}

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
        Err(WrongType)
    }
    
    fn put(&mut self, key: String, value: Value) -> Result<Option<Value>, EngineError> {
        Err(WrongType)
    }
    
    fn l_push(&mut self, key: String, value: String) -> Result<(), EngineError> {
        Err(WrongType)
    }
    
    fn l_pop(&mut self, key: String) -> Result<Option<String>, EngineError> {
        Err(WrongType)
    }
    
    fn get(&self, key: &str) -> Result<Option<&Value>, EngineError> {
        Err(WrongType)
    }
    
    fn exist(&self, key: &str) -> bool {
        self.engine.exist(key)
    }
    
    fn del(&mut self, key: &str) -> Result<Option<Value>, EngineError> {
        Err(WrongType)
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

pub struct LoggingEngine<E: Engine> {
    engine: E,
}

impl<E: Engine> LoggingEngine<E> {
    pub(crate) fn new(inner: E) -> Self {
        LoggingEngine { engine: inner }
    }
}

impl<E: Engine> Engine for LoggingEngine<E> {

    fn flush(&mut self) -> Result<(), EngineError> {
        let result = self.engine.flush();
        match &result {
            Ok(_) => println!("[LOG] Flushed"),
            Err(e) => println!("[LOG] Failed to flush."),
        }
        return result;
    }

    fn put(&mut self, key: String, value: Value) -> Result<Option<Value>, EngineError> {
        let result = self.engine.put(key, value);
        match &result {
            Ok(option) => {
                match &option {
                    Some(old_val) => println!("[LOG] Inserted. Old value: {:?}", old_val),
                    None => println!("[LOG] Inserted."),
                }       
            },
            Err(e) => println!("[LOG] Failed to put."),
        }
        return result;
    }

    fn l_push(&mut self, key: String, value: String) -> Result<(), EngineError> {
        let result = self.engine.l_push(key, value);
        match &result {
            Ok(..) => println!("[LOG] Pushed."),
            Err(e) => println!("[LOG] Failed to push."),
        }
        return result;
    }

    fn l_pop(&mut self, key: String) -> Result<Option<String>, EngineError> {
        let result = self.engine.l_pop(key);
        match &result {
            Ok(..) => println!("[LOG] Popped."),
            Err(e) => println!("[LOG] Failed to pop."),
        }
        return result;
    }

    fn get(&self, key: &str) -> Result<Option<&Value>, EngineError> {
        let result = self.engine.get(key);
        match &result {
            Ok(option) => {
                match option {
                    Some(value) => println!("[LOG] Found value: {:?}", value),
                    None => println!("[LOG] No value"),
                }       
            }
            Err(e) => println!("[LOG] Failed to get."),
        }
        return result;
    }

    fn exist(&self, key: &str) -> bool {
        let result = self.engine.exist(key);
        if result {
            println!("[LOG] Key exists.");
        }
        else {
            println!("[LOG] Key not found.");
        }
        return result;
    }

    fn del(&mut self, key: &str) -> Result<Option<Value>, EngineError> {
        let result = self.engine.del(key);
        match &result {
            Ok(option) => {
                match option {
                    Some(old_val) => println!("[LOG] Deleted. Old value: {:?}", old_val),
                    None => println!("[LOG] Deleted."),
                }
            }
            Err(e) => println!("[LOG] Failed to delete."),
        }
        return result;
    }

    fn print(&self) {
        println!("[LOG] Printing.");
        self.engine.print();
    }

    fn matcher_get(&self, matcher: &str) -> Vec<String> {
        let result = self.engine.matcher_get(matcher);
        println!("[LOG] Match result: {:?}", result);
        return result;
    }

    fn size(&self) -> usize {
        let result = self.engine.size();
        println!("[LOG] Size: {}", result);
        return result;
    }
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
