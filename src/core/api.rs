extern crate reqwest;

use core::Method;

pub struct Api {
    uri: String,
    method: Method,
}

impl Api {
    pub fn exec(&self) -> reqwest::Result<String> {
        match self.method {
            Method::Get => self.get(),
        }
    }

    fn get(&self) -> reqwest::Result<String> {
        let mut resp = reqwest::get(self.uri.as_str())?;

        assert!(resp.status().is_success());
        resp.text()
    }
}

pub struct ApiBuilder {
    uri: String,
    method: Method,
}

impl ApiBuilder {
    pub fn new() -> ApiBuilder {
        ApiBuilder {
            uri: "".to_string(),
            method: Method::Get,
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

    pub fn finalize(&self) -> Api {
        Api {
            uri: self.uri.clone(),
            method: self.method,
        }
    }
}
