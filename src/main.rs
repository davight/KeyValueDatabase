pub mod engine;
pub mod command;
pub mod errors;
pub mod types;

use std::io;
use engine::{Engine, MemoryEngine};
use command::Command;
use crate::engine::LoggingEngine;

fn run_main_loop(engine: &mut dyn Engine) {
    println!("--- Kvdb ---\nEnter input:");
    let mut input = String::new();
    let prefix = "+>";
    loop {
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {},
            Err(e) => {
                eprintln!("{prefix} Error reading input: {}", e);
                break;
            }
        }

        let command = match Command::parse(input.trim()) {
            Some(command) => command,
            None => {
                println!("{prefix} Invalid command");
                continue;
            }
        };
        match command {
            Command::Get(key) => {
                match engine.get(&key).ok() {
                    Some(val) => println!("{prefix} {:?}", val),
                    None => println!("{prefix} Key '{key}' not found"),
                }
            }
            Command::MGet(keys) => {
                for key in keys {
                    match engine.get(&key).ok() {
                        Some(val) => println!("{prefix} {:?}", val),
                        None => println!("{prefix} Key '{key}' not found"),
                    }
                }
            }
            Command::Put(key, value) => {
                match engine.put(key, value).ok() {
                    Some(old_val) => println!("{prefix} Inserted. The old value was: '{old_val:?}'"),
                    None => println!("{prefix} Inserted"),
                }
            }
            Command::Contains(key) => {
                println!("{prefix} {}", engine.exist(&key));
            }
            Command::Del(key) => {
                match engine.del(&key).ok() {
                    Some(old_val) => println!("{prefix} Deleted. The old value was: '{old_val:?}'"),
                    None => println!("{prefix} Key '{key}' not found."),
                }
            }
            Command::LPush(key, value) => {
                let output = engine.l_push(key, value);
                match output {
                    Err(_) => println!("{prefix} Error encountered."),
                    Ok(_) => println!("{prefix} Value pushed.")
                }
            }
            Command::LPop(key) => {
                let output = engine.l_pop(key);
                match output {
                    Err(_) => println!("{prefix} Error encountered."),
                    Ok(_) => println!("{prefix} Value popped.")
                }
            }
            Command::Print() => {
                if engine.size() == 0 {
                    println!("{prefix} Database is empty.");
                }
                else {
                    engine.print();
                }
            }
            Command::Keys(matcher) => {
                let found = engine.matcher_get(&*matcher);
                let size = found.len();
                for key in found {
                    println!("{}", key);
                }
                println!("{prefix} Found '{size}' keys.");
            }
            Command::Exit => {
                break;
            }
            Command::DbFlush() => {
                engine.flush();
                println!("{prefix} Database flushed.");
            }
            Command::DbSize() => {
                let size = engine.size();
                println!("{prefix} Database has {size} entries.");
            }
            Command::Unknown(args) => {
                println!("{prefix} Unknown command: '{}'", args);
            }
        }
    }
}

fn main() {
    let use_log = true;
    let mut engine: Box<dyn Engine> = if (use_log) {
        Box::new(LoggingEngine::new(MemoryEngine::new()))
    } else {
        Box::new(MemoryEngine::default())
    };
    run_main_loop(engine.as_mut());
}

