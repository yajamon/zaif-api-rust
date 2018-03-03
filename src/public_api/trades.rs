extern crate serde;
extern crate serde_json;

use public_api::PublicApi;

builder!(TradesBuilder => Trades {
    currency_pair: String = "all".to_string()
});

impl Trades {
    pub fn exec(&self) -> ::Result<Vec<TradesResponse>> {
        Ok(serde_json::from_value(<Self as PublicApi>::exec(&self)?)?)
    }
}

impl PublicApi for Trades {
    fn action(&self) -> &str {
        "trades"
    }
    fn parameter(&self) -> &str {
        self.currency_pair.as_str()
    }
}

#[derive(Deserialize)]
pub struct TradesResponse {
    pub date: u64,
    pub price: f64,
    pub amount: f64,
    pub tid: u64,
    pub currency_pair: String,
    pub trade_type: String,
}
