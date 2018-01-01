extern crate reqwest;

pub struct Api {}

impl Api {
    fn exec(&self) {}
}

pub struct ApiBuilder {}

impl ApiBuilder {
    fn new() -> ApiBuilder {}

    fn finalize(&self) -> Api {
        Api {}
    }
}
