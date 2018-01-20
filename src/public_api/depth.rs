extern crate reqwest;

use core::*;

builder!(DepthBuilder => Depth {
    currency_pair: String = "btc_jpy".to_string()
});

impl Depth {
    pub fn exec(&self) -> reqwest::Result<String> {
        let api = ApiBuilder::new()
            .uri(
                format!("https://api.zaif.jp/api/1/depth/{}", self.currency_pair).as_str(),
            )
            .finalize();

        api.exec()
    }
}

