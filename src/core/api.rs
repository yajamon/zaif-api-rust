extern crate reqwest;

pub struct Api {}

impl Api {
    fn exec(&self) -> reqwest::Result<String> {
        let mut resp = reqwest::get("https://example.com")?;

        assert!(resp.status().is_success());
        resp.text()
    }
}

pub struct ApiBuilder {}

impl ApiBuilder {
    fn new() -> ApiBuilder {
        ApiBuilder {}
    }

    fn finalize(&self) -> Api {
        Api {}
    }
}
