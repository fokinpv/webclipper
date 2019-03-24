#![allow(dead_code)]
#![allow(unused_variables)]
// #![allow(unused_imports)]
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate serde_derive;

use std::env;
use actix_web::{
    server, middleware, fs, http, App, Responder, Json
};

mod db;
mod models;
mod handlers;
mod views;

use models::*;
use handlers::*;
use views::*;


pub struct AppState {
    db: db::DB<Item>
}

// we need to read the PORT from the env variable (Heroku sets it)
fn get_server_port() -> u16 {
    env::var("PORT")
    .ok()
    .and_then(|p| p.parse().ok())
    .unwrap_or(8181)
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port = get_server_port();

    server::new(|| {
        App::with_state(AppState { db: db::DB::new() })
            .middleware(middleware::Logger::default())
            .handler("/static", fs::StaticFiles::new("static").unwrap())
            .resource("/", |r| r.f(index))
            .resource("/clips", |r| {
                r.method(http::Method::GET).with(Clips::get);
                r.post().with(Clips::post);
            })
    })
    .bind(format!("0.0.0.0:{}", port))
    .expect(&format!("Can not bind to port {}", port))
    .run();
}
