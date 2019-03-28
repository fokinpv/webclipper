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
    fn from_snippet(snippet: &Snippet) -> Self {
        CreatedSnippet {
            href: format!("/api/snippets/{}", snippet.id.unwrap()),
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
                let item = Snippet {
                    id: None,
                    content: content,
                };
                let created_item =
                    state.db.lock().unwrap().insert(item.clone());
                let resource = CreatedSnippet::from_snippet(&created_item);

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
