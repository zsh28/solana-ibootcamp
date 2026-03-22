// Module 3: Functions and Control Flow
// Task 3: Repeating Code
// Rust provides three looping constructs:
// - loop runs indefinitely until you break
// - while runs as long as a condition is true
// - for iterates over ranges or collections
//
// The for loop with ranges is the most common:
// for i in 1..=10 {
//     // runs for i = 1, 2, ..., 10
// }
//
// Your Task:
// Write a function factorial(n: u64) -> u64 that computes the factorial of n.
// Recall: 0! = 1, and 5! = 1 x 2 x 3 x 4 x 5 = 120.
pub fn factorial(n: u64) -> u64 {
    // Start with 1 because multiplication identity is 1.
    let mut result = 1;

    // Multiply result by every number from 1 up to n (inclusive).
    // eg 1 * 2 * 3 * 4 * 5 = 120 for n = 5.
    // n = 0 will skip the loop and return 1, which is correct for 0!.
    // n = 10  will be 1 * 2 * 3 * ... * 10 = 3628800.
    for i in 1..=n {
        result *= i;
    }

    // For n = 0, the loop does not run and result stays 1, which is correct.
    result
}

pub fn run() {
    let n = 5;
    println!("factorial({n}) = {}", factorial(n));
}

#[cfg(test)]
mod tests {
    use super::factorial;

    #[test]
    fn computes_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }
}
