extern crate zaif_api;
extern crate serde_json;

use std::{thread, time};
use serde_json::Value;

use zaif_api::AccessKey;
use zaif_api::public_api::*;
use zaif_api::trade_api::*;

fn main() {
    let api = CurrenciesBuilder::new().name("btc".to_string()).finalize();
    for currency in api.exec().unwrap() {
        println!("name: {} is_token: {}", currency.name, currency.is_token);
    }

    let api = CurrencyPairsBuilder::new().finalize();
    for currency_pair in api.exec().unwrap() {
        println!(
            "name: {} description: {}",
            currency_pair.name,
            currency_pair.description
        );
    }

    let api = LastPriceBuilder::new()
        .currency_pair("btc_jpy".to_string())
        .finalize();
    println!("last_price: {}", api.exec().unwrap().last_price);

    let api = DepthBuilder::new()
        .currency_pair("btc_jpy".to_string())
        .finalize();
    for ask in api.exec().unwrap().asks {
        println!("ask price: {} amount: {}", ask.price(), ask.amount());
    }
    for bid in api.exec().unwrap().bids {
        println!("bid price: {} amount: {}", bid.price(), bid.amount());
    }

    let access_key = AccessKey::new("YOUR_API_KEY", "YOUR_API_SECRET");
    let api = GetInfo2Builder::new()
        .access_key(access_key.clone())
        .finalize();
    for (coin, amount) in api.exec().unwrap().funds.iter() {
        println!("coin: {} amount: {}", coin, amount);
    }

    let api = TradeBuilder::new()
        .access_key(access_key.clone())
        .currency_pair("zaif_jpy".to_string())
        .action(TradeAction::Bid)
        .price(1.0)
        .amount(0.1)
        .finalize();
    match api.exec()
        .and_then(|res| {
            println!(
                "received: {}, remains: {}, order_id: {}",
                res.received,
                res.remains,
                res.order_id
            );
            if res.order_id == 0 {
                panic!("Complete trade.");
            }
            Ok(res.order_id)
        })
        .and_then(|order_id| {
            let api = CancelOrderBuilder::new()
                .access_key(access_key.clone())
                .order_id(order_id)
                .currency_pair(Some("zaif_jpy".to_string()))
                .finalize();
            let wait_time = time::Duration::from_secs(5);
            thread::sleep(wait_time);
            api.exec()
        })
        .and_then(|res| {
            println!("Cancel order_id: {}", res.order_id);
            Ok(())
        }) {

        Ok(_) => println!("Complete trade and cancel"),
        Err(e) => println!("Error: {}", e),
    }

    let api = ActiveOrdersBuilder::new()
        .access_key(access_key.clone())
        .currency_pair(Some("zaif_jpy".to_string()))
        .finalize();
    for (order_id, order) in api.exec().unwrap().iter() {
        println!(
            "order_id: {}, currency_pair: {}, action: {}, amount: {}, price: {}",
            order_id,
            order.currency_pair,
            order.action,
            order.amount,
            order.price
        );
    }
}
