extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

use trade_api::TradeApi;
use core::AccessKey;

builder!(TradeHistoryBuilder => TradeHistory {
    access_key: AccessKey = AccessKey::new("", "")
});

impl TradeHistory {
    pub fn exec(&self) -> serde_json::Result<TradeHistoryResponse> {
        serde_json::from_value(<Self as TradeApi>::exec(&self)?)
    }
}

impl TradeApi for TradeHistory {
    fn method(&self) -> &str {
        "trade_history"
    }
    fn parameters(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    fn access_key(&self) -> &AccessKey {
        &self.access_key
    }
}

#[derive(Deserialize)]
pub struct TradeHistoryResponse {
}
