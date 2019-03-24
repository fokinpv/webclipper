use actix_web::{HttpRequest, HttpResponse, State};
// use std::thread;
use super::*;

pub struct Clips;
impl Clips {
    pub fn get((req, state): (HttpRequest<AppState>, State<AppState>)) -> HttpResponse {
        HttpResponse::Ok().json(state.db.lock().unwrap().all())
    }
    pub fn get_one() {}
    pub fn post((item, state): (Json<Item>, State<AppState>)) -> HttpResponse {
        println!("{:?}", std::thread::current().id());
        state.db.lock().unwrap().add(item.0.clone());
        println!("{:?}", item);
        println!("{:?}", state.db.lock().unwrap().all());
        HttpResponse::Ok().json(item.0)
    }
}
