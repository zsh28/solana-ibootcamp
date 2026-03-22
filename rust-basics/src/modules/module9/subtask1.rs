// Module 9: Generics and Traits
// Task 1: Writing Reusable Code with Generics
//
// Your Task:
// Write largest(list: &[i32]) -> i32 that returns the largest element.
//
// Note:
// This concrete i32 version uses the same algorithm a generic version would use.
pub fn largest(list: &[i32]) -> i32 {
    // Assume the first element is the largest so far.
    // This function expects a non-empty slice.
    let mut current_largest = list[0];

    // Compare every element against the current largest value.
    for value in list {
        if *value > current_largest {
            current_largest = *value;
        }
    }

    // Return the maximum value discovered.
    current_largest
}

pub fn run() {
    let numbers = [8, 12, 4, 30, 19];
    println!("largest([8, 12, 4, 30, 19]) = {}", largest(&numbers));
}

#[cfg(test)]
mod tests {
    use super::largest;

    #[test]
    fn finds_largest_number() {
        assert_eq!(largest(&[1, 9, 2, 7]), 9);
        assert_eq!(largest(&[-10, -2, -50]), -2);
    }
}
