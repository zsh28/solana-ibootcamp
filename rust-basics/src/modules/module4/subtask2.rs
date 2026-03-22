// Module 4: Ownership and Borrowing
// Task 2: Borrowing Instead of Moving
// References let you access a value without taking ownership.
// The original owner keeps the data.
//
// fn measure(s: &str) -> usize {
//     s.len() // s is borrowed, not consumed
// }
//
// The & symbol creates a reference. In function signatures,
// prefer &str over &String for flexibility.
//
// Your Task:
// Write a function count_chars(s: &str) -> usize that returns
// the number of Unicode characters in the given string.
// Use .chars().count() to count individual characters properly.
pub fn count_chars(s: &str) -> usize {
    //&str is a string slice, which is a reference to a string. It does not own the data, so we can use it without taking ownership.
    // chars() iterates by Unicode scalar values, not bytes.
    // count() returns how many characters were produced.
    s.chars().count()
}

pub fn run() {
    let text = "hello";
    println!("count_chars(\"{text}\") = {}", count_chars(text));
}

#[cfg(test)]
mod tests {
    use super::count_chars;

    #[test]
    fn counts_unicode_characters() {
        assert_eq!(count_chars("rust"), 4);
        assert_eq!(count_chars("caf\u{00E9}"), 4);
        assert_eq!(count_chars("\u{0939}\u{093F}"), 2);
    }
}
