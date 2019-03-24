use actix_web::{ HttpRequest, HttpResponse, State };
use super::*;

pub struct Clips;
impl Clips {
    pub fn get((req, state): (HttpRequest<AppState>, State<AppState>)) -> HttpResponse {
        println!("{:?}", state.db.all());
        HttpResponse::Ok().finish()
    }
    pub fn get_one() {}
    pub fn post((item, state): (Json<Item>, State<AppState>)) -> HttpResponse {
        state.db.add(item.0.clone());
        println!("{:?}", item);
        println!("{:?}", state.db.all());
        HttpResponse::Ok().json(item.0)
    }
}
