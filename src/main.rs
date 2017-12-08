extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

fn main() {
    let mut core = match Core::new() {
        Ok(v) => v,
        Err(_) => panic!("error"),
    };
    let client = Client::new(&core.handle());

    //let uri = match "https://api.zaif.jp/api/1/currencies/btc".parse() {
    let uri = match "http://www.example.com/".parse() {
        Ok(v) => v,
        Err(_) => panic!("error"),
    };

    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map(|_| ())
                .map_err(From::from)
        })
    });
    match core.run(work) {
        Ok(_) => println!("ok"),
        Err(_) => panic!("error"),
    }
}
