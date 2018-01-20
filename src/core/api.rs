extern crate chrono;
extern crate openssl;
extern crate reqwest;
extern crate serde_json;

use self::reqwest::header::Headers;
use self::chrono::Utc;
use self::openssl::hash::MessageDigest;
use self::openssl::pkey::PKey;
use self::openssl::sign::Signer;
use self::serde_json::{Value, Error};

use std::collections::HashMap;

use core::{AccessKey, Method};

pub struct Api {
    uri: String,
    method: Method,
    access_key: Option<AccessKey>,
    param: HashMap<String, String>,
}

impl Api {
    pub fn exec(&self) -> reqwest::Result<String> {
        match self.method {
            Method::Get => self.get(),
            Method::Post => self.post(),
        }
    }

    fn get(&self) -> reqwest::Result<String> {
        let mut resp = reqwest::get(self.uri.as_str())?;

        assert!(resp.status().is_success());
        resp.text()
    }

    fn post(&self) -> reqwest::Result<String> {
        // body生成
        let body = self.create_body();
        let sign = self.create_sign(body.as_str());

        let mut headers = Headers::new();

        let access_key = self.access_key.clone().unwrap();
        headers.set_raw("Key", access_key.key.as_str());
        headers.set_raw("Sign", sign.as_str());

        let client = reqwest::Client::new();
        let uri = "https://api.zaif.jp/tapi";
        let mut res = client.post(uri).headers(headers).body(body).send().unwrap();

        assert!(res.status().is_success());
        let response_body = res.text().unwrap();
        let v: Value = serde_json::from_str(response_body.as_str()).unwrap();
        if v["success"].as_i64().unwrap() == 0 {
            let msg = v["error"].as_str().unwrap();
            panic!(msg.to_string());
        }

        Ok(response_body)
    }

    fn create_body(&self) -> String {
        let now = Utc::now();
        let nonce = format!("{}.{}", now.timestamp(), now.timestamp_subsec_millis());
        let param = &mut self.param.clone();
        param.insert("nonce".to_string(), nonce);

        let body = &mut String::new();
        let param_strs = param.iter().map(|(key, val)| {
            format!("{}={}", key.as_str(), val.as_str())
        });
        for val in param_strs {
            if body.len() != 0 {
                body.push('&');
            }
            body.push_str(val.as_str());
        }
        body.clone()
    }
    fn create_sign(&self, body: &str) -> String {
        let access_key = self.access_key.clone().unwrap();
        let key = PKey::hmac(access_key.secret.as_bytes()).unwrap();
        let mut signer = Signer::new(MessageDigest::sha512(), &key).unwrap();
        signer.update(body.as_bytes()).unwrap();
        // Rust: byte array to hex String
        // http://illegalargumentexception.blogspot.jp/2015/05/rust-byte-array-to-hex-string.html
        let signed = signer.sign_to_vec().unwrap();
        let strs: Vec<String> = signed.iter().map(|b| format!("{:02x}", b)).collect();
        let sign = strs.join("");
        // println!("Key: {}", self.api_key.as_str());
        // println!("Body: {}", body);
        // println!("Sign: {}", sign);
        sign
    }
}

pub struct ApiBuilder {
    uri: String,
    method: Method,
    access_key: Option<AccessKey>,
    param: HashMap<String, String>,
}

impl ApiBuilder {
    pub fn new() -> ApiBuilder {
        ApiBuilder {
            uri: "".to_string(),
            method: Method::Get,
            access_key: None,
            param: HashMap::new(),
        }
    }

    pub fn uri(&mut self, uri: &str) -> &mut ApiBuilder {
        self.uri = uri.to_string();
        self
    }

    pub fn method(&mut self, method: Method) -> &mut ApiBuilder {
        self.method = method;
        self
    }

    pub fn access_key(&mut self, access_key: AccessKey) -> &mut ApiBuilder {
        self.access_key = Some(access_key);
        self
    }

    pub fn param(&mut self, param: HashMap<String, String>) -> &mut ApiBuilder {
        self.param = param;
        self
    }

    pub fn finalize(&self) -> Api {
        Api {
            uri: self.uri.clone(),
            method: self.method,
            access_key: self.access_key.clone(),
            param: self.param.clone(),
        }
    }
}
