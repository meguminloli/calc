use std::{
    fmt::{Debug, Display, Formatter},
    num::ParseFloatError,
};

pub enum Error {
    UnknownToken,
    InvalidToken,
    UnbalancedParens,
    UnfinishedExpr,
    NumberParseError,
    NotEnoughArgs,
    UnexpectedToken,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Error::UnknownToken => write!(f, "Unknown token"),
            Error::InvalidToken => write!(f, "Invalid token"),
            Error::UnbalancedParens => write!(f, "Unbalanced parentheses"),
            Error::UnfinishedExpr => write!(f, "Unfinished expression"),
            Error::NumberParseError => write!(f, "Number parse error"),
            Error::NotEnoughArgs => write!(f, "Not enough arguments"),
            Error::UnexpectedToken => write!(f, "Unexpected token"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Self {
        Error::NumberParseError
    }
}

impl std::error::Error for Error {}
