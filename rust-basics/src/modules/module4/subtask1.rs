// Module 4: Ownership and Borrowing
// Task 1: Rust's Ownership System
// Ownership is Rust's most distinctive feature. The rules are simple:
// - Every value has exactly one owner
// - When the owner goes out of scope, the value is automatically cleaned up
// - Assigning a heap value (like String) to another variable moves it
//
// let greeting = String::from("hi");
// let copy = greeting; // greeting is moved to copy
//
// Your Task:
// Write a function join_strings(a: String, b: String) -> String that
// takes ownership of both strings and returns them concatenated with
// a space between.
pub fn join_strings(a: String, b: String) -> String {
    // a and b are owned by this function, so we are free to consume them.
    // format! builds a new String in the shape "a b".
    format!("{} {}", a, b)
}

pub fn run() {
    let first = String::from("Hello");
    let second = String::from("Rust");
    println!("join_strings = {}", join_strings(first, second));
}

#[cfg(test)]
mod tests {
    use super::join_strings;

    #[test]
    fn joins_with_space() {
        assert_eq!(
            join_strings(String::from("good"), String::from("day")),
            "good day"
        );
    }
}
