extern crate http;
extern crate pretty_env_logger;
extern crate silver_rs;

use http::Method;
use silver_rs::{App, Context, Error, Route};

fn welcome(_cx: &Context) -> Result<&'static str, Error> {
    Ok("Hello")
}

fn main() -> silver_rs::app::Result<()> {
    pretty_env_logger::init();
    App::builder()
        .mount("/", vec![Route::new("/", Method::GET, welcome)])
        .serve()
}
