pub mod engine;
pub mod log_engine;
pub mod memory_engine;
pub mod read_only_engine;
pub mod file_engine;
pub mod command;
pub mod errors;
pub mod types;

use file_engine::FileEngine;
use command::Command;
use engine::Engine;
use memory_engine::MemoryEngine;
use std::io;

fn run_main_loop(mut engine: impl Engine) {
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
                match engine.flush() {
                    Ok(_) => println!("{prefix} Database flushed."),
                    Err(_) => println!("{prefix} Could not flush the database.")
                }
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
    let path = std::env::args().nth(1).unwrap_or_else(|| "kvdb.aof".to_string());
    let file_engine = FileEngine::open(path, MemoryEngine::new());
    match file_engine {
        Ok(engine) => {
            run_main_loop(engine);
        }
        _ => {}
    }
    /*
    let use_log = true;
    let mut engine: Box<dyn Engine> = if use_log {
        Box::new(LoggingEngine::new(MemoryEngine::new()))
    } else {
        Box::new(MemoryEngine::default())
    };
    run_main_loop(engine.as_mut());
     */
}

