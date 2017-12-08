extern crate api_kit;
extern crate hyper;
extern crate serde_json;

use std::io::{Read};
use std::rc::Rc;
use std::cell::RefCell;
use self::api_kit::api_request::{ApiRequest, ApiRequestBuilder};
use self::api_kit::api_request::HttpMethod;
use self::api_kit::api_client::ApiClient;
use self::api_kit::error::ApiError;
use self::hyper::header::{Headers, Accept, qitem};
use self::hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use self::hyper::client::request::Request;
use self::hyper::client::response::Response;
use self::hyper::net::Fresh;


pub struct ZaifInfo {
}

impl ApiClient for ZaifInfo {
    fn base_url(&self) -> String {
        return String::from("https://api.zaif.jp/api/1")
    }
}

pub struct InfoRequest {
}

impl InfoRequest {
    pub fn new() -> InfoRequest {
        InfoRequest {}
    }
}

impl ApiRequestBuilder<serde_json::Value> for InfoRequest {

    fn method(&self) -> HttpMethod {
        return HttpMethod::Get;
    }

    fn path(&self) -> String {
        return String::from("/currencies");
    }

    fn queryParameters(&self) -> Vec<(String, String)> {
        return vec![];
    }

    fn interceptRequest(&self, mut request: Request<Fresh>) -> Result<Request<Fresh>, ApiError> {
        request.headers_mut().set(
            Accept(vec![
                   qitem(Mime(TopLevel::Application, SubLevel::Json,
                              vec![(Attr::Charset, Value::Utf8)])),
            ])
        );
        return Ok(request);
    }

    fn responseFromObject(&self, response: Rc<RefCell<Response>>) -> Result<serde_json::Value, ApiError> {
        let mut raw_response = (*response).borrow_mut();
        let mut buffer = String::new();
        raw_response.read_to_string(&mut buffer).unwrap();
        return Ok(serde_json::from_str(&buffer).unwrap());
    }
}
