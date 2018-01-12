extern crate zaif_api;
extern crate serde_json;

use serde_json::Value;

use zaif_api::AccessKey;
use zaif_api::public_api::*;
use zaif_api::trade_api::*;

fn main() {
    let api = CurrenciesBuilder::new().name("btc").finalize();
    println!("{}", api.exec().unwrap());

    let api = CurrencyPairsBuilder::new().finalize();
    println!("{}", api.exec().unwrap());

    let api = LastPriceBuilder::new().currency_pair("btc_jpy").finalize();
    println!("{}", api.exec().unwrap());

    let api = DepthBuilder::new().currency_pair("btc_jpy").finalize();
    println!("{}", api.exec().unwrap());

    let access_key = AccessKey::new("YOUR_API_KEY", "YOUR_API_SECRET");
    let api = GetInfo2Builder::new(access_key.clone()).finalize();
    println!("{}", api.exec().unwrap());

    let api = TradeBuilder::new(access_key.clone())
        .currency_pair("zaif_jpy")
        .action(TradeAction::Bid)
        .price(1.0)
        .amount(0.1)
        .finalize();
    match api.exec() {
        Ok(res) => {
            println!("{}", res);
            let json:Value = serde_json::from_str(res.as_str()).unwrap();
            let order_id = json["return"]["order_id"].as_u64().unwrap();
            let api = CancelOrderBuilder::new(access_key.clone())
                .order_id(order_id)
                .finalize();
            println!("{}", api.exec().unwrap());
        },
        _ => return,
    }

    let api = ActiveOrdersBuilder::new(access_key.clone()).currency_pair("zaif_jpy").finalize();
    println!("{}", api.exec().unwrap());
}
