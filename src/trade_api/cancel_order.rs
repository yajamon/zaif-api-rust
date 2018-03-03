extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

use trade_api::TradeApi;
use core::AccessKey;

builder!(CancelOrderBuilder => CancelOrder {
    access_key: AccessKey = AccessKey::new("", ""),
    order_id: u64 = 0,
    currency_pair: Option<String> = None
});

impl CancelOrder {
    pub fn exec(&self) -> ::Result<CancelOrderResponse> {
        Ok(serde_json::from_value(<Self as TradeApi>::exec(&self)?)?)
    }
}

impl TradeApi for CancelOrder {
    fn method(&self) -> &str {
        "cancel_order"
    }
    fn parameters(&self) -> HashMap<String, String> {
        let mut param = HashMap::new();
        param.insert("order_id".to_string(), format!("{}", self.order_id));
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
pub struct CancelOrderResponse {
    pub order_id: u64,
    pub funds: HashMap<String, f64>,
}
