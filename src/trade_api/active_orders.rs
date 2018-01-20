extern crate reqwest;

use std::collections::HashMap;

use core::*;

builder!(ActiveOrdersBuilder => ActiveOrders {
    access_key: AccessKey = AccessKey::new("", ""),
    currency_pair: Option<String> = None
});

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

