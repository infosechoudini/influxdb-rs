use serde::{Deserialize, Serialize};
use std::fmt;
use std::io;
use futures::Future;
use std::pin::Pin;
use futures::task::{Poll, Context};


/// Influxdb-rs Error
#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Error {
    /// Holds the inner error kind
    pub inner: ErrorKind,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}


/// The error of influxdb client
#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum ErrorKind {
    /// Syntax error
    SyntaxError(String),
    /// Invalid credentials
    InvalidCredentials(String),
    /// The specified database does not exist
    DataBaseDoesNotExist(String),
    /// The specified retention policy does not exist
    RetentionPolicyDoesNotExist(String),
    /// Some error on build url or io.
    Communication(String),
    /// Some other error, I don't expect
    Unknown(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::SyntaxError(ref t) => write!(f, "{}", t),
            ErrorKind::InvalidCredentials(ref t) => write!(f, "{}", t),
            ErrorKind::DataBaseDoesNotExist(ref t) => write!(f, "{}", t),
            ErrorKind::RetentionPolicyDoesNotExist(ref t) => write!(f, "{}", t),
            ErrorKind::Communication(ref t) => write!(f, "{}", t),
            ErrorKind::Unknown(ref t) => write!(f, "{}", t),
        }
    }
}

impl std::error::Error for Error {}

impl std::error::Error for ErrorKind {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error{
            inner: ErrorKind::Communication(format!("{}", err)),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error {
            inner: ErrorKind::Communication(format!("{}", err))
        }
    }
}

 // This is our `Future` implementation
impl Future for Error {
    type Output = String;

      // Poll is the what drives the state machine forward and it's the only
    // method we'll need to call to drive futures to completion.
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {

        match self.inner {
            ErrorKind::SyntaxError(ref t) => Poll::Ready(format!("{}", t).to_string()),
            ErrorKind::InvalidCredentials(ref t) => Poll::Ready(format!("{}", t).to_string()),
            ErrorKind::DataBaseDoesNotExist(ref t) => Poll::Ready(format!("{}", t).to_string()),
            ErrorKind::RetentionPolicyDoesNotExist(ref t) => Poll::Ready(format!("{}", t).to_string()),
            ErrorKind::Communication(ref t) => Poll::Ready(format!("{}", t).to_string()),
            ErrorKind::Unknown(ref t) => Poll::Ready(format!("{}", t).to_string()),
        }        
    }

}

#[cfg(test)]
mod tests {

    use crate::serialization;
    use crate::{Error, error::ErrorKind};

    #[test]
    fn syntax_error() {
        let syntax = Error {
            inner: ErrorKind::SyntaxError(serialization::conversion(
            "BAD SYNTAX",)),
        };

        match syntax.inner {
            ErrorKind::SyntaxError(ref e) => {
                assert_eq!(e.as_str(), "BAD SYNTAX")
            },
            _ => panic!("WAS NOT SYNTAX ERROR"),
        }


    }
}