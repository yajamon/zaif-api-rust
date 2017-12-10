extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use std::io::{self, Write};
use self::futures::{Future, Stream};
use self::hyper::Client;
use self::hyper_tls::HttpsConnector;
use self::tokio_core::reactor::Core;

pub struct Currencies {
}

impl Currencies {
    pub fn get(&self, name: &str) {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .build(&handle);

        let uri = format!("https://api.zaif.jp/api/1/currencies/{}", name).parse().unwrap();

        let work = client.get(uri).and_then(|res| {
            res.body().for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map(|_| ())
                    .map_err(From::from)
            })
        });
        core.run(work).unwrap();
    }
}
