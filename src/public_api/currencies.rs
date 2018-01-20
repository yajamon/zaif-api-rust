extern crate reqwest;

use core::*;

builder!(CurrenciesBuilder => Currencies {
    name: String = "all".to_string()
});

impl Currencies {
    pub fn exec(&self) -> reqwest::Result<String> {
        let api = ApiBuilder::new()
            .uri(
                format!("https://api.zaif.jp/api/1/currencies/{}", self.name).as_str(),
            )
            .finalize();

        api.exec()
    }
}
