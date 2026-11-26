use std::fmt;
use std::io;

#[derive(Debug)]
pub enum StoreError {
    Io(io::Error),
    Parse { line: usize, reason: String },
    NotFound(u32),
    EmptyField(&'static str),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::Io(err) => write!(f, "I/O error: {err}"),
            StoreError::Parse { line, reason } => {
                write!(f, "parse error on line {line}: {reason}")
            }
            StoreError::NotFound(id) => write!(f, "entry with ID {id} not found"),
            StoreError::EmptyField(field) => write!(f, "field '{field}' cannot be empty"),
        }
    }
}

impl std::error::Error for StoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StoreError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for StoreError {
    fn from(err: io::Error) -> Self {
        StoreError::Io(err)
    }
}