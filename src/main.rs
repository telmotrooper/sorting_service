#![allow(dead_code, unused_imports)]

extern crate env_logger;

// Top level module declarations
mod models;
mod routes;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let address = "127.0.0.1:8000";
    println!("Running web server on http://{}\n", address);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(routes::home)
            .service(routes::read_book)
    })
    .bind(address)?
    .run()
    .await
}
