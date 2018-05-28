use std::{error, fmt};

// pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.kind.as_str())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.kind.as_str()
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    InvalidSettings,
    NameNotFound,
    PathNotFound,

    #[doc(hidden)]
    __Nonexhaustive,
}

impl ErrorKind {
    fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::InvalidSettings => "invalid settings found",
            ErrorKind::NameNotFound => "distribution name not found",
            ErrorKind::PathNotFound => "path not found",
            ErrorKind::__Nonexhaustive => unreachable!(),
        }
    }
}
