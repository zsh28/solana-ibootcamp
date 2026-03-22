// Module 6: Enums and Pattern Matching
// Task 2: No Null in Rust
// Rust uses Option<T> instead of null.
// - Some(value) means data exists
// - None means no value
//
// Your Task:
// Write safe_divide(a: i32, b: i32) -> Option<i32> that:
// - Returns Some(a / b) when b is not zero
// - Returns None when b is zero
pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    // Division by zero is invalid, so represent it with None.
    if b == 0 {
        None
    } else {
        // Valid division result is wrapped in Some(...).
        Some(a / b)
    }
}

pub fn run() {
    println!("safe_divide(20, 4) = {:?}", safe_divide(20, 4));
    println!("safe_divide(20, 0) = {:?}", safe_divide(20, 0));
}

#[cfg(test)]
mod tests {
    use super::safe_divide;

    #[test]
    fn handles_division_safely() {
        assert_eq!(safe_divide(12, 3), Some(4));
        assert_eq!(safe_divide(7, 0), None);
    }
}
