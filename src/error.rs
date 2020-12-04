use reqwest;
use serde_json;



#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    SimpleError(String),
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
impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::SimpleError(error)
    }
}
