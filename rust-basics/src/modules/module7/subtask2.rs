// Module 7: Collections
// Task 2: Owned vs Borrowed Strings
// String is owned and growable; &str is a borrowed string slice.
//
// Your Task:
// Write count_vowels(s: &str) -> usize that counts vowels
// (a, e, i, o, u) regardless of case.
pub fn count_vowels(s: &str) -> usize {
    // Normalize to lowercase so matching is case-insensitive.
    let lower = s.to_lowercase();

    // Count characters that are vowels.
    lower
        .chars()
        .filter(|ch| match *ch {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        })
        .count()
}

pub fn run() {
    let text = "Hello, World!";
    println!("count_vowels(\"{text}\") = {}", count_vowels(text));
}

#[cfg(test)]
mod tests {
    use super::count_vowels;

    #[test]
    fn counts_vowels_ignoring_case() {
        assert_eq!(count_vowels("Rust"), 1);
        assert_eq!(count_vowels("AEIOU"), 5);
        assert_eq!(count_vowels("xyz"), 0);
    }
}
