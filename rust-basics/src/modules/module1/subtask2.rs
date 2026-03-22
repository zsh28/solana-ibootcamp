// Task 2: Binding Values to Names
// In Rust, you create variables using let.
// The compiler infers types automatically, though you can specify them explicitly.
// By default, Rust variables are immutable.
//
// let age = 25;        // type inferred as i32
// let pi: f64 = 3.14;  // type explicitly set
//
// To print variables, use {} as a placeholder inside println!:
// println!("age = {}", age);
//
// Your Task:
// Create two variables:
// x set to 42
// name set to "Rust"
// Then print them so the output is exactly:
// x = 42
// name = Rust
pub fn run() {
    let x = 42;
    let name = "Rust";

    println!("x = {x}");
    println!("name = {name}");
}
