extern crate reqwest;
extern crate serde_json;

use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeJsonError(error)
    }
}
