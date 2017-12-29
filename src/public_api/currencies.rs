extern crate reqwest;

pub struct Currencies {}

impl Currencies {
    pub fn get(&self, name: &str) -> reqwest::Result<String> {
        let uri = format!("https://api.zaif.jp/api/1/currencies/{}", name);
        let mut resp = reqwest::get(uri.as_str())?;

        assert!(resp.status().is_success());
        resp.text()
    }
}
