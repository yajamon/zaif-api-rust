
use serde_json;

use crate::public_api::PublicApi;

builder!(TickerBuilder => Ticker {
    currency_pair: String = "btc_jpy".to_string()
});

impl Ticker {
    pub fn exec(&self) -> crate::Result<TickerResponse> {
        Ok(serde_json::from_value(<Self as PublicApi>::exec(&self)?)?)
    }
}

impl PublicApi for Ticker {
    fn action(&self) -> &str {
        "ticker"
    }
    fn parameter(&self) -> &str {
        self.currency_pair.as_str()
    }
}

#[derive(Deserialize)]
pub struct TickerResponse {
    pub last: f64,
    pub high: f64,
    pub low: f64,
    pub vwap: f64,
    pub volume: f64,
    pub bid: f64,
    pub ask: f64,
}
