use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum ApplicationError {
  InvalidModeError,
}

impl Display for ApplicationError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use ApplicationError::*;
    match *self {
      InvalidModeError => f.write_str("Invalid Mode Number"),
    }
  }
}

impl Error for ApplicationError {}
