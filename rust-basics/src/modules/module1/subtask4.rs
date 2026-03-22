// Task 4: Reusing Names with Shadowing
// Shadowing lets you declare a new variable with the same name,
// effectively replacing the previous binding.
// Unlike mut, each let creates a completely new variable.
//
// let input = "  42  ";
// let input = input.trim();
// let input: i32 = input.parse().unwrap();
//
// Your Task:
// Write a function transform(s: &str) -> i32 that:
// 1) Trims whitespace from the string
// 2) Parses the trimmed result into an i32
// 3) Returns the square of that number
// Use shadowing at each step to keep the code clean.
pub fn transform(s: &str) -> i32 {
    // Step 1: Trim whitespace from the string
    let s = s.trim();
    // Step 2: Parse the trimmed result into an i32
    let s: i32 = s.parse().unwrap();
    // Step 3: Return the square of that number
    s * s
}

pub fn run() {
    let input = " 12 ";
    println!("transform(\"{input}\") = {}", transform(input));
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn trims_parses_and_squares() {
        assert_eq!(transform(" 7 "), 49);
        assert_eq!(transform("10"), 100);
    }
}
