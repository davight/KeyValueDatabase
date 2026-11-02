use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum EngineError {
    Io(std::io::Error),
    WrongType,
    ParseError(String),
    ReadOnly,
}

impl Display for EngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::WrongType => {
               write!(f, "WRONGTYPE operation - value type mismatch")
            }
            EngineError::ReadOnly => {
                write!(f, "READONLY operation - you can not edit this database")
            }
            EngineError::ParseError(_) => {
                write!(f, "PARSING fail - could not parse file")
            }
            EngineError::Io(err) => {
                write!(f, "IO Error - {}", err)
            }
        }
    }
}


impl std::error::Error for EngineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EngineError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for EngineError {
    fn from(err: std::io::Error) -> Self {
        EngineError::Io(err)
    }
}

