extern crate reqwest;

use public_api::PublicApi;

builder!(CurrenciesBuilder => Currencies {
    name: String = "all".to_string()
});

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
