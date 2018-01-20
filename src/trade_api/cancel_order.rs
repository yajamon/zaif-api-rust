extern crate reqwest;

use std::collections::HashMap;

use core::*;

builder!(CancelOrderBuilder => CancelOrder {
    access_key: AccessKey = AccessKey::new("", ""),
    order_id: u64 = 0,
    currency_pair: Option<String> = None
});

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

