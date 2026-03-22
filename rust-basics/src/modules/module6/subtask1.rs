// Module 6: Enums and Pattern Matching
// Task 1: The match Expression
// match compares a value against patterns and runs the first match.
// Every possible case must be handled.
//
// Your Task:
// Write coin_value(coin: &str) -> i32 that returns:
// - "penny" -> 1
// - "nickel" -> 5
// - "dime" -> 10
// - "quarter" -> 25
// - anything else -> 0
pub fn coin_value(coin: &str) -> i32 {
    // match checks each pattern from top to bottom.
    // _ is the wildcard for all remaining values.
    match coin {
        "penny" => 1,
        "nickel" => 5,
        "dime" => 10,
        "quarter" => 25,
        _ => 0,
    }
}

pub fn run() {
    let coin = "dime";
    println!("coin_value(\"{coin}\") = {}", coin_value(coin));
}

#[cfg(test)]
mod tests {
    use super::coin_value;

    #[test]
    fn maps_coin_names_to_values() {
        assert_eq!(coin_value("penny"), 1);
        assert_eq!(coin_value("nickel"), 5);
        assert_eq!(coin_value("dime"), 10);
        assert_eq!(coin_value("quarter"), 25);
        assert_eq!(coin_value("rupee"), 0);
    }
}
