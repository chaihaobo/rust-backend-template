use std::sync::Arc;
use crate::context::AppContext;
use actix_web::{get, web, HttpResponse, Responder};

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_word);
}

#[get("/")]
async fn hello_word(ctx: web::Data<AppContext>) -> impl Responder {
    HttpResponse::Ok().body(ctx.service.hello_service.hello().await)
}