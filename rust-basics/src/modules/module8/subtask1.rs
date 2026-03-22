// Module 8: Error Handling
// Task 1: The Result Type
// Result<T, E> has two variants:
// - Ok(value) for success
// - Err(error) for failure
//
// Your Task:
// Write parse_number(s: &str) -> Result<i32, String> that:
// - Returns Ok(n) when parsing succeeds
// - Returns Err("invalid number".to_string()) when parsing fails
pub fn parse_number(s: &str) -> Result<i32, String> {
    // Try to parse to i32.
    // Convert any parse error into the required custom String error.
    s.parse::<i32>().map_err(|_| "invalid number".to_string())
}

pub fn run() {
    println!("parse_number(\"42\") = {:?}", parse_number("42"));
    println!("parse_number(\"abc\") = {:?}", parse_number("abc"));
}

#[cfg(test)]
mod tests {
    use super::parse_number;

    #[test]
    fn parses_or_returns_invalid_number_error() {
        assert_eq!(parse_number("123"), Ok(123));
        assert_eq!(parse_number("oops"), Err("invalid number".to_string()));
    }
}
