#![allow(dead_code, unused_imports)]

use actix_web::middleware::{Compress, Logger};
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
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(routes::home)
            .service(routes::sort_books)
    })
    .bind(address)?
    .run()
    .await
}
