use crate::parser::Rule;
use std::{fmt::Display, io, string::FromUtf8Error};

pub enum Error {
    IO(io::Error),
    FromUtf8(FromUtf8Error),
    Parser(Box<pest::error::Error<Rule>>),
    Serde(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Self::FromUtf8(value)
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(value: pest::error::Error<Rule>) -> Self {
        Self::Parser(Box::new(value))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(io_error) => write!(f, "{io_error}"),
            Error::FromUtf8(from_utf8_error) => write!(f, "{from_utf8_error}"),
            Error::Parser(parser_error) => write!(f, "{parser_error}"),
            Error::Serde(serde_error) => write!(f, "{serde_error}"),
        }
    }
}
