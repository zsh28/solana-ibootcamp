// Module 3: Functions and Control Flow
// Task 4: A Programming Classic
// FizzBuzz is one of the most well-known coding exercises - it tests your ability
// to combine conditionals, modular arithmetic, and type conversion.
//
// Your Task:
// Write a function fizzbuzz(n: i32) -> String that returns:
// - "FizzBuzz" when n is divisible by both 3 and 5
// - "Fizz" when divisible by 3 only
// - "Buzz" when divisible by 5 only
// - The number itself as a String otherwise
//
// Use n.to_string() to convert an integer into an owned String.
pub fn fizzbuzz(n: i32) -> String {
    // Check divisibility by both 3 and 5 first.
    if n % 3 == 0 && n % 5 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        // Divisible by 3 only.
        "Fizz".to_string()
    } else if n % 5 == 0 {
        // Divisible by 5 only.
        "Buzz".to_string()
    } else {
        // Convert the number to String when no special rule applies.
        n.to_string()
    }
}

pub fn run() {
    let n = 15;
    println!("fizzbuzz({n}) = {}", fizzbuzz(n));
}

#[cfg(test)]
mod tests {
    use super::fizzbuzz;

    #[test]
    fn returns_correct_fizzbuzz_values() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(9), "Fizz");
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(7), "7");
    }
}
