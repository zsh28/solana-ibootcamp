use std::collections::HashSet;

// Module 7: Collections
// Task 3: Hash-Based Collections
// HashSet<T> stores only unique values.
//
// Your Task:
// Write unique_word_count(s: &str) -> usize that returns
// the number of distinct words in the input string.
pub fn unique_word_count(s: &str) -> usize {
    // Split text into words by whitespace.
    // Collect into a HashSet to remove duplicates automatically.
    let unique: HashSet<&str> = s.split_whitespace().collect();

    // The set length equals the distinct word count.
    unique.len()
}

pub fn run() {
    let text = "rust is fast and rust is safe";
    println!("unique_word_count = {}", unique_word_count(text));
}

#[cfg(test)]
mod tests {
    use super::unique_word_count;

    #[test]
    fn counts_distinct_words() {
        assert_eq!(unique_word_count("a b c"), 3);
        assert_eq!(unique_word_count("hello hello world"), 2);
        assert_eq!(unique_word_count(""), 0);
    }
}
