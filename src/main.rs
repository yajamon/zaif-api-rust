extern crate zaif_api;
extern crate serde_json;

use std::{thread, time, env};
use serde_json::Value;

use zaif_api::AccessKey;
use zaif_api::public_api::*;
use zaif_api::trade_api::*;

fn put_help() {
    let s = "
command [action]
        currency
        currency_pair
        last_price
        depth
        trades
        ticker
        get_info2
        trade
        active_orders
        get_personal_info
";
    println!("{}", s);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        put_help();
        return;
    }
    let action = &args[1];
    println!("action: {}", action);

    let access_key = AccessKey::new("YOUR_ACCESS_KEY", "YOUR_SECRET");

    match action.as_str() {
        "currency" => call_currency(),
        "currency_pair" => call_currency_pair(),
        "last_price" => call_last_price(),
        "depth" => call_depth(),
        "trades" => call_trades(),
        "ticker" => call_ticker(),
        "get_info2" => call_get_info2(&access_key),
        "trade" => call_trade(&access_key),
        "active_orders" => call_active_orders(&access_key),
        "get_personal_info" => call_get_personal_info(&access_key),
        _ => put_help(),
    }
    return;
}

fn call_currency() {
    let api = CurrenciesBuilder::new().name("btc".to_string()).finalize();
    for currency in api.exec().unwrap() {
        println!("name: {} is_token: {}", currency.name, currency.is_token);
    }
}

fn call_currency_pair() {
    let api = CurrencyPairsBuilder::new().finalize();
    for currency_pair in api.exec().unwrap() {
        println!(
            "name: {} description: {}",
            currency_pair.name,
            currency_pair.description
        );
    }
}

fn call_last_price() {
    let api = LastPriceBuilder::new()
        .currency_pair("btc_jpy".to_string())
        .finalize();
    println!("last_price: {}", api.exec().unwrap().last_price);

}

fn call_depth() {
    let api = DepthBuilder::new()
        .currency_pair("btc_jpy".to_string())
        .finalize();
    for ask in api.exec().unwrap().asks {
        println!("ask price: {} amount: {}", ask.price(), ask.amount());
    }
    for bid in api.exec().unwrap().bids {
        println!("bid price: {} amount: {}", bid.price(), bid.amount());
    }

}

fn call_trades() {
    let api = TradesBuilder::new()
        .currency_pair("btc_jpy".to_string())
        .finalize();
    for trade in api.exec().unwrap() {
        println!(
            "type: {}, price: {}, amount: {}",
            trade.trade_type,
            trade.price,
            trade.amount
        );
    }
}

fn call_ticker() {
    let api = TickerBuilder::new()
        .currency_pair("btc_jpy".to_string())
        .finalize();
    let res = api.exec().unwrap();
    println!("last: {}, high: {}, low: {}", res.last, res.high, res.low);
}

fn call_get_info2(access_key: &AccessKey) {
    let api = GetInfo2Builder::new()
        .access_key(access_key.clone())
        .finalize();
    for (coin, amount) in api.exec().unwrap().funds.iter() {
        println!("coin: {} amount: {}", coin, amount);
    }
}

fn call_trade(access_key: &AccessKey) {
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
}

fn call_active_orders(access_key: &AccessKey) {

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

fn call_get_personal_info(access_key: &AccessKey) {
    let api = GetPersonalInfoBuilder::new()
        .access_key(access_key.clone())
        .finalize();
    println!("{}", api.exec().unwrap().ranking_nickname);
}
