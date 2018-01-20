extern crate reqwest;

use core::*;

builder!(CurrencyPairsBuilder => CurrencyPairs {
    currency_pair: String = "all".to_string()
});

impl CurrencyPairs {
    pub fn exec(&self) -> reqwest::Result<String> {
        let api = ApiBuilder::new()
            .uri(
                format!(
                    "https://api.zaif.jp/api/1/currency_pairs/{}",
                    self.currency_pair
                ).as_str(),
            )
            .finalize();

        api.exec()
    }
}

