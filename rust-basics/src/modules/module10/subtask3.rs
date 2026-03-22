// Module 10: Lifetimes
// Task 3: Automatic Lifetime Inference
//
// With one input reference, elision often works automatically.
// With multiple references, explicit lifetimes are sometimes required so
// the compiler knows which input the output borrows from.
//
// Your Task:
// trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str
// - If s starts with prefix, return s without that prefix
// - Otherwise return s unchanged
pub fn trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    // starts_with verifies the prefix bytes match at the beginning.
    if s.starts_with(prefix) {
        // Return a slice that skips the prefix bytes.
        // The returned slice still points into s, so lifetime is 'a.
        &s[prefix.len()..]
    } else {
        // Return original reference when prefix is absent.
        s
    }
}

pub fn run() {
    let s = "rust-lang";
    println!(
        "trim_prefix(\"{}\", \"rust-\") = {}",
        s,
        trim_prefix(s, "rust-")
    );
    println!(
        "trim_prefix(\"{}\", \"go-\") = {}",
        s,
        trim_prefix(s, "go-")
    );
}

#[cfg(test)]
mod tests {
    use super::trim_prefix;

    #[test]
    fn trims_when_present_otherwise_returns_original() {
        assert_eq!(trim_prefix("prefix_value", "prefix_"), "value");
        assert_eq!(trim_prefix("value", "prefix_"), "value");
    }
}
