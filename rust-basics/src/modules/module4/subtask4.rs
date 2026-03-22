// Module 4: Ownership and Borrowing
// Task 4: Slicing into Strings
// A slice is a reference to a contiguous portion of data.
// For strings, you can create slices with range syntax or use built-in methods.
//
// let text = String::from("hello world");
// let first_five = &text[0..5]; // "hello"
//
// Methods like .split_whitespace() and .find() provide safer ways to work
// with string data.
//
// Your Task:
// Write a function first_word(s: &str) -> String that extracts and returns
// the first whitespace-separated word from the input.
// If there are no spaces, return the entire string.
pub fn first_word(s: &str) -> String {
    // split_whitespace() yields each word separated by any whitespace.
    // next() gets the first word if it exists.
    if let Some(word) = s.split_whitespace().next() {
        // Convert &str to owned String for the return type.
        word.to_string()
    } else {
        // For empty or whitespace-only input, return the full original text.
        s.to_string()
    }
}

pub fn run() {
    let text = "hello world from rust";
    println!("first_word(\"{text}\") = {}", first_word(text));
}

#[cfg(test)]
mod tests {
    use super::first_word;

    #[test]
    fn returns_first_word_or_full_string() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("soloword"), "soloword");
        assert_eq!(first_word(""), "");
    }
}
