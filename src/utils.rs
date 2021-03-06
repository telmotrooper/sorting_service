use crate::models::Book;

pub fn get_value<'b>(book: &'b Book, attribute: &str) -> &'b Option<String> {
    return match attribute {
        "title" => &book.title,
        "subtitle" => &book.subtitle,
        "author" => &book.author,
        "edition_year" => &book.edition_year,
        _ => &book.title,
    };
}
