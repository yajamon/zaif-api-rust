extern crate reqwest;

use std::collections::HashMap;

use core::*;

pub struct ActiveOrders {
    access_key: AccessKey,
    currency_pair: Option<String>,
}

impl ActiveOrders {
    pub fn exec(&self) -> reqwest::Result<String> {
        let param: &mut HashMap<String, String> = &mut HashMap::new();
        param.insert("method".to_string(), "active_orders".to_string());
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

pub struct ActiveOrdersBuilder {
    access_key: AccessKey,
    currency_pair: Option<String>,
}

impl ActiveOrdersBuilder {
    pub fn new(access_key: AccessKey) -> ActiveOrdersBuilder {
        ActiveOrdersBuilder {
            access_key: access_key,
            currency_pair: None,
        }
    }
    pub fn currency_pair(&mut self, currency_pair: &str) -> &mut ActiveOrdersBuilder {
        self.currency_pair = Some(currency_pair.to_string());
        self
    }
    pub fn finalize(&self) -> ActiveOrders {
        ActiveOrders {
            access_key: self.access_key.clone(),
            currency_pair: self.currency_pair.clone(),
        }
    }
}
