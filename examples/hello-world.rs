extern crate silver_rs;

use silver_rs::prelude::*;
use std::io::Error as SilverError;
struct HelloWorld;

impl Handler for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = SilverError;
    type Future = SilverResult;

    fn call(&self, _: Request) -> SilverResult {
        let mut resp = Response::new();

        resp.body("Hello World!");

        resp.ok()
    }
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    let mut server = Server::new(Http, addr);
    server.threads(8);
    server.serve(|| Ok(HelloWorld));
}
