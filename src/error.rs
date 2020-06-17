use std::{error, fmt};

/// Standard Error type for this crate.
#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidCurrency,
    InvalidAmount,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidCurrency => write!(f, "Currency was not valid"),
            Error::InvalidAmount => write!(f, "Amount is not a number"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidCurrency => "Currency was not valid",
            Error::InvalidAmount => "Amount is not a number",
        }
    }
}