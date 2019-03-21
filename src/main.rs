#![allow(dead_code)]
#![allow(unused_variables)]
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate serde_derive;

use std::env;
use actix_web::{
    server, http, middleware, App, HttpRequest, HttpResponse, Responder, Json
};


#[derive(Debug, Serialize, Deserialize)]
struct Item {
    id: u32,
    data: String,
}

struct Clips;
impl Clips {
    fn get() {}
    fn get_one() {}
    fn post(item: Json<Item>) -> HttpResponse{
        println!("model: {:?}", &item);
        HttpResponse::Ok().json(item.0)
    }
}

// we need to read the PORT from the env variable (Heroku sets it)
fn get_server_port() -> u16 {
    env::var("PORT")
    .ok()
    .and_then(|p| p.parse().ok())
    .unwrap_or(8181)
}

fn index(req: &HttpRequest) -> impl Responder {
    "WebClipper Application"
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port = get_server_port();
    let clips = Clips;

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.f(index))
            // .resource("/clips", |r| r.method(http::Method::POST).with(create_clip))
            .resource("/clips", |r| r.method(http::Method::POST).with(Clips::post))
    })
    .bind(format!("0.0.0.0:{}", port))
    .expect(&format!("Can not bind to port {}", port))
    .run();
}
