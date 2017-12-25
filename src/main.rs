extern crate zaif_api;

use zaif_api::public_api::currencies::Currencies;
// use zaif_api::trade_api::get_info2::GetInfo2;

fn main() {
    let api = Currencies{};
    api.get("btc");

    // let api = GetInfo2::new("YOUR_API_KEY", "YOUR_API_SECRET");
    // api.post();
}
