extern crate reqwest;

use core::*;

pub use self::currencies::*;
pub use self::currency_pairs::*;
pub use self::last_price::*;
pub use self::depth::*;

mod currencies;
mod currency_pairs;
mod last_price;
mod depth;

trait PublicApi {
    fn action(&self) -> &str;
    fn parameter(&self) -> &str;
    fn exec(&self) -> reqwest::Result<String> {
        let endpoint = "https://api.zaif.jp/api/1";
        let api = ApiBuilder::new()
            .uri(
                format!("{}/{}/{}", endpoint, self.action(), self.parameter()).as_str(),
            )
            .finalize();

        api.exec()
    }
}
