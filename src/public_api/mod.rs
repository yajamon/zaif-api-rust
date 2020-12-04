use serde_json;

use self::serde_json::Value;

use crate::core::*;

pub use self::currencies::*;
pub use self::currency_pairs::*;
pub use self::depth::*;
pub use self::last_price::*;
pub use self::ticker::*;
pub use self::trades::*;

mod currencies;
mod currency_pairs;
mod depth;
mod last_price;
mod ticker;
mod trades;

trait PublicApi {
    fn action(&self) -> &str;
    fn parameter(&self) -> &str;
    fn exec(&self) -> crate::Result<Value> {
        let endpoint = "https://api.zaif.jp/api/1";
        let api = ApiBuilder::new()
            .uri(format!("{}/{}/{}", endpoint, self.action(), self.parameter()).as_str())
            .finalize();

        let res = api.exec()?;
        Ok(serde_json::from_str(res.as_str())?)
    }
}
