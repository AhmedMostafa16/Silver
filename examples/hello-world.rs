extern crate silver;

use silver::{Handler, Http, Request, Response, Server, SilverResult};
use std::io;

struct HelloWorld;

impl Handler for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Future = SilverResult;
    type Error = io::Error;

    fn call(&self, _: Request) -> SilverResult {
        let mut resp = Response::new();

        resp.body("Hello, World!");

        resp.ok();
    }
}

fn main() {
    // Set your own address and port here.
    let addr = "0.0.0.0:8000".parse().unwrap();
    let mut server = Server::new(Http, addr);
    // Set the number of Your CPU cores.
    server.threads(8);
    server.serve(|| Ok(HelloWorld));
}
