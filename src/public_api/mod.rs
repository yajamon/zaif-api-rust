extern crate reqwest;
extern crate serde_json;

use self::serde_json::Value;

use core::*;

pub use self::currencies::*;
pub use self::currency_pairs::*;
pub use self::last_price::*;
pub use self::depth::*;
pub use self::trades::*;
pub use self::ticker::*;

mod currencies;
mod currency_pairs;
mod last_price;
mod depth;
mod trades;
mod ticker;

trait PublicApi {
    fn action(&self) -> &str;
    fn parameter(&self) -> &str;
    fn exec(&self) -> serde_json::Result<Value> {
        let endpoint = "https://api.zaif.jp/api/1";
        let api = ApiBuilder::new()
            .uri(
                format!("{}/{}/{}", endpoint, self.action(), self.parameter()).as_str(),
            )
            .finalize();

        let res = match api.exec() {
            Ok(res) => res,
            Err(e) => panic!("reqwest Error: {}", e),
        };
        serde_json::from_str(res.as_str())
    }
}
