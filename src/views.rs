use super::*;
use actix_web::{HttpRequest, HttpResponse};
use askama::actix_web::TemplateIntoResponse;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "snippet.html")]
struct SnippetTemplate<'a> {
    content: &'a String,
}

pub fn index(req: &HttpRequest<AppState>) -> impl Responder {
    println!("{}", req.uri());
    IndexTemplate.into_response()
}

pub fn snippet(req: &HttpRequest<AppState>) -> impl Responder {
    let hashid = req.match_info().get("id").unwrap();
    println!("{}", hashid);
    let snippet_id = req.state().hashid.lock().unwrap().decode(hashid);
    println!("{}", snippet_id);
    let snippet = req.state().db.lock().unwrap().get(snippet_id);
    println!("{:?}", snippet);

    match snippet {
        Some(snippet) => {
            let template = SnippetTemplate {
                content: &snippet.content,
            };
            template.into_response()
        }
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
