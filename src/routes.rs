use crate::models;

use actix_web::web::Json;
use actix_web::{error, get, post, HttpResponse, Responder, Result};

#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[post("/books")]
pub async fn sort_books(input: Json<models::SortingInput>) -> Result<String> {
    for book in input.books.iter() {
        if book.title.is_none() || book.author.is_none() {
            return Err(error::ErrorBadRequest("Invalid book provided."));
        }
    }

    // e.g. ["title desc", " author asc"]
    let rules: Vec<&str> = input.order_by.split(",").collect();

    // e.g. [["title", "desc"], ["author", "asc"]]
    let processed_rules: Vec<Vec<&str>> = rules
        .iter()
        .map(|rule| rule.split(" ").filter(|text| *text != "").collect())
        .collect();

    // TODO: multi-level sorting

    let title = input.books[0].title.as_ref().unwrap();
    let author = input.books[0].author.as_ref().unwrap();
    Ok(format!("The book is {:?}, written by {:?}.", title, author))
}
