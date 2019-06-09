extern crate http;
extern crate silver_rs;

#[cfg(unix)]
fn main() -> silver_rs::AppResult<()> {
    use http::Method;
    use silver_rs::server::Server;
    use silver_rs::{App, Route};

    let sock_path: std::path::PathBuf = std::env::args()
        .nth(1)
        .map(Into::into)
        .unwrap_or_else(|| "/tmp/silver_rs-uds.sock".into());

    let app = App::builder()
        .mount("/", vec![Route::new("/", Method::GET, |_: &_| Ok("Hello"))])
        .finish()?;

    let server = Server::builder()
        .transport(|t| {
            t.bind_uds(&sock_path);
        })
        .finish(app)?;

    println!("Serving on {}...", sock_path.display());
    println!();
    println!("The test command is as follows:");
    println!();
    println!("  $ curl --unix-socket /tmp/silver_rs-uds.sock http://localhost/");
    println!();
    server.serve();

    Ok(())
}

#[cfg(not(unix))]
fn main() {
    println!("This example works only on Unix platform.");
}
