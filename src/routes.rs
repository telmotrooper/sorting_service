use actix_web::{HttpResponse, Result, Responder, get, post, web};
use serde::Deserialize;

#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[post("/book")]
pub async fn read_book(book: web::Json<Book>) -> Result<String> {
    if book.title.is_none() || book.author.is_none() {
        return Ok(format!("Invalid book"));
    }
    let title = book.title.as_ref().unwrap();
    let author = book.author.as_ref().unwrap();
    Ok(format!("The book is {:?}, written by {:?}.", title, author))
}

#[derive(Deserialize)]
struct Book {
    title: Option<String>,
    subtitle: Option<String>,
    author: Option<String>,
    isbn_10: Option<String>,
    isbn_13: Option<String>
}
