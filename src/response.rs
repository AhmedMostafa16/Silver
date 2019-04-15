use std::fmt::{self, Write};

use bytes::{BufMut, BytesMut};
use futures::future;

use SilverResult;

enum StatusMessage {
    Ok,
    Custom(u32, String),
}

pub struct Response {
    headers: Vec<(String, String)>,
    response: String,
    status_message: StatusMessage,
}

impl Response {
    pub fn new() -> Response {
        Response {
            headers: Vec::new(),
            response: String::new(),
            status_message: StatusMessage::Ok,
        }
    }

    pub fn status(&mut self, code: u32, message: &str) -> &mut Response {
        self.status_message = StatusMessage::Custom(code, message.to_string());
        self
    }

    pub fn header(&mut self, name: &str, val: &str) -> &mut Response {
        self.headers.push((name.to_string(), val.to_string()));
        self
    }

    pub fn body(&mut self, s: &str) -> &mut Response {
        self.response = s.to_string();
        self
    }

    pub fn ok(self) -> SilverResult {
        future::ok(self)
    }
}

struct FastWrite<'a>(&'a mut BytesMut);

fn push(buf: &mut BytesMut, data: &[u8]) {
    buf.reserve(data.len());
    unsafe {
        buf.bytes_mut()[..data.len()].copy_from_slice(data);
        buf.advance_mut(data.len());
    }
}

pub fn encode(msg: Response, buf: &mut BytesMut) {
    let length = msg.response.len();

    write!(
        FastWrite(buf),
        "\
         HTTP/1.1 {}\r\n\
         Server: Example\r\n\
         Content-Length: {}\r\n\
         ",
        msg.status_message,
        length
    )
    .unwrap();

    for &(ref k, ref v) in &msg.headers {
        push(buf, k.as_bytes());
        push(buf, ": ".as_bytes());
        push(buf, v.as_bytes());
        push(buf, "\r\n".as_bytes());
    }

    push(buf, "\r\n".as_bytes());
    push(buf, msg.response.as_bytes());
}

impl<'a> fmt::Write for FastWrite<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        push(&mut *self.0, s.as_bytes());
        Ok(())
    }
    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        fmt::write(self, args)
    }
}

impl fmt::Display for StatusMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StatusMessage::Ok => f.pad("200 OK"),
            StatusMessage::Custom(c, ref s) => write!(f, "{} {}", c, s),
        }
    }
}
