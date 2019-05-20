extern crate bytes;
extern crate futures;
extern crate http;
extern crate pretty_env_logger;
extern crate silver_rs;
extern crate tokio_io;

use silver_rs::transport::Io;
use silver_rs::upgrade::UpgradeContext;
use silver_rs::{App, Context, Error, Route};

use bytes::Bytes;
use futures::prelude::*;
use futures::stream;
use http::Method;
use tokio_io::codec::{Framed, FramedParts, LinesCodec};

fn handler(
    io: Io,
    read_buf: Bytes,
    _cx: &Context,
) -> impl Future<Item = (), Error = ()> + Send + 'static {
    let parts = FramedParts {
        inner: io,
        readbuf: read_buf.into(),
        writebuf: Default::default(),
    };
    let (sink, stream) = Framed::from_parts(parts, LinesCodec::new()).split();

    let lines = stream
        .take_while(|line| Ok(!line.is_empty()))
        .map_err(|_| ())
        .chain(stream::once(Ok("bye.".into())));

    sink.sink_map_err(|_| ()).send_all(lines).map(|_| ())
}

fn handshake(_cx: &Context) -> Result<UpgradeContext, Error> {
    // TODO: validate request
    Ok(UpgradeContext::builder("lines").finish(handler))
}

fn main() -> silver_rs::app::Result<()> {
    pretty_env_logger::init();
    App::builder()
        .mount("/",vec![Route::new("/", Method::GET, handshake)])
        .serve()
}
