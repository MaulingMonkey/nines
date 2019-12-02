/// A generic nines error.  Currently opaque by design.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Error(ErrorKind);

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            ErrorKind::Generic(msg) => write!(fmt, "{}", msg),
        }
    }
}

pub(crate) fn err<T>(value: impl Into<ErrorKind>) -> Result<T, Error> {
    Err(Error(value.into()))
}



#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ErrorKind {
    Generic(&'static str),
}

impl From<&'static str> for ErrorKind {
    fn from(value: &'static str) -> Self { ErrorKind::Generic(value) }
}
