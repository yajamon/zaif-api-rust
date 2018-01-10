extern crate reqwest;

use std::collections::HashMap;

use core::*;

#[derive(Copy, Clone)]
pub enum TradeAction {
    Bid, // 買い
    Ask, // 売り
}
impl TradeAction {
    fn param_string
        (&self) -> String {
        match *self {
            TradeAction::Bid => "bid".to_string(),
            TradeAction::Ask => "ask".to_string(),
        }
    }
}

pub struct Trade {
    access_key: AccessKey,
    currency_pair: String,
    action: TradeAction,
    price: f32,
    amount: f32,
}

impl Trade {
    pub fn exec(&self) -> reqwest::Result<String> {
        let param: &mut HashMap<String, String> = &mut HashMap::new();
        param.insert("method".to_string(), "trade".to_string());
        param.insert("currency_pair".to_string(), self.currency_pair.clone());
        param.insert("action".to_string(), self.action.param_string());
        param.insert("price".to_string(), format!("{}", self.price));
        param.insert("amount".to_string(), format!("{}", self.amount));

        let api = ApiBuilder::new()
            .access_key(self.access_key.clone())
            .uri("https://api.zaif.jp/tapi")
            .method(Method::Post)
            .param(param.clone())
            .finalize();

        api.exec()
    }
}

pub struct TradeBuilder {
    access_key: AccessKey,
    currency_pair: Option<String>,
    action: Option<TradeAction>,
    price: Option<f32>,
    amount: Option<f32>,
}

impl TradeBuilder{
    pub fn new(access_key: AccessKey) -> TradeBuilder {
        TradeBuilder {
            access_key: access_key,
            currency_pair: None,
            action: None,
            price: None,
            amount: None,
        }
    }
    pub fn currency_pair(&mut self, currency_pair: &str) -> &mut TradeBuilder {
        self.currency_pair = Some(currency_pair.to_string());
        self
    }
    pub fn action(&mut self, action: TradeAction) -> &mut TradeBuilder {
        self.action = Some(action);
        self
    }
    pub fn price(&mut self, price: f32) -> &mut TradeBuilder {
        self.price = Some(price);
        self
    }
    pub fn amount(&mut self, amount: f32) -> &mut TradeBuilder {
        self.amount = Some(amount);
        self
    }
    pub fn finalize(&self) -> Trade {
        Trade {
            access_key: self.access_key.clone(),
            currency_pair: self.currency_pair.clone().unwrap().clone(),
            action: self.action.unwrap(),
            price: self.price.unwrap(),
            amount: self.amount.unwrap(),
        }
    }
}

