extern crate silver_rs;

use std::io;

use silver_rs::prelude::*;

struct HelloWorld;

impl Handler for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = SilverResult;

    fn call(&self, _: Request) -> SilverResult {
        let mut resp = Response::new();

        resp.body("Hello World!");

        resp.ok()
    }
}

fn main() {
    // Change to your address and port.
    let addr = "0.0.0.0:8080".parse().unwrap();
    let mut server = Server::new(Http, addr);
    // Change to number of your CPU threads.
    server.threads(16);
    server.serve(|| Ok(HelloWorld));
}
