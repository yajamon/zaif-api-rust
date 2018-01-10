# zaif-api-rust
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
    let api = public_api::DepthBuilder::new().currency_pair("btc_jpy").finalize();
    println!("{}", api.exec().unwrap());

    let access_key = AccessKey::new("YOUR_API_KEY", "YOUR_API_SECRET");

    let api = TradeBuilder::new(access_key.clone())
        .currency_pair("zaif_jpy")
        .action(TradeAction::Bid)
        .price(1.0)
        .amount(0.1)
        .finalize();
    println!("{}", api.exec().unwrap());
}
```
