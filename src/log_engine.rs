use crate::engine::Engine;
use crate::errors::EngineError;
use crate::types::Value;

pub struct LoggingEngine<E: Engine> {
    engine: E,
}

impl<E: Engine> LoggingEngine<E> {
    pub fn new(inner: E) -> Self {
        LoggingEngine { engine: inner }
    }
}

impl<E: Engine> Engine for LoggingEngine<E> {

    fn flush(&mut self) -> Result<(), EngineError> {
        let result = self.engine.flush();
        match &result {
            Ok(_) => println!("[LOG] Flushed"),
            Err(_) => println!("[LOG] Failed to flush."),
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
            Err(_) => println!("[LOG] Failed to put."),
        }
        return result;
    }

    fn l_push(&mut self, key: String, value: String) -> Result<(), EngineError> {
        let result = self.engine.l_push(key, value);
        match &result {
            Ok(..) => println!("[LOG] Pushed."),
            Err(_) => println!("[LOG] Failed to push."),
        }
        return result;
    }

    fn l_pop(&mut self, key: String) -> Result<Option<String>, EngineError> {
        let result = self.engine.l_pop(key);
        match &result {
            Ok(..) => println!("[LOG] Popped."),
            Err(_) => println!("[LOG] Failed to pop."),
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
            Err(_) => println!("[LOG] Failed to get."),
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
            Err(_) => println!("[LOG] Failed to delete."),
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
