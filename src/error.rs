use std::convert::From;
use std::error;
use std::fmt;
use std::io;
use toml::de;

#[derive(Debug)]
pub enum Error {
    ConfigNotFound(io::Error),
    MalformedConfig(de::Error),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ConfigNotFound(ref err) => Some(err),
            Error::MalformedConfig(ref err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::ConfigNotFound(ref err) => err.fmt(f),
            Error::MalformedConfig(ref err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::ConfigNotFound(err)
    }
}
