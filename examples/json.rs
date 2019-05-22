extern crate silver_rs;
extern crate http;
#[macro_use]
extern crate serde;
extern crate pretty_env_logger;

use silver_rs::json::Json;
use silver_rs::{App, Context,Route};
use http::Method;

#[derive(Debug, Serialize)]
struct User{
    name:String,
    age:u32,
}

fn handler(_:&Context)->silver_rs::Result<Json<User>>{
    Ok(Json(User{
        name: "Ahmed Mostafa".into(),
        age: 17,
    }))
}

fn main()-> silver_rs::AppResult<()>{
    pretty_env_logger::init();
    let app = App::builder().mount("/", vec![Route::new("/", Method::GET, handler)]).finish()?;
    silver_rs::run(app)
}