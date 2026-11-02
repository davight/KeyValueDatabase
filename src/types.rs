use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    String(String),
    List(VecDeque<String>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::String(s) => write!(f, "{}", s),
            Value::List(l) => {
                let items: Vec<String> = l.iter().map(|s| s.to_string()).collect();
                write!(f, "[{}]", items.join(", "))
            }
        }
    }
}

