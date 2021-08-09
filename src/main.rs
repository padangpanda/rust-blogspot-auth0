#[macro_use]
extern crate diesel;
extern crate actix_cors;

use actix_web::{http, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_cors::Cors;

mod routes;
mod errors;
mod models;
mod schema;
mod controllers;
mod helpers;

use routes::router_account;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default() // allowed_origin return access-control-allow-origin: * by default
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("http://localhost:3000")
                .send_wildcard()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600))
            .data(pool.clone())
            .wrap(actix_web::middleware::Logger::default())
            .route("/register", web::post().to(router_account::register))
            .route("/login", web::post().to(router_account::login))
    })
    .bind("127.0.0.1:8008")?
    .run()
    .await
}