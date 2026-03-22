// Module 4: Ownership and Borrowing
// Task 3: Mutable Borrowing
// When you need to modify data through a reference, use &mut.
//
// fn increment(n: &mut i32) {
//     *n += 1; // dereference with * to modify the original
// }
//
// Rust enforces: either one mutable reference OR many immutable references,
// but never both at the same time.
//
// Your Task:
// Write a function double_all(nums: &mut Vec<i32>) that multiplies
// every element in the vector by 2, modifying it in place.
pub fn double_all(nums: &mut Vec<i32>) {
    // &mut Vec<i32> is a mutable borrow.
    // We can modify the caller's vector, but the caller still owns it.
    // Iterate mutably so each element can be changed in place.
    for value in nums.iter_mut() {
        // value is &mut i32, so dereference to update the actual number.
        *value *= 2;
    }
}

pub fn run() {
    let mut nums = vec![1, 2, 3, 4];
    double_all(&mut nums);
    println!("double_all([1, 2, 3, 4]) = {:?}", nums);
}

#[cfg(test)]
mod tests {
    use super::double_all;

    #[test]
    fn doubles_every_element() {
        let mut nums = vec![2, 5, 8];
        double_all(&mut nums);
        assert_eq!(nums, vec![4, 10, 16]);
    }
}
