extern crate reqwest;

use std::collections::HashMap;

use core::*;

builder!(GetInfo2Builder => GetInfo2 {
    access_key: AccessKey = AccessKey::new("", "")
});

impl GetInfo2 {
    pub fn new(access_key: AccessKey) -> GetInfo2 {
        GetInfo2 { access_key: access_key }
    }
    pub fn exec(&self) -> reqwest::Result<String> {
        let param: &mut HashMap<String, String> = &mut HashMap::new();
        param.insert("method".to_string(), "get_info2".to_string());

        let api = ApiBuilder::new()
            .access_key(self.access_key.clone())
            .uri("https://api.zaif.jp/tapi")
            .method(Method::Post)
            .param(param.clone())
            .finalize();

        api.exec()
    }
}

