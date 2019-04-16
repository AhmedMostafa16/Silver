# Silver
Fast http framework for `Rust`.

## About
The goal of this project is, to show how fast Rust can be. It isn't made for huge complex applications, just a test project for benchmark reasons.

## Usage
Add the following to your `Cargo.toml`:
```toml
[dependencies]
silver-rs = 0.1
```

## Features
### Speed
- It is wrapping **x3** times faster than Iron which is based on hyper.
- It can do about `~908k` requests per second.

### Syntax
Silver has a flexible and easy syntax compared to Iron.

- Silver
This example can be run, by:

```
$ git clone https://github.com/AhmedMostafa16/Silver && cd Silver
$ cargo run --example hello-world --release
```

```rust
extern crate silver_rs;

use std::io;
use silver_rs::{Server, Http, Handler, SilverResult, Request, Response};

struct HelloWorld;

impl Handler for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Future = SilverResult;
    type Error = io::Error;

    fn call(&self, _:Request) -> SilverResult {
        let mut resp = Response::new();

        resp.body("Hello, World!");
        resp.ok();
    }
}

fn main() {
    let addr = "0.0.0.0:8000".parse().unwrap();
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
