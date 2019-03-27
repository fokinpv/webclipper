use super::*;
use actix_web::{
    AsyncResponder, FutureResponse, HttpMessage, HttpRequest, HttpResponse,
    State,
};
use bytes::Bytes;
use futures::Future;

pub struct Clips;
impl Clips {
    pub fn get(
        (req, state): (HttpRequest<AppState>, State<AppState>),
    ) -> HttpResponse {
        HttpResponse::Ok().json(state.db.lock().unwrap().all())
    }
    pub fn get_one(
        (req, state): (HttpRequest<AppState>, State<AppState>),
    ) -> HttpResponse {
        let pk: usize = req.match_info().get("id").unwrap().parse().unwrap();
        let item = state.db.lock().unwrap().get(pk);
        HttpResponse::Ok().json(item)
    }
    pub fn post(
        (req, state): (HttpRequest<AppState>, State<AppState>),
    ) -> FutureResponse<HttpResponse> {
        req.body() // <- get Body future
            .limit(8192) // <- change max size of the body to a 4kb
            .from_err()
            .and_then(move |bytes| {
                // <- complete body
                let content = String::from_utf8(bytes.to_vec()).unwrap();
                let item = Item {
                    id: None,
                    content: content,
                };
                let created_item =
                    state.db.lock().unwrap().insert(item.clone());
                Ok(HttpResponse::Ok().json(created_item))
            })
            .responder()
    }
    pub fn delete_one(
        (req, state): (HttpRequest<AppState>, State<AppState>),
    ) -> HttpResponse {
        let pk: usize = req.match_info().get("id").unwrap().parse().unwrap();
        let item = state.db.lock().unwrap().get(pk);
        HttpResponse::Ok().json(item)
    }
}
