extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

use crate::trade_api::TradeApi;
use crate::core::AccessKey;

builder!(GetPersonalInfoBuilder => GetPersonalInfo {
    access_key: AccessKey = AccessKey::new("", "")
});

impl GetPersonalInfo {
    pub fn exec(&self) -> crate::Result<GetPersonalInfoResponse> {
        Ok(serde_json::from_value(<Self as TradeApi>::exec(&self)?)?)
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
