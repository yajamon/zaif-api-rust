extern crate reqwest;

use std::collections::HashMap;

use core::*;

pub struct Trade {
    access_key: AccessKey,
}

impl Trade {
    pub fn exec(&self) -> reqwest::Result<String> {
        let param: &mut HashMap<String, String> = &mut HashMap::new();
        param.insert("method".to_string(), "trade".to_string());

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
    access_key: AccessKey
}

impl TradeBuilder{
    pub fn new(access_key: AccessKey) -> TradeBuilder {
        TradeBuilder {
            access_key: access_key,
        }
    }
    pub fn finalize(&self) -> Trade {
        Trade {
            access_key: self.access_key.clone(),
        }
    }
}

