# zaif-api-rust [![Crates.io](https://img.shields.io/crates/v/zaif-api.svg)](https://crates.io/crates/zaif-api)
Use Zaif-Api from Rust

## Description

Zaif ApiのWrapperです。
reqwest crateを使って実装しています。

## API Reference

http://techbureau-api-document.readthedocs.io/ja/latest/index.html

## Usage

```rs
extern crate zaif_api;

use zaif_api::AccessKey;
use zaif_api::public_api::*;
use zaif_api::trade_api::*;

fn main() {
    let api = CurrenciesBuilder::new().name("btc".to_string()).finalize();
    for currency in api.exec().unwrap() {
        println!("name: {} is_token: {}", currency.name, currency.is_token);
    }

    let access_key = AccessKey::new("YOUR_API_KEY", "YOUR_API_SECRET");

    let api = TradeBuilder::new()
        .access_key(access_key.clone())
        .currency_pair("zaif_jpy".to_string())
        .action(TradeAction::Bid)
        .price(1.0)
        .amount(0.1)
        .finalize();
    let _ = api.exec().and_then(|res| {
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
    });
}
```
