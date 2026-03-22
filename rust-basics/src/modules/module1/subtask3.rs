// Task 3: Opting into Mutability
// Rust makes variables immutable by default.
// When you need a variable that can be updated, add mut.
//
// let mut total = 0;
// total += 10; // allowed because total is mutable
//
// Attempting to reassign without mut will cause a compile error.
//
// Your Task:
// Write a function sum_to(n: i32) -> i32 that computes
// the sum of all integers from 1 to n (inclusive).
// You'll need a mutable accumulator to build up the result.
pub fn sum_to(n: i32) -> i32 {
    let mut total = 0;
    let mut current = 1;

    while current <= n {
        total += current;
        current += 1;
    }

    total
}

pub fn run() {
    let n = 10;
    println!("sum_to({n}) = {}", sum_to(n));
}

#[cfg(test)]
mod tests {
    use super::sum_to;

    #[test]
    fn sums_to_n() {
        assert_eq!(sum_to(5), 15);
        assert_eq!(sum_to(1), 1);
    }
}
