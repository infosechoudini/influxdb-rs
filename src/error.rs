use serde::{Deserialize, Serialize};
use std::fmt;
use std::io;
use futures::Future;
use std::pin::Pin;
use futures::task::{Poll, Context};

/// The error of influxdb client
#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum Error {
    /// Syntax error, some is bug, some is SQL error. If it's a bug, welcome to PR.
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::SyntaxError(ref t) => write!(f, "{}", t),
            Error::InvalidCredentials(ref t) => write!(f, "{}", t),
            Error::DataBaseDoesNotExist(ref t) => write!(f, "{}", t),
            Error::RetentionPolicyDoesNotExist(ref t) => write!(f, "{}", t),
            Error::Communication(ref t) => write!(f, "{}", t),
            Error::Unknown(ref t) => write!(f, "{}", t),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Communication(format!("{}", err))
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Communication(format!("{}", err))
    }
}

 // This is our `Future` implementation
impl Future for Error {
    type Output = String;

      // Poll is the what drives the state machine forward and it's the only
    // method we'll need to call to drive futures to completion.
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {

        match *self {
            Error::SyntaxError(ref t) => Poll::Ready(format!("{}", t).to_string()),
            Error::InvalidCredentials(ref t) => Poll::Ready(format!("{}", t).to_string()),
            Error::DataBaseDoesNotExist(ref t) => Poll::Ready(format!("{}", t).to_string()),
            Error::RetentionPolicyDoesNotExist(ref t) => Poll::Ready(format!("{}", t).to_string()),
            Error::Communication(ref t) => Poll::Ready(format!("{}", t).to_string()),
            Error::Unknown(ref t) => Poll::Ready(format!("{}", t).to_string()),
        }        
    }

}