use crate::command::Command;
use crate::engine::Engine;
use crate::errors::EngineError;
use crate::types::Value;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct FileEngine<E: Engine> {
    inner: E,
    log_file: File
}

impl<E: Engine> FileEngine<E> {
    pub fn open<P: AsRef<Path>>(path: P, mut inner: E) -> Result<Self, EngineError> {
        let log_file = OpenOptions::new().read(true).write(true).create(true).open(path)?;
        let reader = BufReader::new(&log_file);
        for (n, line) in reader.lines().enumerate() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let parse_error = || EngineError::ParseError(format!("Invalid line {}: {line}", n + 1));
            match Command::parse(&line).ok_or_else(parse_error)? {
                Command::Put(key, value) => { inner.put(key, value)?; }
                Command::Del(key) => { inner.del(&key)?; }
                Command::LPush(key, value) => { inner.l_push(key, value)?; }
                Command::LPop(key) => { inner.l_pop(key)?; }
                Command::DbFlush() => { inner.flush()?; }
                _ => { return Err(parse_error()); }
            };
        }
        Ok(FileEngine { inner, log_file })
    }

    pub fn append(&mut self, line: &str) -> Result<(), EngineError> {
        writeln!(self.log_file, "{}", line)?;
        Ok(())
    }
}

impl<E: Engine> Engine for FileEngine<E> {
    fn flush(&mut self) -> Result<(), EngineError> {
        let line = "DBFLUSH";
        let res = self.inner.flush()?;
        writeln!(self.log_file, "{}", line)?;
        Ok(res)
    }

    fn put(&mut self, key: String, value: Value) -> Result<Option<Value>, EngineError> {
        let line = format!("PUT {}", key);
        let old = self.inner.put(key, value)?;
        self.append(line.as_str())?;
        Ok(old)
    }

    fn l_push(&mut self, key: String, value: String) -> Result<(), EngineError> {
        let line = format!("LPUSH {}", key);
        let res = self.inner.l_push(key, value)?;
        self.append(line.as_str())?;
        Ok(res)
    }

    fn l_pop(&mut self, key: String) -> Result<Option<String>, EngineError> {
        let line = format!("LPOP {}", key);
        let res = self.inner.l_pop(key)?;
        self.append(line.as_str())?;
        Ok(res)
    }

    fn get(&self, key: &str) -> Result<Option<&Value>, EngineError> {
        self.inner.get(key)
    }

    fn exist(&self, key: &str) -> bool {
        self.inner.exist(key)
    }

    fn del(&mut self, key: &str) -> Result<Option<Value>, EngineError> {
        let line = format!("DEL {}", key);
        let old = self.inner.del(key)?;
        self.append(line.as_str())?;
        Ok(old)
    }

    fn print(&self) {
        self.inner.print()
    }

    fn matcher_get(&self, matcher: &str) -> Vec<String> {
        self.inner.matcher_get(matcher)
    }

    fn size(&self) -> usize {
        self.inner.size()
    }
}