// Module 12: Capstone
// Task 2: 2D Data Processing
//
// Your Task:
// Write diagonal_sum(input: &str) -> i32 that:
// 1) Parses a square matrix from rows separated by ';'
// 2) Sums primary and secondary diagonals
// 3) Subtracts center once for odd dimension matrices
pub fn diagonal_sum(input: &str) -> i32 {
    // Parse rows into Vec<Vec<i32>>.
    let matrix: Vec<Vec<i32>> = input
        .split(';')
        .map(|row| {
            row.split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let n = matrix.len();
    let mut total = 0;

    // Add both diagonals in one pass.
    for i in 0..n {
        // Primary diagonal element: (i, i)
        total += matrix[i][i];

        // Secondary diagonal element: (i, n - 1 - i)
        total += matrix[i][n - 1 - i];
    }

    // For odd n, center appears in both diagonals; remove one duplicate.
    if n % 2 == 1 {
        let mid = n / 2;
        total -= matrix[mid][mid];
    }

    total
}

pub fn run() {
    let input = "1 2 3;4 5 6;7 8 9";
    println!("diagonal_sum = {}", diagonal_sum(input));
}

#[cfg(test)]
mod tests {
    use super::diagonal_sum;

    #[test]
    fn computes_diagonal_sum_with_center_correction() {
        assert_eq!(diagonal_sum("1 2 3;4 5 6;7 8 9"), 25);
        assert_eq!(diagonal_sum("1 2;3 4"), 10);
    }
}
