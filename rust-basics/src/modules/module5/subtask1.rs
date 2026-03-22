// Module 5: Structs and Methods
// Task 1: Creating Custom Data Types
// Structs let you define your own types by grouping named fields together.
//
// struct Product {
//     name: String,
//     price: f64,
//     in_stock: bool,
// }
//
// You create instances using the struct name followed by field values.
//
// Your Task:
// Define a struct Rectangle with two fields: width: i32 and height: i32.
// Then write a function rect_area(w: i32, h: i32) -> i32 that creates a
// Rectangle instance and returns its area.
pub struct Rectangle {
    width: i32,
    height: i32,
}

pub fn rect_area(w: i32, h: i32) -> i32 {
    // Create a Rectangle instance from input dimensions.
    let rect = Rectangle {
        width: w,
        height: h,
    };

    // Area is width multiplied by height.
    rect.width * rect.height
}

pub fn run() {
    println!("rect_area(8, 3) = {}", rect_area(8, 3));
}

#[cfg(test)]
mod tests {
    use super::rect_area;

    #[test]
    fn computes_rectangle_area() {
        assert_eq!(rect_area(4, 5), 20);
        assert_eq!(rect_area(10, 2), 20);
    }
}
