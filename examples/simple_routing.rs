extern crate silver_rs;

use silver_rs::{Http, Handler, Request, Response, Server, SilverResult};
use std::io;

struct HelloWorld;

impl Handler for HelloWorld {
    type Future = SilverResult;
    type Request = Request;
    type Response = Response;
    type Error = io::Error;

    fn call(&self, req: Request) -> SilverResult {
        let mut resp = Response::new();

        match (req.method(), req.path()) {
            ("GET", "") => {
                resp.body("Hello, World!");
            },
            ("GET", "/bye") => {
                resp.body("Bye, World!");
            },
            _ => {
                resp.body("Not Found").status(404, "Not Found");
            }
        }

        resp.ok()
    }
}

fn main() {
    // Change to your address and port.
    let addr = "0.0.0.0:8080".parse().unwrap();
    let mut server = Server::new(Http, addr);
    // Change to number of your CPU threads.
    server.threads(8);
    server.serve(|| Ok(HelloWorld));
}
