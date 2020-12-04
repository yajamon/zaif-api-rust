extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

use crate::trade_api::TradeApi;
use crate::core::AccessKey;

#[derive(Copy, Clone)]
pub enum TradeAction {
    None,
    Bid, // 買い
    Ask, // 売り
}
impl TradeAction {
    fn param_string(&self) -> String {
        match *self {
            TradeAction::Bid => "bid".to_string(),
            TradeAction::Ask => "ask".to_string(),
            _ => "".to_string(),
        }
    }
}

builder!(TradeBuilder => Trade {
    access_key: AccessKey = AccessKey::new("", ""),
    currency_pair: String = "".to_string(),
    action: TradeAction = TradeAction::None,
    price: f32 = 0.0,
    amount: f32 = 0.0,
    limit: Option<f32> = None,
    comment: Option<String> = None
});

impl Trade {
    pub fn exec(&self) -> crate::Result<TradeResponse> {
        Ok(serde_json::from_value(<Self as TradeApi>::exec(&self)?)?)
    }
}

impl TradeApi for Trade {
    fn method(&self) -> &str {
        "trade"
    }
    fn parameters(&self) -> HashMap<String, String> {
        let mut param = HashMap::new();
        param.insert("currency_pair".to_string(), self.currency_pair.clone());
        param.insert("action".to_string(), self.action.param_string());
        param.insert("price".to_string(), format!("{}", self.price));
        param.insert("amount".to_string(), format!("{}", self.amount));
        if let Some(limit) = self.limit {
            param.insert("limit".to_string(), format!("{}", limit));
        }
        if let Some(ref comment) = self.comment {
            param.insert("comment".to_string(), format!("{}", comment.clone()));
        }
        param
    }
    fn access_key(&self) -> &AccessKey {
        &self.access_key
    }
}

#[derive(Deserialize)]
pub struct TradeResponse {
    pub received: f64,
    pub remains: f64,
    pub order_id: u64,
    pub funds: HashMap<String, f64>,
}
