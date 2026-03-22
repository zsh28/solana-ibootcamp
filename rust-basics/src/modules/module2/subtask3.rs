// Module 2: Data Types
// Subtask 3: Fixed-Size Collections
// Arrays hold a fixed number of elements, all of the same type.
// Access elements by zero-based index:
//
// let data = [10, 20, 30];
// let first = data[0];     // 10
// let count = data.len();  // 3
//
// A slice (&[i32]) is a view into a contiguous sequence - it works with both arrays and vectors.
//
// Your Task:
// Write a function first_last(nums: &[i32]) -> (i32, i32)
// that returns a tuple of the first and last elements from the input slice.
pub fn first_last(nums: &[i32]) -> (i32, i32) {
    // Ensure the slice is not empty to avoid panicking
    let first = nums[0];
    // Access the last element using the length of the slice
    let last = nums[nums.len() - 1];
    // Return the tuple with the first and last elements
    (first, last)
}

pub fn run() {
    let data = [10, 20, 30, 40];
    let (first, last) = first_last(&data);
    println!("first_last([10, 20, 30, 40]) = ({first}, {last})");
}

#[cfg(test)]
mod tests {
    use super::first_last;

    #[test]
    fn returns_first_and_last() {
        assert_eq!(first_last(&[5, 6, 7]), (5, 7));
        assert_eq!(first_last(&[42]), (42, 42));
    }
}
