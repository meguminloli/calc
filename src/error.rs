use std::{
    fmt::{Debug, Display, Formatter},
    num::ParseFloatError,
};

/// Error enum for the crate.
/// It implements `std::error::Error` for ease of use
/// These errors can occur during the parsing, the conversion to rpn, or the evaluation
/// of the tree
pub enum Error {
    /// Unknown token error
    UnknownToken,
    /// Invalid token error
    InvalidToken,
    /// Unbalanced parenthesis error
    UnbalancedParens,
    /// Unfinished expression error
    UnfinishedExpr,
    /// Number parsing error
    NumberParseError,
    /// Not enough arguments to functions
    NotEnoughArgs,
    /// Unexpected token error
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
