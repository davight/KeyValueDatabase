use crate::engine::Value;

pub enum Command {
    Put(String, Value),
    Get(String),
    MGet(Vec<String>),
    Contains(String),
    Del(String),
    LPush(String, String),
    LPop(String),
    DbSize(),
    DbFlush(),
    Keys(String),
    Print(),
    Exit,
    Unknown(String)
}

impl Command {
    pub fn parse(input: &str) -> Option<Self> {
        let args = input.split_whitespace().collect::<Vec<&str>>();
        if args.is_empty() {
            return None;
        }
        let cmd = &args[0].to_uppercase();
        match (cmd.as_str(), &args[1..]) {
            ("PUT", [key, rest_value @ ..]) => Some(Command::Put(key.to_string(), Self::parse_value(&*rest_value.join(" ")))),
            ("GET", [key]) => Some(Command::Get(key.to_string())),
            ("MGET", keys) if !keys.is_empty() => {
                let vec = keys.iter().map(| k| k.to_string()).collect::<Vec<String>>();
                Some(Command::MGet(vec))
            }
            ("LPUSH", [key, value]) => Some(Command::LPush(key.to_string(), value.to_string())), //Some(Command::LPush(key, String::from(args[2..].join(" "))))
            ("LPOP", [key]) => Some(Command::LPop(key.to_string())),
            ("DBSIZE", _) => Some(Command::DbSize()),
            ("DBFLUSH", _) => Some(Command::DbFlush()),
            ("CONTAINS", [key]) => Some(Command::Contains(key.to_string())),
            ("DELETE", [key]) => Some(Command::Del(key.to_string())),
            ("PRINT", _) => Some(Command::Print()),
            ("KEYS", [matcher]) => Some(Command::Keys(matcher.to_string())),
            ("EXIT", _) => Some(Command::Exit),
            _ => Some(Command::Unknown(cmd.to_string())),
        }
    }

    fn parse_value(input: &str) -> Value {
        match input.parse::<i64>() {
            Ok(number) => Value::Integer(number),
            Err(_) => Value::String(input.to_string()),
        }
    }
}
