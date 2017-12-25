extern crate reqwest;
extern crate chrono;
extern crate openssl;

use std::io::{self, Write};
use self::reqwest::header::Headers;
use self::chrono::Utc;
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

        let mut headers = Headers::new();

        headers.set_raw("Key", self.api_key.as_str());
        headers.set_raw("Sign", sign.as_str());

        let client = reqwest::Client::new();
        let uri = "https://api.zaif.jp/tapi";
        let mut res = client.post(uri)
            .headers(headers)
            .body(body)
            .send().unwrap();
        let res_body = res.text().unwrap();

        io::stdout().write(res_body.as_bytes()).unwrap();
    }
}
