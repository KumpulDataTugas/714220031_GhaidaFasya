struct Book {
    title: String,
    pages: u32,
}

impl Book {
    fn new(title: &str, pages: u32) -> Self {
        Book {
            title: title.to_string(),
            pages,
        }
    }
}

fn main() {
    let book = Book::new("Rust 101", 300);
    println!("Book: {}, {} pages", book.title, book.pages);
}