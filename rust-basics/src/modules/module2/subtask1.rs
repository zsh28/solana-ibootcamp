// Module 2: Data Types
// Subtask 1: Rust's Primitive Types
// Rust provides a rich set of scalar types:
// - Integers come in signed (i8 through i128) and unsigned (u8 through u128) variants
// - Floats are either f32 (single precision) or f64 (double precision, the default)
// - Booleans are true or false
// - Characters (char) represent a single Unicode scalar value
//
// Your Task:
// Write a function multiply(a: i64, b: i64) -> i64
// that returns the product of its two arguments.
pub fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

pub fn run() {
    let a = 6;
    let b = 7;
    println!("multiply({a}, {b}) = {}", multiply(a, b));
}

#[cfg(test)]
mod tests {
    use super::multiply;

    #[test]
    fn multiplies_two_numbers() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(-2, 5), -10);
    }
}
