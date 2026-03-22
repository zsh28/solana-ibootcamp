// Module 11: Closures and Iterators
// Task 1: Inline Functions
//
// Closures use |params| body syntax and are heavily used with iterators.
//
// Your Task:
// Write double_all(nums: &[i32]) -> Vec<i32>
// using .iter().map(...).collect().
pub fn double_all(nums: &[i32]) -> Vec<i32> {
    // 1) .iter() gives shared references (&i32)
    // 2) map closure dereferences each value and multiplies by 2
    // 3) collect() gathers transformed items into a new Vec<i32>
    nums.iter().map(|n| *n * 2).collect()
}

pub fn run() {
    let input = [1, 2, 3, 4];
    println!("double_all([1, 2, 3, 4]) = {:?}", double_all(&input));
}

#[cfg(test)]
mod tests {
    use super::double_all;

    #[test]
    fn doubles_every_value() {
        assert_eq!(double_all(&[2, 5, 8]), vec![4, 10, 16]);
        assert_eq!(double_all(&[]), Vec::<i32>::new());
    }
}
