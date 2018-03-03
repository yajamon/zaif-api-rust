extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

use trade_api::TradeApi;
use core::AccessKey;

builder!(ActiveOrdersBuilder => ActiveOrders {
    access_key: AccessKey = AccessKey::new("", ""),
    currency_pair: Option<String> = None
});

impl ActiveOrders {
    pub fn exec(&self) -> ::Result<HashMap<u64, ActiveOrdersResponse>> {
        Ok(serde_json::from_value(<Self as TradeApi>::exec(&self)?)?)
    }
}

impl TradeApi for ActiveOrders {
    fn method(&self) -> &str {
        "active_orders"
    }
    fn parameters(&self) -> HashMap<String, String> {
        let mut param = HashMap::new();
        if let Some(ref currency_pair) = self.currency_pair {
            param.insert(
                "currency_pair".to_string(),
                format!("{}", currency_pair.clone()),
            );
        }
        param
    }
    fn access_key(&self) -> &AccessKey {
        &self.access_key
    }
}

#[derive(Deserialize)]
pub struct ActiveOrdersResponse {
    pub currency_pair: String,
    pub action: String,
    pub amount: f64,
    pub price: f64,
    pub timestamp: String,
    pub comment: String,
}
