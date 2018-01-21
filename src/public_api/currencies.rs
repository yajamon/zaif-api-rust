extern crate serde;
extern crate serde_json;

use self::serde_json::Value;

use public_api::PublicApi;

builder!(CurrenciesBuilder => Currencies {
    name: String = "all".to_string()
});

impl Currencies {
    pub fn exec(&self) -> serde_json::Result<Vec<CurrenciesResponse>> {
        serde_json::from_value(<Self as PublicApi>::exec(&self)?)
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

#[derive(Deserialize)]
pub struct CurrenciesResponse {
    pub name: String,
    pub is_token: bool,
}
