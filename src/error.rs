extern crate reqwest;
extern crate serde_json;

use std::error::Error as StdError;

#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

#[derive(Debug)]
enum Kind {
    Network(reqwest::Error),
    Json(serde_json::Error),
}
