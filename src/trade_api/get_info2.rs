extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use self::hyper::Client;
use self::hyper_tls::HttpsConnector;
use self::tokio_core::reactor::Core;

pub struct GetInfo2 {
}

impl GetInfo2 {
    pub fn post(&self) {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .build(&handle);
    }
}
