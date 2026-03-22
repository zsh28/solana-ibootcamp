// Module 2: Data Types
// Subtask 2: Grouping Values with Tuples
// A tuple lets you combine values of different types into a single compound value.
// You can pull them apart with destructuring or access by index:
//
// let pair = (10, "hello");
// let (num, text) = pair; // destructure
// let first = pair.0;     // index access
//
// Tuples are especially useful for returning multiple values from a function.
//
// Your Task:
// Write a function swap(a: i32, b: i32) -> (i32, i32)
// that returns the two values in reversed order.
pub fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

pub fn run() {
    let a = 10;
    let b = 20;
    let (x, y) = swap(a, b);
    println!("swap({a}, {b}) = ({x}, {y})");
}

#[cfg(test)]
mod tests {
    use super::swap;

    #[test]
    fn swaps_values() {
        assert_eq!(swap(1, 2), (2, 1));
        assert_eq!(swap(-4, 9), (9, -4));
    }
}
