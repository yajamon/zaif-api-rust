use serde_json;

use crate::public_api::PublicApi;

builder!(DepthBuilder => Depth {
    currency_pair: String = "btc_jpy".to_string()
});

impl Depth {
    pub fn exec(&self) -> crate::Result<DepthResponse> {
        Ok(serde_json::from_value(<Self as PublicApi>::exec(&self)?)?)
    }
}

impl PublicApi for Depth {
    fn action(&self) -> &str {
        "depth"
    }
    fn parameter(&self) -> &str {
        self.currency_pair.as_str()
    }
}

#[derive(Deserialize)]
pub struct DepthItem(f64, f64);
impl DepthItem {
    pub fn price(&self) -> f64 {
        self.0
    }
    pub fn amount(&self) -> f64 {
        self.1
    }
}

#[derive(Deserialize)]
pub struct DepthResponse {
    pub asks: Vec<DepthItem>,
    pub bids: Vec<DepthItem>,
}
