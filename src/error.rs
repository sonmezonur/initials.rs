//! Error module includes the custom error types.
use std::num::ParseIntError;
use std::io;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "unexpected hex color format: expected({}), got({})", expected, actual)]
    InvalidHexFormat {
        actual: String,
        expected: String,
    },

    #[fail(display = "couldn't parse hex value")]
    Parse(ParseIntError),
    #[fail(display = "io error")]
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
