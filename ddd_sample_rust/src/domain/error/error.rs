use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    BadRequestError(String),
    ValidationError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::MyError::*;
        match self {
            BadRequestError(s) => write!(f, "BadRequestError: {}", s),
            ValidationError(s) => write!(f, "BadRequestError: {}", s),
        }
    }
}

impl Error for MyError {}
