use std::{error::Error, fmt};

#[derive(Debug)]
pub struct MessageError(String);

impl MessageError {
    pub fn new(msg: &str) -> Self {
        Self(String::from(msg))
    }
}

impl Error for MessageError {}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
