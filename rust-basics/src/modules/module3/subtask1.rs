// Module 3: Functions and Control Flow
// Task 1: Defining Functions
// In Rust, functions are declared with fn.
// Every parameter needs a type annotation, and you specify the return type after ->.
//
// fn add(a: i32, b: i32) -> i32 {
//     a + b // the last expression (no semicolon) is the return value
// }
//
// You can also use the return keyword for early returns.
//
// Your Task:
// Write a function max_of_three(a: i32, b: i32, c: i32) -> i32
// that returns the largest of the three input values.
pub fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    // Start by assuming "a" is the largest value.
    let mut max = a;

    // If "b" is larger than the current max, update max.
    if b > max {
        max = b;
    }

    // If "c" is larger than the current max, update max.
    if c > max {
        max = c;
    }

    // Return the final largest value.
    max
}

pub fn run() {
    let a = 12;
    let b = 30;
    let c = 25;
    println!("max_of_three({a}, {b}, {c}) = {}", max_of_three(a, b, c));
}

#[cfg(test)]
mod tests {
    use super::max_of_three;

    #[test]
    fn returns_largest_value() {
        assert_eq!(max_of_three(1, 2, 3), 3);
        assert_eq!(max_of_three(9, 4, 7), 9);
        assert_eq!(max_of_three(-5, -2, -8), -2);
    }
}
