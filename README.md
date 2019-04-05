# Silver 
Fast http framework for `Rust`.

## About
The goal of this project is, to show how fast Rust can be. It isn't made for huge complex applications, just a test project for benchmark reasons.

## Features
### Speed
- It is wrapping **x3** times faster than Tron which is based on hyper.
- It can do about `~308k` requests per second.

### Syntax
Silver has a flexible and easy syntax compared to Iron.

- Silver
 ```rust
extern crate silver;
use std::io;
use silver::{Server, Http, Handler, SilverResult, Request, Response};

struct HelloWorld;

impl Handler for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = SilverResult;
    fn call(&self, req: Request) -> SilverResult {
        let mut resp = Response::new();
        resp.body("Hello World!").ok()
    }
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    let mut server = Server::new(Http, addr);
    server.threads(8);
    server.serve(|| Ok(HelloWorld));
}
``` 

- Iron
```rust
extern crate iron;
use iron::prelude::*;
use iron::status;
fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }
    let _server = Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}
```

