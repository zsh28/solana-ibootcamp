// Module 8: Error Handling
// Task 2: Concise Error Handling
// The ? operator returns early when it sees Err(...).
//
// Your Task:
// Write add_parsed(a: &str, b: &str) -> Result<i32, String> that:
// 1) Parses both strings into i32 (map parse errors to "parse error")
// 2) Returns Ok(sum)
pub fn add_parsed(a: &str, b: &str) -> Result<i32, String> {
    // Parse first input or return Err("parse error") immediately.
    let left = a.parse::<i32>().map_err(|_| "parse error".to_string())?;

    // Parse second input or return Err("parse error") immediately.
    let right = b.parse::<i32>().map_err(|_| "parse error".to_string())?;

    Ok(left + right)
}

pub fn run() {
    println!("add_parsed(\"10\", \"32\") = {:?}", add_parsed("10", "32"));
    println!("add_parsed(\"10\", \"x\") = {:?}", add_parsed("10", "x"));
}

#[cfg(test)]
mod tests {
    use super::add_parsed;

    #[test]
    fn adds_when_both_values_parse() {
        assert_eq!(add_parsed("7", "8"), Ok(15));
        assert_eq!(add_parsed("7", "bad"), Err("parse error".to_string()));
    }
}
