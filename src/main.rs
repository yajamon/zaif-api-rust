extern crate zaif_api;

use zaif_api::public_api::*;
use zaif_api::trade_api::get_info2::GetInfo2;

fn main() {
    let api = CurrenciesBuilder::new().name("btc").finalize();
    println!("{}", api.exec().unwrap());

    let api = CurrencyPairsBuilder::new().finalize();
    println!("{}", api.exec().unwrap());

    // let api = GetInfo2::new("YOUR_API_KEY", "YOUR_API_SECRET");
    // api.post();
}
