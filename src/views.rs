use super::*;
use actix_web::{HttpRequest, HttpResponse};
use askama::Template;
use askama::actix_web::TemplateIntoResponse;

#[derive(Template)]
#[template(path = "snippet.html")]
struct SnippetTemplate<'a> {
    content: &'a String,
}

pub fn index(req: &HttpRequest<AppState>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

pub fn snippet(req: &HttpRequest<AppState>) -> impl Responder {
    let snippet_id: usize =
        req.match_info().get("id").unwrap().parse().unwrap();
    let snippet = req.state().db.lock().unwrap().get(snippet_id);
    match snippet {
        Some(snippet) => {
            let template = SnippetTemplate {
                content: &snippet.content,
            };
            template.into_response()
        },
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
