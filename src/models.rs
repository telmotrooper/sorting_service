use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SortingInput {
    pub order_by: String,
    pub books: Vec<Book>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Book {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub author: Option<String>,
    pub edition_year: Option<String>,
}
