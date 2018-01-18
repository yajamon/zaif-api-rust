extern crate reqwest;

use public_api::PublicApi;

pub struct Currencies {
    name: String,
}

impl Currencies {
    pub fn exec(&self) -> reqwest::Result<String> {
        <Self as PublicApi>::exec(&self)
    }
}

impl PublicApi for Currencies {
    fn action(&self) -> &str {
        "currencies"
    }
    fn parameter(&self) -> &str {
        self.name.as_str()
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
