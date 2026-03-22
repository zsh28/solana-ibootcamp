// Module 11: Closures and Iterators
// Task 4: Making Any Type Iterable
//
// By implementing Iterator for a custom type, we can use it in for loops
// and with iterator adapters.
//
// Your Task:
// Implement Iterator for Countdown so it yields n, n-1, ..., 1, then None.
pub struct Countdown {
    pub n: i32,
}

impl Countdown {
    pub fn new(n: i32) -> Self {
        Self { n }
    }
}

impl Iterator for Countdown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // Stop iteration once we reach zero or below.
        if self.n <= 0 {
            None
        } else {
            // Return current value, then decrement internal state.
            let current = self.n;
            self.n -= 1;
            Some(current)
        }
    }
}

pub fn run() {
    let values: Vec<i32> = Countdown::new(3).collect();
    println!("Countdown::new(3) => {:?}", values);
}

#[cfg(test)]
mod tests {
    use super::Countdown;

    #[test]
    fn countdown_yields_descending_values() {
        let values: Vec<i32> = Countdown::new(5).collect();
        assert_eq!(values, vec![5, 4, 3, 2, 1]);

        let empty: Vec<i32> = Countdown::new(0).collect();
        assert_eq!(empty, Vec::<i32>::new());
    }
}
