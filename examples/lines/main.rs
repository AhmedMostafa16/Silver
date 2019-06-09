extern crate futures;
extern crate http;
extern crate pretty_env_logger;
extern crate silver_rs;
extern crate tokio_io;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

mod lines;

use http::Method;
use silver_rs::upgrade::Upgrade;
use silver_rs::{App, Context, Error, Route};

fn index(cx: &Context) -> Result<Upgrade, Error> {
    lines::start(cx, |line| {
        if !line.is_empty() {
            Some(format!(">> {}", line))
        } else {
            None
        }
    })
}

fn main() -> silver_rs::AppResult<()> {
    ::std::env::set_var("RUST_LOG", "lines=info");
    pretty_env_logger::init();

    let app = App::builder()
        .mount("/", vec![Route::new("/", Method::GET, index)])
        .finish()?;
    silver_rs::run(app)
}
