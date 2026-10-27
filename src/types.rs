use std::collections::VecDeque;

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    String(String),
    List(VecDeque<String>),
}