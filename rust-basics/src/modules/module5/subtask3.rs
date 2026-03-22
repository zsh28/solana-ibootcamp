// Module 5: Structs and Methods
// Task 3: Associated Functions as Constructors
// Functions inside an impl block that do not take self are associated
// functions, commonly used as constructors.
//
// impl Circle {
//     fn new(radius: f64) -> Self {
//         Self { radius }
//     }
// }
//
// Your Task:
// Implement two functions for Rectangle:
// - square(size: i32) -> Self
// - area(&self) -> i32
pub struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn square(size: i32) -> Self {
        // Build a Rectangle with equal width and height.
        //Self is an alias for the type we are implementing, so we can return Self instead of Rectangle.
        Self {
            width: size,
            height: size,
        }
    }

    pub fn area(&self) -> i32 {
        // &self is an immutable reference to the Rectangle instance.
        // Compute area from the current dimensions.
        self.width * self.height
    }
}

pub fn run() {
    let sq = Rectangle::square(6);
    println!("Rectangle::square(6).area() = {}", sq.area());
}

#[cfg(test)]
mod tests {
    use super::Rectangle;

    #[test]
    fn creates_square_and_computes_area() {
        let sq = Rectangle::square(9);
        assert_eq!(sq.area(), 81);
    }
}
