extern crate reqwest;

use core::*;

pub struct Currencies {}

impl Currencies {
    pub fn get(&self, name: &str) -> reqwest::Result<String> {
        self.exec(name)
    }

    pub fn exec(&self, name: &str) -> reqwest::Result<String> {
        let api = ApiBuilder::new()
            .uri(format!("https://api.zaif.jp/api/1/currencies/{}", name).as_str())
            .finalize();

        api.exec()
    }
}
