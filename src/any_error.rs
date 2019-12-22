use std::error::Error;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub struct AnyError(String);

impl AnyError {
    pub fn new(msg: String) -> AnyError {
        AnyError(msg)
    }
}

impl fmt::Display for AnyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AnyError: {}", self.0)
    }
}

impl Error for AnyError {}
