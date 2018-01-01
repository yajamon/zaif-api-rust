extern crate reqwest;

pub struct Api {
    uri: String,
}

impl Api {
    pub fn exec(&self) -> reqwest::Result<String> {
        let mut resp = reqwest::get(self.uri.as_str())?;

        assert!(resp.status().is_success());
        resp.text()
    }
}

pub struct ApiBuilder {
    uri: String,
}

impl ApiBuilder {
    pub fn new() -> ApiBuilder {
        ApiBuilder {
            uri: "".to_string(),
        }
    }

    pub fn finalize(&self) -> Api {
        Api {
            uri: self.uri.clone(),
        }
    }
}
