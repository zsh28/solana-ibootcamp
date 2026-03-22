// Module 8: Error Handling
// Task 3: Putting Error Handling Together
//
// Your Task:
// Write parse_csv_sum(s: &str) -> Result<i32, String> that:
// 1) Takes a comma-separated string of numbers
// 2) Parses each number as i32
// 3) Returns Ok(sum)
// 4) Returns Err("empty input") if the string is empty
// 5) Returns Err("invalid number: X") if any value X cannot be parsed
pub fn parse_csv_sum(s: &str) -> Result<i32, String> {
    // Reject empty or whitespace-only input early.
    if s.trim().is_empty() {
        return Err("empty input".to_string());
    }

    let mut sum = 0;

    // Split on commas and parse each part.
    for raw in s.split(',') {
        let value = raw.trim();

        // Use ? to return early with a descriptive error.
        let n = value
            .parse::<i32>()
            .map_err(|_| format!("invalid number: {}", value))?;

        sum += n;
    }

    Ok(sum)
}

pub fn run() {
    println!("parse_csv_sum(\"1,2,3\") = {:?}", parse_csv_sum("1,2,3"));
    println!("parse_csv_sum(\"1,x,3\") = {:?}", parse_csv_sum("1,x,3"));
}

#[cfg(test)]
mod tests {
    use super::parse_csv_sum;

    #[test]
    fn parses_csv_and_sums_values() {
        assert_eq!(parse_csv_sum("1,2,3"), Ok(6));
        assert_eq!(parse_csv_sum(""), Err("empty input".to_string()));
        assert_eq!(
            parse_csv_sum("1,two,3"),
            Err("invalid number: two".to_string())
        );
    }
}
