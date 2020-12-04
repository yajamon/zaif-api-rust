extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

use crate::trade_api::TradeApi;
use crate::core::AccessKey;

builder!(GetIdInfoBuilder => GetIdInfo {
    access_key: AccessKey = AccessKey::new("", "")
});

impl GetIdInfo {
    pub fn exec(&self) -> crate::Result<GetIdInfoResponse> {
        let result = <Self as TradeApi>::exec(&self)?;
        Ok(serde_json::from_value(result["user"].clone())?)
    }
}

impl TradeApi for GetIdInfo {
    fn method(&self) -> &str {
        "get_id_info"
    }
    fn parameters(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    fn access_key(&self) -> &AccessKey {
        &self.access_key
    }
}

#[derive(Deserialize)]
pub struct GetIdInfoResponse {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub kana: String,
    pub certified: bool,
}
