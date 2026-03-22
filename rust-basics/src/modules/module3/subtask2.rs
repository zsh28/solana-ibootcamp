// Module 3: Functions and Control Flow
// Task 2: if Returns a Value
// Unlike many languages, Rust's if is an expression - it evaluates to a value
// that you can assign.
//
// let label = if score > 90 { "great" } else { "ok" };
//
// Both branches must produce the same type.
//
// Your Task:
// Write a function abs_value(n: i32) -> i32 that returns the absolute value of n.
// Use if as an expression to return the result directly.
pub fn abs_value(n: i32) -> i32 {
    // if is used as an expression: the chosen branch value is returned.
    if n < 0 {
        -n
    } else {
        n
    }
}

pub fn run() {
    let x = -42;
    println!("abs_value({x}) = {}", abs_value(x));
}

#[cfg(test)]
mod tests {
    use super::abs_value;

    #[test]
    fn returns_absolute_value() {
        assert_eq!(abs_value(-10), 10);
        assert_eq!(abs_value(0), 0);
        assert_eq!(abs_value(15), 15);
    }
}
