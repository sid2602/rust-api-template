mod config;
mod model;
mod api;
mod services;

use std::net::TcpListener;

use actix_web::{ web, App, HttpServer, middleware::Logger};
use config::db::DbConnection;

extern crate dotenv;
use dotenv::dotenv;

use api::user::{
    get_users
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let db = DbConnection::new().await;
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to create listener");

    std::env::set_var("RUST_LOG", "DEBUG");
    std::env::set_var("RUST_BACKTRACE", "1");

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(web::Data::new(db.pool.clone()))
            .service(get_users)
    })
    .listen(listener)?
    .run()
    .await
}