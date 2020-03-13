use crate::models;

use actix_web::{get, post, web, HttpResponse, Responder, Result};

#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[post("/book")]
pub async fn read_book(book: web::Json<models::Book>) -> Result<String> {
    if book.title.is_none() || book.author.is_none() {
        return Ok(format!("Invalid book"));
    }
    let title = book.title.as_ref().unwrap();
    let author = book.author.as_ref().unwrap();
    Ok(format!("The book is {:?}, written by {:?}.", title, author))
}
