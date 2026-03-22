// Module 12: Capstone
// Task 3: Build a Calculator
//
// Your Task:
// Write evaluate(expr: &str) -> Result<i32, String> for expressions like:
// <number> <operator> <number>
// Supported operators: +, -, *, /
// Errors:
// - "division by zero"
// - "invalid expression"
// - "unknown operator"
pub fn evaluate(expr: &str) -> Result<i32, String> {
    // Expect exactly three whitespace-separated tokens.
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("invalid expression".to_string());
    }

    // Parse operands; parse failures map to "invalid expression".
    let left = match parts[0].parse::<i32>() {
        Ok(n) => n,
        Err(_) => return Err("invalid expression".to_string()),
    };

    let right = match parts[2].parse::<i32>() {
        Ok(n) => n,
        Err(_) => return Err("invalid expression".to_string()),
    };

    // Dispatch by operator.
    match parts[1] {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "/" => {
            if right == 0 {
                Err("division by zero".to_string())
            } else {
                Ok(left / right)
            }
        }
        _ => Err("unknown operator".to_string()),
    }
}

pub fn run() {
    println!("evaluate(\"8 * 7\") = {:?}", evaluate("8 * 7"));
    println!("evaluate(\"10 / 0\") = {:?}", evaluate("10 / 0"));
}

#[cfg(test)]
mod tests {
    use super::evaluate;

    #[test]
    fn evaluates_valid_and_invalid_expressions() {
        assert_eq!(evaluate("3 + 4"), Ok(7));
        assert_eq!(evaluate("9 - 5"), Ok(4));
        assert_eq!(evaluate("6 * 7"), Ok(42));
        assert_eq!(evaluate("8 / 2"), Ok(4));
        assert_eq!(evaluate("8 / 0"), Err("division by zero".to_string()));
        assert_eq!(evaluate("8 ^ 2"), Err("unknown operator".to_string()));
        assert_eq!(evaluate("bad + 2"), Err("invalid expression".to_string()));
        assert_eq!(evaluate("1 +"), Err("invalid expression".to_string()));
    }
}
