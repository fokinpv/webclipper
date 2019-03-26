#![allow(dead_code)]
#![allow(unused_variables)]
// #![allow(unused_imports)]
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate serde_derive;

use actix_web::{fs, middleware, server, App, Json, Responder};
use std::env;
use std::sync::{Arc, Mutex};

mod db;
mod handlers;
mod models;
mod views;

use db::{DBType, DB};
use handlers::Clips;
use models::Item;
use views::index;

pub struct AppState {
    db: Arc<Mutex<DBType>>,
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
    let db = Arc::new(Mutex::new(DB::default()));

    server::new(move || {
        App::with_state(AppState { db: db.clone() })
            .middleware(middleware::Logger::default())
            .handler("/static", fs::StaticFiles::new("static").unwrap())
            .resource("/", |r| r.f(index))
            .resource("/clips", |r| {
                r.get().with(Clips::get);
                r.post().with(Clips::post);
            })
            .resource("/clips/{id}", |r| r.get().with(Clips::get_one))
    })
    .bind(format!("0.0.0.0:{}", port))
    .expect(&format!("Can not bind to port {}", port))
    .run();
}
