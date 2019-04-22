use super::*;
use crate::models::Snippet;
use actix_web::{
    AsyncResponder, FutureResponse, HttpMessage, HttpRequest, HttpResponse,
    State,
};
use bytes::Bytes;
use futures::Future;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct CreatedSnippet {
    href: String,
}
impl CreatedSnippet {
    fn from_snippet(hash: &str) -> Self {
        CreatedSnippet {
            href: format!("/{}", hash),
        }
    }
}

pub struct Clips;
impl Clips {
    pub fn get(
        (req, state): (HttpRequest<AppState>, State<AppState>),
    ) -> HttpResponse {
        HttpResponse::Ok().json(state.db.lock().unwrap().all())
    }
    pub fn get_one(req: HttpRequest<AppState>) -> HttpResponse {
        let hashid = req.match_info().get("id").unwrap();
        let pk = req.state().hashid.lock().unwrap().decode(hashid);
        let item = req.state().db.lock().unwrap().get(pk);
        HttpResponse::Ok().json(item)
    }
    pub fn post(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
        let db = req.state().db.clone();
        let hashid = req.state().hashid.clone();
        // Default limit for a body size is 256K
        req.body() // <- get Body future
            .from_err()
            .and_then(move |bytes| {
                // <- complete body
                let content = String::from_utf8(bytes.to_vec()).unwrap();
                let item = Snippet { id: None, content };
                let created_item = db.lock().unwrap().insert(item.clone());
                let hash =
                    hashid.lock().unwrap().encode(created_item.id.unwrap());
                let resource = CreatedSnippet::from_snippet(&hash);

                Ok(HttpResponse::Created().json(resource))
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
