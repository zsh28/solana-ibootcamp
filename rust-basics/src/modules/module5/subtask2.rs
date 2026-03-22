// Module 5: Structs and Methods
// Task 2: Methods on Structs
// Use impl blocks to define methods that operate on your struct's data.
// Methods take &self as their first parameter to access the instance.
//
// impl Circle {
//     fn circumference(&self) -> f64 {
//         2.0 * std::f64::consts::PI * self.radius
//     }
// }
//
// Your Task:
// For Rectangle, implement two methods:
// - area(&self) -> i32
// - is_square(&self) -> bool
pub struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn area(&self) -> i32 {
        // Read fields through shared reference and compute area.
        self.width * self.height
    }

    pub fn is_square(&self) -> bool {
        // A rectangle is a square when both sides are equal.
        self.width == self.height
    }
}

pub fn run() {
    let rect = Rectangle {
        width: 5,
        height: 5,
    };
    println!("area = {}", rect.area());
    println!("is_square = {}", rect.is_square());
}

#[cfg(test)]
mod tests {
    use super::Rectangle;

    #[test]
    fn area_and_square_check_work() {
        let a = Rectangle {
            width: 3,
            height: 7,
        };
        assert_eq!(a.area(), 21);
        assert!(!a.is_square());

        let b = Rectangle {
            width: 4,
            height: 4,
        };
        assert_eq!(b.area(), 16);
        assert!(b.is_square());
    }
}
