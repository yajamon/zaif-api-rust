extern crate reqwest;

use core::*;

builder!(LastPriceBuilder => LastPrice {
    currency_pair: String = "btc_jpy".to_string()
});

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

