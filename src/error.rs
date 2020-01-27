use dbus;
use std::convert::From;
use std::error;
use std::fmt;
use std::io;
use std::sync::mpsc;
use toml::de;

#[derive(Debug)]
pub enum Error {
    NoBatteryFound,
    ConfigNotFound(io::Error),
    MalformedConfig(de::Error),
    DBusError(dbus::Error),
    SendError,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ConfigNotFound(ref err) => Some(err),
            Error::MalformedConfig(ref err) => Some(err),
            Error::DBusError(ref err) => Some(err),
            Error::NoBatteryFound => None,
            Error::SendError => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::NoBatteryFound => write!(f, "No battery device found"),
            Error::ConfigNotFound(ref err) => err.fmt(f),
            Error::MalformedConfig(ref err) => err.fmt(f),
            Error::DBusError(ref err) => err.fmt(f),
            Error::SendError => write!(f, "mpsc::send failed"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::ConfigNotFound(err)
    }
}

impl From<dbus::Error> for Error {
    fn from(err: dbus::Error) -> Self {
        Error::DBusError(err)
    }
}

impl<T> From<mpsc::SendError<T>> for Error {
    fn from(_err: mpsc::SendError<T>) -> Self {
        Error::SendError
    }
}
