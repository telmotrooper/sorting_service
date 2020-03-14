use crate::models::Book;

pub fn get_from_book<'b>(book: &'b Book, attribute: &str) -> &'b Option<String> {
    return match attribute {
        "title" => &book.title,
        "subtitle" => &book.subtitle,
        "author" => &book.author,
        "isbn_10" => &book.isbn_10,
        "isbn_13" => &book.isbn_13,
        _ => &book.title,
    };
}
