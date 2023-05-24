// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        // If the first char = Option<None> then return an empty String
        None => String::new(),
        // Follow the intuition of extracting first from the Some(first) arm
        // and making it uppercase with the to_uppercase() method before
        // concatenating it to the rest of c.
        // This was doable thanks to the .as_str() that we can apparently apply
        // to an Iterator of chars (maybe of anything?)
        Some(first) => first.to_string().to_uppercase() + c.as_str(),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // Convert words into an Iterator in order to apply the capitalize_first()
    // function to all items using .map() Iterator method
    // store modified words in new Vector using .collect() and return it
    words.iter()
        .map(|w| capitalize_first(w))
        .collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // Use the capitalize_words_vector() function along with the .join() method
    // - which can be used on a Vector of Strings (the return type of the fn) -
    // which creates a String by concatenating all the vecs items
    //capitalize_words_vector(words).join("")
    

    // Second solution (from egghead.io)
    // Utilise the same solution for capitalize_words_vector() but include a
    // "turbofish". This seems to modify the return type of collect(), making
    // it return a string rather than a Vector.  Cool
    words.iter()
        .map(|w| capitalize_first(w))
        .collect::<String>()

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
