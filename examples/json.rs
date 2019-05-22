#![cfg_attr(rustfmt,rustfmt_skip)]

extern crate silver_rs;
extern crate http;
#[macro_use]
extern crate serde;
extern crate futures;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use futures::prelude::*;
use silver_rs::json::Json;
use silver_rs::{App, Context,Route,Error};
use http::Method;

#[derive(Debug, Serialize,Deserialize)]
struct User{
    name:String,
    age:u32,
}

fn get_json(_:&Context)->silver_rs::Result<Json<User>>{
    Ok(Json(User{
        name: "Ahmed Mostafa".into(),
        age: 17,
    }))
}

fn read_json_payload(ctxt:&Context)->impl Future<Item=Json<User>,Error=Error>+Send+'static{
    ctxt.body()
        .read_all()
        .convert_to::<Json<User>>()
        .map(|user| {
            info!("Received: {:?}", user);
            user
        })
}

fn main()-> silver_rs::AppResult<()>{
    pretty_env_logger::init();
    let app = App::builder()        .mount("/", vec![
            Route::new("/", Method::GET, get_json),
            Route::new("/", Method::POST, read_json_payload),
        ]).finish()?;
    silver_rs::run(app)
}