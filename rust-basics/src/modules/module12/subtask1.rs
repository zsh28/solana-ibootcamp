use std::collections::HashMap;

// Module 12: Capstone
// Task 1: Text Analysis
//
// Your Task:
// Write most_frequent(s: &str) -> String that:
// - Splits by whitespace
// - Counts words case-insensitively (lowercase)
// - On ties, returns the word that appears first in the input
// - Assumes input is non-empty
pub fn most_frequent(s: &str) -> String {
    // Count occurrences of each lowercase word.
    let mut counts: HashMap<String, i32> = HashMap::new();

    // Track the best answer while scanning left-to-right.
    // This naturally keeps the first word in case of ties.
    let mut best_word = String::new();
    let mut best_count = 0;

    for raw in s.split_whitespace() {
        // Normalize so "Rust" and "rust" count as the same word.
        let word = raw.to_lowercase();

        // Increment this word's frequency and get updated count.
        let count = counts.entry(word.clone()).or_insert(0);
        *count += 1;

        // Update winner only when strictly better.
        // If count ties best_count, we keep existing best_word,
        // preserving the earliest appearance rule.
        if *count > best_count {
            best_count = *count;
            best_word = word;
        }
    }

    best_word
}

pub fn run() {
    let text = "Rust is fast and rust is safe";
    println!("most_frequent = {}", most_frequent(text));
}

#[cfg(test)]
mod tests {
    use super::most_frequent;

    #[test]
    fn finds_most_frequent_with_tie_by_first_appearance() {
        assert_eq!(most_frequent("a b a c"), "a");
        assert_eq!(most_frequent("Rust rust RUST go"), "rust");
        assert_eq!(most_frequent("go rust go rust"), "go");
    }
}
