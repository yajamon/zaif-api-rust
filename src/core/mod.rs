pub use self::api::*;

mod api;

#[derive(Copy, Clone)]
pub enum Method {
    Get,
}
