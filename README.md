# Silver


Silver is a next generation Web framework for Rust.

> **This project is now actively development.**

## Features

- Ultra-fast.
- Thread-safety.
- Scalability.
- Built-in router.
- Asynchronous.
- Clear syntax.

    and more...


## Usage

Add the following to your Cargo.toml:

[dependencies]
silver-rs = "0.2.0-dev"


## Syntax

**Silver**

```rust
extern crate http;
extern crate pretty_env_logger;
extern crate silver_rs;

use http::Method;
use silver_rs::router::{Route, RouterContext};
use silver_rs::{App, Context, Error};

fn welcome(_cx: &Context, _rcx: &mut RouterContext) -> Result<&'static str, Error> {
    Ok("Hello World!")
}

fn main() -> silver_rs::app::Result<()> {
    pretty_env_logger::init();
    App::builder()
        .mount(vec![Route::new("/", Method::GET, welcome)])
        .finish()?
        .serve()
        // address is 127.0.0.1:8080
        // it will be changable soon.
}
```
for more, go to [examples](/examples).

**Iron**

```rust
extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    Iron::new(hello_world).http("127.0.0.1:3000").unwrap();
}
```

## Performance
The benchmark results have been computed with this command: 
```wrk -t16 -c500 -d10s http://127.0.0.1:4000 --latency```

**Silver**

```
Running 10s test @ http://127.0.0.1:8080
  16 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.40ms    3.67ms 230.59ms   96.92%
    Req/Sec    13.66k     2.16k   35.14k    87.82%
  Latency Distribution
     50%    2.07ms
     75%    2.88ms
     90%    3.98ms
     99%    9.06ms
  2046797 requests in 10.09s, 249.85MB read
Requests/sec: 202791.97
Transfer/sec:     24.75MB
```

**Iron**

```
Running 10s test @ http://127.0.0.1:3000
  16 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   764.38us  719.64us  89.94ms   96.01%
    Req/Sec    17.04k     9.79k   37.05k    58.54%
  Latency Distribution
     50%  716.00us
     75%    0.93ms
     90%    1.15ms
     99%    2.91ms
  1530505 requests in 10.09s, 166.39MB read
Requests/sec: 151658.40
Transfer/sec:     16.49MB
```



### License

Silver-rs is licensed under [MIT license](LICENSE-MIT).
