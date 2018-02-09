extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

use trade_api::TradeApi;
use core::AccessKey;

builder!(GetPersonalInfoBuilder => GetPersonalInfo {
    access_key: AccessKey = AccessKey::new("", "")
});

impl GetPersonalInfo {
    pub fn exec(&self) -> serde_json::Result<GetPersonalInfoResponse> {
        serde_json::from_value(<Self as TradeApi>::exec(&self)?)
    }
}

impl TradeApi for GetPersonalInfo {
    fn method(&self) -> &str {
        "get_personal_info"
    }
    fn parameters(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    fn access_key(&self) -> &AccessKey {
        &self.access_key
    }
}

#[derive(Deserialize)]
pub struct GetPersonalInfoResponse {
    pub ranking_nickname: String,
    pub icon_path: String,
}
