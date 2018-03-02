extern crate reqwest;
extern crate serde_json;

use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;
