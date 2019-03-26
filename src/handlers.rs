use super::*;
use actix_web::{HttpRequest, HttpResponse, State};

pub struct Clips;
impl Clips {
    pub fn get((req, state): (HttpRequest<AppState>, State<AppState>)) -> HttpResponse {
        HttpResponse::Ok().json(state.db.lock().unwrap().all())
    }
    pub fn get_one((req, state): (HttpRequest<AppState>, State<AppState>)) -> HttpResponse {
        let pk: usize = req.match_info().get("id").unwrap().parse().unwrap();
        let item = state.db.lock().unwrap().get(pk);
        HttpResponse::Ok().json(item)
    }
    pub fn post((item, state): (Json<Item>, State<AppState>)) -> HttpResponse {
        let created_item = state.db.lock().unwrap().insert(item.clone());
        HttpResponse::Ok().json(created_item)
    }
}
