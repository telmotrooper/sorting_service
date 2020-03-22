use crate::models::{Book, SortingInput};
use crate::utils::get_from_book;

use actix_web::web::Json;
use actix_web::{error, get, post, HttpResponse, Responder, Result};

#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[post("/books")]
pub async fn sort_books(mut input: Json<SortingInput>) -> impl Responder {
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

    for rule in processed_rules {
        println!("[{},{}]", rule[0], rule[1]);

        for book in input.books.iter() {
            let value = get_from_book(book, rule[0]);

            if let Some(v) = value {
                println!("{}", v);
            }
        }
    }

    // TODO: Remove this (only here for testing)
    &input.books.sort();

    // let title = input.books[0].title.as_ref().unwrap();
    // let author = input.books[0].author.as_ref().unwrap();

    HttpResponse::Ok().json(&input.books).await
}
