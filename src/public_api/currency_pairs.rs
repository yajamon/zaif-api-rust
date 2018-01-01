extern crate reqwest;

pub struct CurrencyPairs;

impl CurrencyPairs {
    pub fn get(&self, name: &str) -> reqwest::Result<String> {
        let uri = format!("https://api.zaif.jp/api/1/currency_pairs/{}", name);
        let mut resp = reqwest::get(uri.as_str())?;

        assert!(resp.status().is_success());
        resp.text()
    }
}
