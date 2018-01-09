extern crate reqwest;

use core::*;

pub struct Depth {
    currency_pair: String,
}

impl Depth {
    pub fn exec(&self) -> reqwest::Result<String> {
        let api = ApiBuilder::new()
            .uri(
                format!(
                    "https://api.zaif.jp/api/1/depth/{}",
                    self.currency_pair
                ).as_str(),
            )
            .finalize();

        api.exec()
    }
}

pub struct DepthBuilder {
    currency_pair: String,
}

impl DepthBuilder {
    pub fn new() -> DepthBuilder {
        DepthBuilder {
            currency_pair: "btc_jpy".to_string(),
        }
    }
    pub fn currency_pair(&mut self, currency_pair: &str) -> &mut DepthBuilder {
        self.currency_pair = currency_pair.to_string();
        self
    }
    pub fn finalize(&self) -> Depth {
        Depth {
            currency_pair: self.currency_pair.clone(),
        }
    }
}
