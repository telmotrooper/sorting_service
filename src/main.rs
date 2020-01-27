#![allow(
    dead_code
)]

use actix_web::{App, HttpResponse, HttpServer, Result, Responder, get, post, web};
use serde::Deserialize;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8000";
    println!("Running web server on http://{}", address);
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(read_book)
    })
        .bind(address)?
        .run()
        .await
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[post("/book")]
async fn read_book(book: web::Json<Book>) -> Result<String> {
    let title = (book.title).as_ref().unwrap();
    Ok(format!("The book is {:?}, by {:?}", title, book.author))
}


#[derive(Deserialize)]
struct Book {
    title: Option<String>,
    subtitle: Option<String>,
    author: Option<String>,
    isbn_10: Option<String>,
    isbn_13: Option<String>
}
