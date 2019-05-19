extern crate silver_rs;
extern crate http;
extern crate pretty_env_logger;

use silver_rs::{App, Context, Error, Route};
use http::Method;

fn welcome(_cx: &Context) -> Result<&'static str, Error> {
    Ok("Hello")
}

fn main() -> silver_rs::app::Result<()> {
    pretty_env_logger::init();
    App::builder()
        .mount(vec![Route::new("/", Method::GET, welcome)])
        .serve()
}