extern crate reqwest;

use core::*;

pub struct LastPrice {
    currency_pair: String,
}

impl LastPrice {
    pub fn exec(&self) -> reqwest::Result<String> {
        let api = ApiBuilder::new()
            .uri(
                format!(
                    "https://api.zaif.jp/api/1/last_price/{}",
                    self.currency_pair
                ).as_str(),
            )
            .finalize();

        api.exec()
    }
}

pub struct LastPriceBuilder {
    currency_pair: String,
}

impl LastPriceBuilder {
    pub fn new() -> LastPriceBuilder {
        LastPriceBuilder { currency_pair: "btc_jpy".to_string() }
    }
    pub fn currency_pair(&mut self, currency_pair: &str) -> &mut LastPriceBuilder {
        self.currency_pair = currency_pair.to_string();
        self
    }
    pub fn finalize(&self) -> LastPrice {
        LastPrice { currency_pair: self.currency_pair.clone() }
    }
}
