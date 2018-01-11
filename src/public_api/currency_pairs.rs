extern crate reqwest;

use core::*;

pub struct CurrencyPairs {
    currency_pair: String,
}

impl CurrencyPairs {
    pub fn exec(&self) -> reqwest::Result<String> {
        let api = ApiBuilder::new()
            .uri(format!("https://api.zaif.jp/api/1/currency_pairs/{}", self.currency_pair).as_str())
            .finalize();

        api.exec()
    }
}

pub struct CurrencyPairsBuilder {
    currency_pair: String,
}

impl CurrencyPairsBuilder {
    pub fn new() -> CurrencyPairsBuilder {
        CurrencyPairsBuilder {
            currency_pair: "all".to_string(),
        }
    }
    pub fn currency_pair(&mut self, currency_pair: &str) -> &mut CurrencyPairsBuilder {
        self.currency_pair = currency_pair.to_string();
        self
    }
    pub fn finalize(&self) -> CurrencyPairs {
        CurrencyPairs {
            currency_pair: self.currency_pair.clone(),
        }
    }
}
