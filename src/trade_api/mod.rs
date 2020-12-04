extern crate reqwest;
extern crate serde_json;

use std::collections::HashMap;
use self::serde_json::Value;

use crate::core::*;

pub use self::get_info2::*;
pub use self::get_personal_info::*;
pub use self::get_id_info::*;
pub use self::trade::*;
pub use self::active_orders::*;
pub use self::cancel_order::*;
pub use self::trade_history::*;

mod get_info2;
mod get_personal_info;
mod get_id_info;
mod trade;
mod active_orders;
mod cancel_order;
mod trade_history;

trait TradeApi {
    fn method(&self) -> &str;
    fn parameters(&self) -> HashMap<String, String>;
    fn access_key(&self) -> &AccessKey;

    fn exec(&self) -> crate::Result<Value> {
        let mut param = self.parameters().clone();
        param.insert("method".to_string(), self.method().to_string());

        let api = ApiBuilder::new()
            .access_key(self.access_key().clone())
            .uri("https://api.zaif.jp/tapi")
            .method(Method::Post)
            .param(param)
            .finalize();

        let res = api.exec()?;
        let result: Value = serde_json::from_str(res.as_str())?;
        if result["success"].as_i64() != Some(1) {
            return Err(format!("error: {}", result["error"]).into());
        }
        Ok(result["return"].clone())
    }
}
