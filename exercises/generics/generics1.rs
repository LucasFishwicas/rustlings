// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.


// The type is a string slice <&str> because "milk" (quoted string, no functions)
// was the value that was passed
fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
