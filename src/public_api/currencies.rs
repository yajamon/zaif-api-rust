extern crate reqwest;

use core::*;

pub struct Currencies {
    name: String,
}

impl Currencies {
    pub fn exec(&self) -> reqwest::Result<String> {
        let api = ApiBuilder::new()
            .uri(format!("https://api.zaif.jp/api/1/currencies/{}", self.name).as_str())
            .finalize();

        api.exec()
    }
}

pub struct CurrenciesBuilder {
    name: String,
}

impl CurrenciesBuilder {
    pub fn new() -> CurrenciesBuilder {
        CurrenciesBuilder {
            name: "all".to_string(),
        }
    }
    pub fn name(&mut self, name: &str) -> &mut CurrenciesBuilder {
        self.name = name.to_string();
        self
    }
    pub fn finalize(&self) -> Currencies {
        Currencies {
            name: self.name.clone(),
        }
    }
}
