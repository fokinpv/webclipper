#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
extern crate actix_web;
#[macro_use]
extern crate askama;
extern crate env_logger;
#[macro_use]
extern crate serde_derive;

use actix_web::{fs, middleware, server, App, Json, Responder};
use std::env;
use std::sync::{Arc, Mutex};

mod db;
mod handlers;
mod hashid;
mod models;
mod views;

use db::{DBType, DB};
use handlers::Clips;
use hashid::HashID;
use views::{index, snippet};

pub struct AppState {
    db: Arc<Mutex<DBType>>,
    hashid: Arc<Mutex<HashID>>,
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
    let hashid = Arc::new(Mutex::new(HashID::new()));
    println!("{}", env::var("HOST").unwrap_or("localhost".to_string()));

    server::new(move || {
        App::with_state(AppState {
            db: db.clone(),
            hashid: hashid.clone(),
        })
        .middleware(middleware::Logger::default())
        .handler("/static", fs::StaticFiles::new("static").unwrap())
        .resource("/", |r| r.f(index))
        .resource("/{id}", |r| r.f(snippet))
        .resource("/api/snippets", |r| {
            r.get().with(Clips::get);
            r.post().with(Clips::post);
        })
        .resource("/api/snippets/{id}", |r| r.get().with(Clips::get_one))
    })
    .bind(format!("0.0.0.0:{}", port))
    .expect(&format!("Can not bind to port {}", port))
    .run();
}
