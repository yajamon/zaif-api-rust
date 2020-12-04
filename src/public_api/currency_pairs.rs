use serde_json;

use crate::public_api::PublicApi;

builder!(CurrencyPairsBuilder => CurrencyPairs {
    currency_pair: String = "all".to_string()
});

impl CurrencyPairs {
    pub fn exec(&self) -> crate::Result<Vec<CurrencyPairsResponse>> {
        Ok(serde_json::from_value(<Self as PublicApi>::exec(&self)?)?)
    }
}

impl PublicApi for CurrencyPairs {
    fn action(&self) -> &str {
        "currency_pairs"
    }
    fn parameter(&self) -> &str {
        self.currency_pair.as_str()
    }
}

#[derive(Deserialize)]
pub struct CurrencyPairsResponse {
    pub name: String,
    pub title: String,
    pub currency_pair: String,
    pub description: String,
    pub is_token: bool,
    pub event_number: i64,
    pub seq: i64,
    pub item_unit_min: f64,
    pub item_unit_step: f64,
    pub item_japanese: String,
    pub aux_unit_min: f64,
    pub aux_unit_step: f64,
    pub aux_unit_point: i64,
    pub aux_japanese: String,
}
