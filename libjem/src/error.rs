extern crate serde_json;

use std::{error, fmt, io, result};

/// Rust idiom for masking error type within libraries
pub type Result<T> = result::Result<T, Error>;

/// Error type for `libjem` operations.
/// Includes the kind of error and a short description.
#[derive(Debug)]
pub struct Error {
    kind:        ErrorKind,
    description: String,
}

impl Error {
    // Returns the description of the error
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Returns the kind of error
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Creates an [`Error`] object with a default description
    pub fn new(kind: ErrorKind) -> Self {
        Error {
            kind,
            description: String::from(kind.as_str()),
        }
    }

    /// Creates an [`Error`] object with a custom description
    pub fn new_with_desc<T>(kind: ErrorKind, description: T) -> Self
    where
        T: Into<String>,
    {
        Error {
            kind,
            description: description.into(),
        }
    }
}

impl From<io::Error> for Error {
    /// Convert an `std::io::Error` to an [`Error`]
    fn from(_e: io::Error) -> Self {
        Error::new(ErrorKind::IoError)
    }
}

impl From<serde_json::Error> for Error {
    /// Convert an `serde_json::Error` to an [`Error`]
    fn from(_e: serde_json::Error) -> Self {
        Error::new(ErrorKind::SerdeError)
    }
}

impl fmt::Display for Error {
    /// How to display an [`Error`]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.kind.as_str())
    }
}

impl error::Error for Error {
    /// Returns a description of the [`Error`]
    fn description(&self) -> &str {
        self.description.as_str()
    }
}

/// Serves to provide the kind of [`Error`] in an enumerated fashion.
/// Typically used in the creation of a new [`Error`] or a `match` expression.
///
/// # Examples
///
/// ```
/// use libjem::error::{Error, ErrorKind};
///
/// let e = Error::new(ErrorKind::IoError);
/// match e.kind() {
///     ErrorKind::BinariesNotFound => {},
///     ErrorKind::DuplicateNames => {},
///     // ...
///     _ => {},
/// }
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    /// Symlinked binaries are not found within `JAVA_ENV_MANAGER_HOME`
    BinariesNotFound,
    /// Custom error for client application
    Custom,
    /// More than one distribution has the same name
    DuplicateNames,
    /// Cause was an `std::io::Error`
    IoError,
    /// Name being searched for was not found
    NameNotFound,
    /// Path already exists
    PathExists,
    /// Path was not found
    PathNotFound,
    /// Unable to serialize/deserialize the `settings.json` file to a
    /// `Settings` struct
    SerdeError,
    /// `settings.json` could not be found
    SettingsNotFound,
    /// Unable to read the `settings.json` file
    SettingsReadFailure,
    /// Unable to write to the `settings.json` file
    SettingsWriteFailure,

    #[doc(hidden)]
    __Nonexhaustive,
}

impl ErrorKind {
    /// Describes the kind of error as a string
    fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::BinariesNotFound => "bin folder not found",
            ErrorKind::Custom => "custom error: please override",
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
