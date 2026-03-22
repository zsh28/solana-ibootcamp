// Module 9: Generics and Traits
// Task 3: The Derive Attribute
//
// Derive gives us PartialEq and Clone automatically.
// - PartialEq enables == and !=
// - Clone enables explicit duplication with .clone()
#[derive(PartialEq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn are_equal(a: &Point, b: &Point) -> bool {
    // Because Point derives PartialEq, we can compare directly.
    a == b
}

pub fn distance_sq(a: &Point, b: &Point) -> i32 {
    // Squared Euclidean distance:
    // (x2 - x1)^2 + (y2 - y1)^2
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    dx * dx + dy * dy
}

pub fn run() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: 6 };
    println!("are_equal = {}", are_equal(&p1, &p2));
    println!("distance_sq = {}", distance_sq(&p1, &p2));
}

#[cfg(test)]
mod tests {
    use super::{are_equal, distance_sq, Point};

    #[test]
    fn compares_points_and_computes_distance() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 3, y: 4 };
        let c = b.clone();

        assert!(!are_equal(&a, &b));
        assert!(are_equal(&b, &c));
        assert_eq!(distance_sq(&a, &b), 25);
    }
}
