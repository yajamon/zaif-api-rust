pub use self::api::*;
pub use self::access_key::*;

mod api;
mod access_key;

#[derive(Copy, Clone)]
pub enum Method {
    Get,
}
