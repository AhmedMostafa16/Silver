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

#### Benchmark
> The benchmark used the following code.

Technical details about the server:

    - CPU: Intel Xeon E5620.
    - RAM: 16GB , 1333MHZ.

**Silver**
```
Running 10s test @ http://0.0.0.0:8080
  16 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.31ms    1.69ms  27.61ms   86.60%
    Req/Sec    14.02k     2.37k   24.76k    70.50%
  Latency Distribution
     50%    1.76ms
     75%    2.63ms
     90%    4.55ms
     99%    8.46ms
  2241258 requests in 10.06s, 109.01MB read
Requests/sec: 222811.75
Transfer/sec:     10.84MB

```

**Iron**
```
Running 10s test @ http://localhost:3000
  16 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.01ms    2.88ms  91.44ms   69.64%
    Req/Sec     3.17k     1.32k    7.47k    60.45%
  Latency Distribution
     50%    4.59ms
     75%    6.51ms
     90%    8.72ms
     99%   13.36ms
  253942 requests in 10.10s, 27.61MB read
Requests/sec:  25151.17
Transfer/sec:      2.73MB

```

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
