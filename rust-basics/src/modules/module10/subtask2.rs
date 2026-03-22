// Module 10: Lifetimes
// Task 2: Why Lifetimes Exist
//
// When returning a reference, Rust must know which input reference it comes from.
// The lifetime annotation 'a connects both input references and the output.
//
// Your Task:
// longest<'a>(x: &'a str, y: &'a str) -> &'a str
// Return the longer string; when equal length, return x.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Compare lengths in bytes (sufficient for "longer string" requirement).
    // If equal, choose x as requested.
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

pub fn run() {
    let a = "rust";
    let b = "lifetimes";
    println!("longest(\"{}\", \"{}\") = {}", a, b, longest(a, b));
}

#[cfg(test)]
mod tests {
    use super::longest;

    #[test]
    fn returns_longer_or_first_when_equal() {
        assert_eq!(longest("abc", "abcd"), "abcd");
        assert_eq!(longest("same", "size"), "same");
    }
}
