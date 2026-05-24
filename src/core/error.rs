use thiserror::Error;

/// Errors returned by vsleep operations.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// An I/O error, typically from opening a spinners file.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// A JSON parse error from deserializing a spinners file.
    #[cfg(feature = "serde")]
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    /// The requested spinner name was not found in the loaded set.
    #[error("spinner not found: '{0}'")]
    SpinnerNotFound(String),

    /// A general error with a freeform message.
    #[error("{0}")]
    Msg(String),
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Error::Msg(s.into())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Msg(s)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
