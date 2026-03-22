// Module 6: Enums and Pattern Matching
// Task 3: Rich Enums and Pattern Matching
// match with range patterns makes numeric branching concise.
//
// Your Task:
// Write get_grade(score: i32) -> &'static str that maps:
// - 90..=100 -> "A"
// - 80..=89  -> "B"
// - 70..=79  -> "C"
// - 60..=69  -> "D"
// - below 60 -> "F"
pub fn get_grade(score: i32) -> &'static str {
    // Range patterns match numeric intervals directly.
    // Anything not matched by A/B/C/D falls back to F.
    match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    }
}

pub fn run() {
    let score = 87;
    println!("get_grade({score}) = {}", get_grade(score));
}

#[cfg(test)]
mod tests {
    use super::get_grade;

    #[test]
    fn maps_scores_to_letter_grades() {
        assert_eq!(get_grade(95), "A");
        assert_eq!(get_grade(84), "B");
        assert_eq!(get_grade(73), "C");
        assert_eq!(get_grade(61), "D");
        assert_eq!(get_grade(40), "F");
    }
}
