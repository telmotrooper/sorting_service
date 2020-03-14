use serde::Deserialize;

#[derive(Deserialize)]
pub struct SortingInput {
    pub order_by: String,
    pub books: Vec<Book>,
}

#[derive(Deserialize)]
pub struct Book {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub author: Option<String>,
    pub isbn_10: Option<String>,
    pub isbn_13: Option<String>,
}
