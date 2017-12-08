extern crate zaif_api;

use zaif_api::Currencies;

fn main() {
    let api = Currencies{};
    api.get();
}
