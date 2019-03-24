use super::*;
use actix_web::{HttpRequest, HttpResponse};

pub fn index(req: &HttpRequest<AppState>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}
