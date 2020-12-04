
use serde_json;

use crate::public_api::PublicApi;

builder!(LastPriceBuilder => LastPrice {
    currency_pair: String = "btc_jpy".to_string()
});

impl LastPrice {
    pub fn exec(&self) -> crate::Result<LastPriceResponse> {
        Ok(serde_json::from_value(<Self as PublicApi>::exec(&self)?)?)
    }
}

impl PublicApi for LastPrice {
    fn action(&self) -> &str {
        "last_price"
    }
    fn parameter(&self) -> &str {
        self.currency_pair.as_str()
    }
}

#[derive(Deserialize)]
pub struct LastPriceResponse {
    pub last_price: f64,
}
