extern crate reqwest;

use core::*;

pub struct CurrencyPairs {
    name: String,
}

impl CurrencyPairs {
    pub fn exec(&self) -> reqwest::Result<String> {
        let api = ApiBuilder::new()
            .uri(format!("https://api.zaif.jp/api/1/currency_pairs/{}", self.name).as_str())
            .finalize();

        api.exec()
    }
}

pub struct CurrencyPairsBuilder {
    name: String,
}

impl CurrencyPairsBuilder {
    pub fn new() -> CurrencyPairsBuilder {
        CurrencyPairsBuilder {
            name: "all".to_string(),
        }
    }
    pub fn name(&mut self, name: &str) -> &mut CurrencyPairsBuilder {
        self.name = name.to_string();
        self
    }
    pub fn finalize(&self) -> CurrencyPairs {
        CurrencyPairs {
            name: self.name.clone(),
        }
    }
}
