pub use self::access_key::*;
pub use self::api::*;

mod access_key;
mod api;

#[derive(Copy, Clone)]
pub enum Method {
    Get,
    Post,
}
