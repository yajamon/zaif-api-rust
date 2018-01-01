extern crate reqwest;

pub struct Api {
    uri: String,
}

impl Api {
    fn exec(&self) -> reqwest::Result<String> {
        let mut resp = reqwest::get(self.uri.as_str())?;

        assert!(resp.status().is_success());
        resp.text()
    }
}

pub struct ApiBuilder {
    uri: String,
}

impl ApiBuilder {
    fn new() -> ApiBuilder {
        ApiBuilder {
            uri: "".to_string(),
        }
    }

    fn finalize(&self) -> Api {
        Api {
            uri: self.uri.clone(),
        }
    }
}
