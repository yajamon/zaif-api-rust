extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

use trade_api::TradeApi;
use core::AccessKey;

builder!(GetInfo2Builder => GetInfo2 {
    access_key: AccessKey = AccessKey::new("", "")
});

impl GetInfo2 {
    pub fn exec(&self) -> serde_json::Result<GetInfo2Response> {
        serde_json::from_value(<Self as TradeApi>::exec(&self)?)
    }
}

impl TradeApi for GetInfo2 {
    fn method(&self) -> &str {
        "get_info2"
    }
    fn parameters(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    fn access_key(&self) -> &AccessKey {
        &self.access_key
    }
}

#[derive(Deserialize)]
pub struct GetInfo2Response {
    pub funds: HashMap<String, f64>,
    pub deposit: HashMap<String, f64>,
    pub rights: HashMap<String, bool>,
    pub open_orders: i64,
    pub server_time: i64,
}
