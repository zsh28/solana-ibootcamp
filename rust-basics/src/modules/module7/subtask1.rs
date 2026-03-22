// Module 7: Collections
// Task 1: The Vec Type
// Vec<T> is Rust's growable array type.
//
// let mut items = vec![10, 20, 30];
// items.push(40);
// let count = items.len();
//
// Your Task:
// Write sum_vec(nums: &[i32]) -> i32 that returns
// the sum of all elements in the slice.
pub fn sum_vec(nums: &[i32]) -> i32 {
    // Start an accumulator at zero.
    let mut total = 0;

    // Borrow each element from the slice and add it to total.
    for num in nums {
        total += *num;
    }

    total
}

pub fn run() {
    let data = vec![10, 20, 30, 40];
    println!("sum_vec([10, 20, 30, 40]) = {}", sum_vec(&data));
}

#[cfg(test)]
mod tests {
    use super::sum_vec;

    #[test]
    fn sums_all_elements() {
        assert_eq!(sum_vec(&[1, 2, 3, 4]), 10);
        assert_eq!(sum_vec(&[]), 0);
    }
}
