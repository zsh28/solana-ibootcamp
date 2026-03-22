// Module 10: Lifetimes
// Task 1: References That Never Expire
//
// 'static means the reference is valid for the entire program.
// String literals are stored in the program binary, so they are &'static str.
//
// Your Task:
// classify(n: i32) -> &'static str
// - "positive" when n > 0
// - "negative" when n < 0
// - "zero" when n == 0
pub fn classify(n: i32) -> &'static str {
    // Return one of three string literals.
    // Because these are literals, Rust gives them 'static lifetime.
    if n > 0 {
        "positive"
    } else if n < 0 {
        "negative"
    } else {
        "zero"
    }
}

pub fn run() {
    println!("classify(10) = {}", classify(10));
    println!("classify(-3) = {}", classify(-3));
    println!("classify(0) = {}", classify(0));
}

#[cfg(test)]
mod tests {
    use super::classify;

    #[test]
    fn classifies_sign_correctly() {
        assert_eq!(classify(7), "positive");
        assert_eq!(classify(-1), "negative");
        assert_eq!(classify(0), "zero");
    }
}
