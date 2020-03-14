use crate::models;

use actix_web::web::Json;
use actix_web::{error, get, post, HttpResponse, Responder, Result};

#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[post("/books")]
pub async fn read_book(books: Json<Vec<models::Book>>) -> Result<String> {
    for book in books.iter() {
        if book.title.is_none() || book.author.is_none() {
            return Err(error::ErrorBadRequest("Invalid book provided."));
        }
    }

    let title = books[0].title.as_ref().unwrap();
    let author = books[0].author.as_ref().unwrap();
    Ok(format!("The book is {:?}, written by {:?}.", title, author))
}
