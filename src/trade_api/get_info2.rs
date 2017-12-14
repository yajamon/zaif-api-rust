extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate chrono;
extern crate openssl;

use std::io::{self, Write};
use self::futures::{Future, Stream};
use self::hyper::Client;
use self::hyper::{Method, Request, StatusCode};
use self::hyper_tls::HttpsConnector;
use self::tokio_core::reactor::Core;
use self::chrono::{DateTime, Utc};
use self::openssl::hash::MessageDigest;
use self::openssl::pkey::PKey;
use self::openssl::sign::Signer;

pub struct GetInfo2 {
    api_key: String,
    api_secret: String,
}

impl GetInfo2 {
    pub fn new(key: &str, secret: &str) -> GetInfo2 {
        GetInfo2 {
            api_key: key.to_string(),
            api_secret: secret.to_string(),
        }
    }
    pub fn post(&self) {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .build(&handle);

        let uri = "https://api.zaif.jp/tapi".parse().unwrap();
        let mut req = Request::new(Method::Post, uri);

        // body生成
        let now = Utc::now();
        let nonce = format!("{}.{}", now.timestamp(), now.timestamp_subsec_millis());
        let param = [
            ["method", "get_info2"].join("="),
            ["nonce", nonce.as_str()].join("="),
        ];
        let body = param.join("&");

        let key = PKey::hmac(self.api_secret.as_bytes()).unwrap();
        let mut signer = Signer::new(MessageDigest::sha512(), &key).unwrap();
        signer.update(body.as_bytes());
        // Rust: byte array to hex String
        // http://illegalargumentexception.blogspot.jp/2015/05/rust-byte-array-to-hex-string.html
        let signed = signer.sign_to_vec().unwrap();
        let strs:Vec<String> = signed.iter().map(|b| format!("{:02x}", b)).collect();
        let sign = strs.join("");
        // println!("Key: {}", self.api_key.as_str());
        // println!("Body: {}", body);
        // println!("Sign: {}", sign);

        req.headers_mut().set_raw("Key", self.api_key.as_str());
        req.headers_mut().set_raw("Sign", sign.as_str());
        req.set_body(body);

        let work = client.request(req).and_then(|res| {
            if res.status() != StatusCode::Ok {
                panic!("Response error status: {}", res.status());
            }
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
