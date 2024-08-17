mod controller;
mod context;
mod service;

use std::sync::Arc;
use crate::context::AppContext;
use crate::controller::hello::setup as helloControllerSetup;
use actix_web::{middleware, HttpServer};
use actix_web::App;
use actix_web::web;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        let state = AppContext::new();
        App::new().app_data(web::Data::new(state))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(helloControllerSetup)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}