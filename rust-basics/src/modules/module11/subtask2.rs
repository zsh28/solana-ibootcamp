// Module 11: Closures and Iterators
// Task 2: Functional Data Processing
//
// Iterator chains let us transform and reduce data without manual loops.
//
// Your Task:
// Write sum_of_squares(nums: &[i32]) -> i32.
pub fn sum_of_squares(nums: &[i32]) -> i32 {
    // map squares each element, sum reduces to one total value.
    nums.iter().map(|n| *n * *n).sum()
}

pub fn run() {
    let input = [1, 2, 3];
    println!("sum_of_squares([1, 2, 3]) = {}", sum_of_squares(&input));
}

#[cfg(test)]
mod tests {
    use super::sum_of_squares;

    #[test]
    fn computes_sum_of_squared_values() {
        assert_eq!(sum_of_squares(&[1, 2, 3]), 14);
        assert_eq!(sum_of_squares(&[0, -2, 4]), 20);
    }
}
