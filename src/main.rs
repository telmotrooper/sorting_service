#![allow(
    dead_code
)]

use actix_web::{App, HttpResponse, HttpServer, Result, Responder, get, web};
use serde::Deserialize;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8000";
    println!("Running web server on http://{}", address);
    HttpServer::new(|| {
        App::new()
            .service(home)
    })
        .bind(address)?
        .run()
        .await
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}


async fn read_book(book: web::Json<Book>) -> Result<String> {
    Ok(format!("The book is \"{}\', by {}", book.title, book.author))
}


#[derive(Deserialize)]
struct Book {
    title: String,
    subtitle: String,
    author: String,
    isbn_10: String,
    isbn_13: String
}
