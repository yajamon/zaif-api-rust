extern crate zaif_api;
extern crate api_kit;

use std::rc::Rc;
use api_kit::api_client::ApiClient;
use zaif_api::client::{ZaifInfo, InfoRequest};

fn main() {
    let zaif = ZaifInfo {};
    let me = zaif.call(Rc::new(InfoRequest::new()))
        .send();
    println!("{}", me.unwrap());
}
