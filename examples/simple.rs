extern crate http;
extern crate pretty_env_logger;
extern crate silver_rs;

use http::Method;
use silver_rs::app::App;
use silver_rs::context::Context;
use silver_rs::error::Error;
use silver_rs::router::{Route, RouterContext};

fn welcome(_cx: &Context, _rcx: &mut RouterContext) -> Result<&'static str, Error> {
    Ok("Hello")
}

fn main() -> silver_rs::rt::Result<()> {
    pretty_env_logger::init();
    App::builder()
        .mount(vec![Route::new("/", Method::GET, welcome)])
        .finish()?
        .serve()
}
