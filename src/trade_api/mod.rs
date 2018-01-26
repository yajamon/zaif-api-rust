extern crate reqwest;
extern crate serde_json;

use std::collections::HashMap;
use self::serde_json::Value;

use core::*;

pub use self::get_info2::*;
pub use self::trade::*;
pub use self::active_orders::*;
pub use self::cancel_order::*;

mod get_info2;
mod trade;
mod active_orders;
mod cancel_order;

trait TradeApi {
    fn method(&self) -> &str;
    fn parameters(&self) -> &HashMap<String, String>;
    fn access_key(&self) -> &AccessKey;

    fn exec(&self) -> serde_json::Result<Value> {
        let mut param = self.parameters().clone();
        param.insert("method".to_string(), self.method().to_string());

        let api = ApiBuilder::new()
            .access_key(self.access_key().clone())
            .uri("https://api.zaif.jp/tapi")
            .method(Method::Post)
            .param(param)
            .finalize();

        let res = match api.exec() {
            Ok(res) => res,
            Err(e) => panic!("reqwest Error: {}", e),
        };
        let result: Value = serde_json::from_str(res.as_str())?;
        if result["success"].as_i64().unwrap() != 1 {
            panic!("error: {}", result["error"]);
        }
        Ok(result["return"].clone())
    }
}
