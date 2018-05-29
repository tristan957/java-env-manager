extern crate serde_json;

use std::{error, fmt, io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind:        ErrorKind,
    description: String,
}

impl Error {
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn new_with_desc(kind: ErrorKind, description: String) -> Self {
        Error { kind, description }
    }

    pub fn new(kind: ErrorKind) -> Self {
        Error {
            kind,
            description: String::from(kind.as_str()),
        }
    }
}

impl From<io::Error> for Error {
    fn from(_e: io::Error) -> Self {
        Error::new(ErrorKind::IoError)
    }
}

impl From<serde_json::Error> for Error {
    fn from(_e: serde_json::Error) -> Self {
        Error::new(ErrorKind::SerdeError)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.kind.as_str())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.description.as_str()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    BinariesNotFound,
    DuplicateNames,
    IoError,
    NameNotFound,
    PathExists,
    PathNotFound,
    SerdeError,
    SettingsNotFound,
    SettingsReadFailure,
    SettingsWriteFailure,

    #[doc(hidden)]
    __Nonexhaustive,
}

impl ErrorKind {
    fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::BinariesNotFound => "bin folder not found",
            ErrorKind::DuplicateNames => "duplicate names found",
            ErrorKind::IoError => "unable to perform I/O",
            ErrorKind::NameNotFound => "distribution name not found",
            ErrorKind::PathExists => "path exists",
            ErrorKind::PathNotFound => "path not found",
            ErrorKind::SerdeError => "unable to serialize/deserialize settings",
            ErrorKind::SettingsNotFound => "settings not found",
            ErrorKind::SettingsReadFailure => "failed to read settings",
            ErrorKind::SettingsWriteFailure => "failed to write settings",
            ErrorKind::__Nonexhaustive => unreachable!(),
        }
    }
}
