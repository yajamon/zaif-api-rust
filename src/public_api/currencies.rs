//! #Examples
//!
//! ```
//! let api = CurrenciesBuilder::new().name("btc".to_string()).finalize();
//! for currency in api.exec().unwrap() {
//!     println!("name: {} is_token: {}", currency.name, currency.is_token);
//! }
//! ```
//!

use serde_json;

use crate::public_api::PublicApi;

builder!(CurrenciesBuilder => Currencies {
    name: String = "all".to_string()
});

impl Currencies {
    pub fn exec(&self) -> crate::Result<Vec<CurrenciesResponse>> {
        Ok(serde_json::from_value(<Self as PublicApi>::exec(&self)?)?)
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
