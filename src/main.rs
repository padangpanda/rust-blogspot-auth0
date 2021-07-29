#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
// use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod routes;
mod errors;
mod models;
mod schema;
mod auth;
mod controllers;
mod helpers;

use routes::router;

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
        // let auth = HttpAuthentication::bearer(auth::auth::validator);
        
        App::new()
            // .wrap(auth)
            .data(pool.clone())
            .route("/users", web::get().to(router::get_users))
            .route("/users/{id}", web::get().to(router::get_user_by_id))
            .route("/register", web::post().to(router::register))
            .route("/login", web::post().to(router::login))
            .route("/users/{id}", web::delete().to(router::delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}