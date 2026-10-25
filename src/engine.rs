use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    String(String),
    List(VecDeque<String>),
}

pub struct DbEngine {
    storage: HashMap<String, Value>,
}

pub enum DbEngineError {
    WrongType,
}

impl DbEngine {
    pub fn new() -> Self {
        DbEngine { storage: HashMap::new() }
    }
    
    pub fn flush(&mut self) {
        self.storage.clear();
    }
    
    pub fn put(&mut self, key: String, value: Value) -> Option<Value> {
        return self.storage.insert(key, value);
    }

    pub fn l_push(&mut self, key: String, value: String) -> Result<(), DbEngineError> {
        let entry = self.storage.entry(key).or_insert_with(|| Value::List(VecDeque::new()));
        if let Value::List(list) = entry {
            list.push_front(value);
            Ok(())
        } else {
            Err(DbEngineError::WrongType)
        }
    }

    pub fn l_pop(&mut self, key: String) -> Result<Option<String>, DbEngineError> {
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
                Err(DbEngineError::WrongType)
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        return self.storage.get(key);
    }

    pub fn exist(&self, key: &str) -> bool {
        return self.storage.contains_key(key);
    }

    pub fn del(&mut self, key: &str) -> Option<Value> {
        return self.storage.remove(key);
    }

    pub fn print(&self) {
        for key in self.storage.keys() {
            println!("{} -> {:?}", key, self.storage.get(key));
        }
    }
    
    pub fn matcher_get(&self, matcher: &str) -> Vec<String> {
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

    pub fn size(&self) -> usize {
        return self.storage.len();
    }
}
