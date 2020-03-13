#![allow(dead_code)]

// Top level module declarations
mod models;
mod routes;

use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8000";
    println!("Running web server on http://{}", address);
    HttpServer::new(|| App::new().service(routes::home).service(routes::read_book))
        .bind(address)?
        .run()
        .await
}
