extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

use trade_api::TradeApi;
use core::AccessKey;

#[derive(Copy, Clone)]
pub enum TradeHistoryOrder {
    ASC,
    DESC,
}
impl TradeHistoryOrder {
    fn param_string(&self) -> String {
        match *self {
            TradeHistoryOrder::ASC => "ASC".to_string(),
            TradeHistoryOrder::DESC => "DESC".to_string(),
        }
    }
}

builder!(TradeHistoryBuilder => TradeHistory {
    access_key: AccessKey = AccessKey::new("", ""),
    from: Option<u64> = None,
    count: Option<u64> = None,
    from_id: Option<u64> = None,
    end_id: Option<u64> = None,
    order: Option<TradeHistoryOrder> = None,
    since: Option<String> = None,
    end: Option<String> = None,
    currency_pair: Option<String> = None
});

impl TradeHistory {
    pub fn exec(&self) -> ::Result<HashMap<u64, TradeHistoryResponse>> {
        Ok(serde_json::from_value(<Self as TradeApi>::exec(&self)?)?)
    }
}

impl TradeApi for TradeHistory {
    fn method(&self) -> &str {
        "trade_history"
    }
    fn parameters(&self) -> HashMap<String, String> {
        let mut param = HashMap::new();
        if let Some(from) = self.from {
            param.insert("from".to_string(), format!("{}", from));
        }
        if let Some(count) = self.count {
            param.insert("count".to_string(), format!("{}", count));
        }
        if let Some(from_id) = self.from_id {
            param.insert("from_id".to_string(), format!("{}", from_id));
        }
        if let Some(end_id) = self.end_id {
            param.insert("end_id".to_string(), format!("{}", end_id));
        }
        if let Some(order) = self.order {
            param.insert("order".to_string(), order.param_string());
        }
        if let Some(ref since) = self.since {
            param.insert("since".to_string(), format!("{}", since));
        }
        if let Some(ref end) = self.end {
            param.insert("end".to_string(), format!("{}", end));
        }
        if let Some(ref currency_pair) = self.currency_pair {
            param.insert("currency_pair".to_string(), format!("{}", currency_pair));
        }

        param
    }
    fn access_key(&self) -> &AccessKey {
        &self.access_key
    }
}

#[derive(Deserialize)]
pub struct TradeHistoryResponse {
    pub currency_pair: String,
    pub action: String,
    pub amount: f64,
    pub price: f64,
    pub fee: f64,
    pub your_action: String,
    pub bonus: Option<f64>,
    pub timestamp: String,
    pub comment: String,
}
