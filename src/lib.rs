extern crate bytes;
extern crate futures;
extern crate httparse;
extern crate tokio_codec;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

mod request;
mod response;

use std::io;

pub use request::Request;
pub use response::Response;

use bytes::BytesMut;
use futures::future;
use tokio_codec::*;
use tokio_io::codec::Framed;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_proto::pipeline::ServerProto;

pub use tokio_proto::TcpServer as Server;
pub use tokio_service::Service as Handler;

pub type SilverResult = future::Ok<Response, io::Error>;

pub mod prelude {
    pub use request::Request;
    pub use response::Response;
    pub use Handler;
    pub use Http;
    pub use Server;
    pub use SilverResult;
}

pub struct Http;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for Http {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, HttpCodec>;
    type BindTransport = io::Result<Framed<T, HttpCodec>>;

    fn bind_transport(&self, io: T) -> io::Result<Framed<T, HttpCodec>> {
        Ok(io.framed(HttpCodec))
    }
}

pub struct HttpCodec;

impl Decoder for HttpCodec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Request>> {
        request::decode(buf)
    }
}

impl Encoder for HttpCodec {
    type Item = Response;
    type Error = io::Error;

    fn encode(&mut self, msg: Response, buf: &mut BytesMut) -> io::Result<()> {
        response::encode(&msg, buf);
        Ok(())
    }
}
