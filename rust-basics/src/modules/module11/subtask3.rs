// Module 11: Closures and Iterators
// Task 3: Filtering Data
//
// filter() keeps elements matching a predicate.
// Because iter() yields references, we convert to owned i32 with copied().
//
// Your Task:
// Write evens_only(nums: &[i32]) -> Vec<i32>.
pub fn evens_only(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        // Keep values divisible by 2.
        .filter(|n| **n % 2 == 0)
        // Convert &i32 to i32.
        .copied()
        .collect()
}

pub fn run() {
    let input = [1, 2, 3, 4, 5, 6];
    println!("evens_only([1, 2, 3, 4, 5, 6]) = {:?}", evens_only(&input));
}

#[cfg(test)]
mod tests {
    use super::evens_only;

    #[test]
    fn keeps_only_even_numbers() {
        assert_eq!(evens_only(&[1, 2, 3, 4, 5]), vec![2, 4]);
        assert_eq!(evens_only(&[7, 9]), Vec::<i32>::new());
    }
}
