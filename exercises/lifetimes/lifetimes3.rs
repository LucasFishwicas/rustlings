// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a hint.

// The struct Book holds borrowed values, meaning the values it holds were created
// before an instance of Book is created, and may or may not die (go out of scope)
// before Book does - resulting in a dangling pointer.
// So we should explicitly tell the compiler that the values going into Book live
// atleast as long as Book does
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
