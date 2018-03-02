extern crate reqwest;
extern crate serde_json;

use std::error::Error as StdError;

#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
enum Kind {
    Network(reqwest::Error),
    Json(serde_json::Error),
}
