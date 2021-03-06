# Silver

Silver is a next generation Web framework for Rust.

> **This project is now actively development.**

[![Build Status](https://travis-ci.org/AhmedMostafa16/Silver.svg?branch=master)](https://travis-ci.org/AhmedMostafa16/Silver)
[![Coverage Status](https://coveralls.io/repos/github/AhmedMostafa16/Silver/badge.svg?branch=master)](https://coveralls.io/github/AhmedMostafa16/Silver?branch=master)

## Features

  - Ultra-fast.
  - Thread-safety.
  - Feature-rich.
  - Scalability.
  - Built-in router.
  - Built-in Cookies manager.
  - Custom error handling.
  - Asynchronous.
  - Clear syntax.

      and more...


#
## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
silver-rs = "0.2.0-dev"
```

#
## Syntax

**Silver**

```rust
extern crate http;
extern crate pretty_env_logger;
extern crate silver_rs;

use http::Method;
use silver_rs::{App, Context, Error, Route};

fn welcome(_cx: &Context) -> Result<&'static str, Error> {
    Ok("Hello World!")
}

fn main() -> silver_rs::AppResult<()> {
    pretty_env_logger::init();
    let app = App::builder()
        .mount("/", vec![Route::new("/", Method::GET, welcome)])
        .finish()?;

    silver_rs::run(app)
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

#
## Performance
The benchmark results have been computed with this command: 
```wrk -t16 -c500 -d10s http://127.0.0.1:8080 --latency```

**Silver**

```
Running 10s test @ http://127.0.0.1:8080
  16 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.86ms    5.83ms 217.09ms   95.70%
    Req/Sec    14.17k     2.27k   32.40k    74.77%
  Latency Distribution
     50%    1.70ms
     75%    3.08ms
     90%    5.52ms
     99%   18.80ms
  2261180 requests in 10.08s, 278.18MB read
Requests/sec: 224312.89
Transfer/sec:     27.60MB
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

#
### License

Silver-rs is licensed under [MIT license](LICENSE-MIT).
