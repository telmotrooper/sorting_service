#![allow(dead_code, unused_imports)]

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger;

// Top level module declarations
mod models;
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8000";

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

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
