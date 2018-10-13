//! Error module includes the custom error types.
use std::num::ParseIntError;
use std::io;

/// Custom Error type for Avatar
#[derive(Debug, Fail)]
pub enum Error {
    /// Invalid hex string
    #[fail(display = "unexpected hex color format: expected({}), got({})", expected, actual)]
    InvalidHexFormat {
        actual: String,
        expected: String,
    },
    /// Parse error
    #[fail(display = "couldn't parse hex value: {}", _0)]
    Parse(ParseIntError),
    /// IO read/write error
    #[fail(display = "IO error: {}", _0)]
    IO(io::Error)
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::Parse(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
    }
}
