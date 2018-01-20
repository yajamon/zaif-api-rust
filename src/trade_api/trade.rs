extern crate reqwest;

use std::collections::HashMap;

use core::*;

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
    pub fn exec(&self) -> reqwest::Result<String> {
        let param: &mut HashMap<String, String> = &mut HashMap::new();
        param.insert("method".to_string(), "trade".to_string());
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

        let api = ApiBuilder::new()
            .access_key(self.access_key.clone())
            .uri("https://api.zaif.jp/tapi")
            .method(Method::Post)
            .param(param.clone())
            .finalize();

        api.exec()
    }
}

