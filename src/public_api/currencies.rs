extern crate reqwest;

use std::io::{self, Write};

pub struct Currencies {
}

impl Currencies {
    pub fn get(&self, name: &str) {

        let uri = format!("https://api.zaif.jp/api/1/currencies/{}", name);
        let mut resp = reqwest::get(uri.as_str()).unwrap();

        assert!(resp.status().is_success());
        let body = resp.text().unwrap();

        io::stdout().write(body.as_bytes()).unwrap();
    }
}
