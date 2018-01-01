extern crate reqwest;

use core::*;

pub struct CurrencyPairs;

impl CurrencyPairs {
    pub fn get(&self, name: &str) -> reqwest::Result<String> {
        let api = ApiBuilder::new()
            .uri(format!("https://api.zaif.jp/api/1/currency_pairs/{}", name).as_str())
            .finalize();

        api.exec()
    }
}
