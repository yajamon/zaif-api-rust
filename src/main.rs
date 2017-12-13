extern crate zaif_api;

use zaif_api::public_api::currencies::Currencies;

fn main() {
    let api = Currencies{};
    api.get("btc");
}
