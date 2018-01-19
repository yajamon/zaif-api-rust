extern crate reqwest;

use std::collections::HashMap;

use core::*;

pub struct CancelOrder {
    access_key: AccessKey,
    order_id: u64,
    currency_pair: Option<String>,
}

impl CancelOrder {
    pub fn exec(&self) -> reqwest::Result<String> {
        let param: &mut HashMap<String, String> = &mut HashMap::new();
        param.insert("method".to_string(), "cancel_order".to_string());
        param.insert("order_id".to_string(), format!("{}", self.order_id));
        if let Some(ref currency_pair) = self.currency_pair {
            param.insert(
                "currency_pair".to_string(),
                format!("{}", currency_pair.clone()),
            );
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

pub struct CancelOrderBuilder {
    access_key: AccessKey,
    order_id: Option<u64>,
    currency_pair: Option<String>,
}

impl CancelOrderBuilder {
    pub fn new(access_key: AccessKey) -> CancelOrderBuilder {
        CancelOrderBuilder {
            access_key: access_key,
            order_id: None,
            currency_pair: None,
        }
    }
    pub fn order_id(&mut self, order_id: u64) -> &mut CancelOrderBuilder {
        self.order_id = Some(order_id);
        self
    }
    pub fn currency_pair(&mut self, currency_pair: &str) -> &mut CancelOrderBuilder {
        self.currency_pair = Some(currency_pair.to_string());
        self
    }
    pub fn finalize(&self) -> CancelOrder {
        CancelOrder {
            access_key: self.access_key.clone(),
            order_id: self.order_id.unwrap(),
            currency_pair: self.currency_pair.clone(),
        }
    }
}
