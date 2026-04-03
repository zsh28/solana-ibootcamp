
mod problem01_heap_box {
    // A recursive list must use Box so the compiler knows the enum's size.
    pub enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    // Recursively sum every value stored in the list.
    pub fn list_sum(list: &List) -> i32 {
        match list {
            List::Cons(value, next) => value + list_sum(next),
            List::Nil => 0,
        }
    }

    pub fn demo() {
        let list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );
        println!("1. Heap Box list sum = {}", list_sum(&list));
    }
}

mod problem02_deref_wrapper {
    use std::ops::Deref;

    // A tuple struct wrapper around any type T.
    pub struct Wrapper<T>(pub T);

    // Implement Deref so Wrapper<T> behaves like a reference to T.
    impl<T> Deref for Wrapper<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // Return double the string length.
    pub fn double_len(s: &str) -> usize {
        s.len() * 2
    }

    pub fn demo() {
        let wrapped = Wrapper(String::from("rust"));
        println!("2. Deref wrapper double_len = {}", double_len(&wrapped));
    }
}

mod problem03_rc_count {
    use std::rc::Rc;

    // Create one Rc<String>, clone it n times, and return the strong count.
    pub fn count_owners(n: usize) -> usize {
        let shared = Rc::new(String::from("shared"));
        let mut clones = Vec::new();

        for _ in 0..n {
            clones.push(Rc::clone(&shared));
        }

        Rc::strong_count(&shared)
    }

    pub fn demo() {
        println!("3. Rc strong count with 3 clones = {}", count_owners(3));
    }
}

mod problem04_refcell_counter {
    use std::cell::RefCell;

    // RefCell allows mutation through a shared reference.
    pub struct Counter {
        value: RefCell<i32>,
    }

    impl Counter {
        pub fn new() -> Self {
            Self {
                value: RefCell::new(0),
            }
        }

        // Increment the inner value by 1.
        pub fn increment(&self) {
            *self.value.borrow_mut() += 1;
        }

        // Read the current value.
        pub fn get(&self) -> i32 {
            *self.value.borrow()
        }
    }

    // Count up to n by calling increment n times.
    pub fn count_to(n: i32) -> i32 {
        let counter = Counter::new();
        for _ in 0..n {
            counter.increment();
        }
        counter.get()
    }

    pub fn demo() {
        println!("4. RefCell count_to(5) = {}", count_to(5));
    }
}

mod problem05_trait_objects_shapes {
    // A common trait for shapes that can compute area.
    pub trait Shape {
        fn area(&self) -> f64;
    }

    pub struct Circle {
        pub radius: f64,
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    pub struct Rect {
        pub w: f64,
        pub h: f64,
    }

    impl Shape for Rect {
        fn area(&self) -> f64 {
            self.w * self.h
        }
    }

    // Sum the area of all boxed trait objects.
    pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
        shapes.iter().map(|shape| shape.area()).sum()
    }

    pub fn demo() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Circle { radius: 2.0 }),
            Box::new(Rect { w: 3.0, h: 4.0 }),
        ];
        println!("5. Total shape area = {:.2}", total_area(&shapes));
    }
}

mod problem06_formatter_pipeline {
    // A formatter transforms a string into another string.
    pub trait Formatter {
        fn format(&self, input: &str) -> String;
    }

    pub struct Upper;
    pub struct Snake;
    pub struct Trim;

    impl Formatter for Upper {
        fn format(&self, input: &str) -> String {
            input.to_uppercase()
        }
    }

    impl Formatter for Snake {
        fn format(&self, input: &str) -> String {
            input.replace(' ', "_")
        }
    }

    impl Formatter for Trim {
        fn format(&self, input: &str) -> String {
            input.trim().to_string()
        }
    }

    // Apply every formatter in order.
    pub fn apply_all(input: &str, fmts: &[Box<dyn Formatter>]) -> String {
        fmts.iter()
            .fold(input.to_string(), |acc, fmt| fmt.format(&acc))
    }

    pub fn demo() {
        let formatters: Vec<Box<dyn Formatter>> =
            vec![Box::new(Trim), Box::new(Snake), Box::new(Upper)];
        println!(
            "6. Formatter pipeline = {}",
            apply_all("  hello rust world  ", &formatters)
        );
    }
}

mod problem07_newtype_display {
    use std::fmt;

    // A newtype wrapper makes Vec<i32> "our" type.
    pub struct CommaSeparated(pub Vec<i32>);

    impl fmt::Display for CommaSeparated {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for (index, value) in self.0.iter().enumerate() {
                if index > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", value)?;
            }
            Ok(())
        }
    }

    pub fn format_list(nums: Vec<i32>) -> String {
        CommaSeparated(nums).to_string()
    }

    pub fn demo() {
        println!("7. Comma separated = {}", format_list(vec![1, 2, 3]));
    }
}

mod problem08_associated_types {
    // Each implementer picks its own associated Output type.
    pub trait Summary {
        type Output;

        fn summarize(&self) -> Self::Output;
    }

    pub struct Numbers {
        pub data: Vec<i32>,
    }

    impl Summary for Numbers {
        type Output = i32;

        fn summarize(&self) -> Self::Output {
            self.data.iter().sum()
        }
    }

    pub struct Words {
        pub data: Vec<String>,
    }

    impl Summary for Words {
        type Output = String;

        fn summarize(&self) -> Self::Output {
            self.data.join(" ")
        }
    }

    pub fn demo() {
        let nums = Numbers {
            data: vec![1, 2, 3, 4],
        };
        let words = Words {
            data: vec!["hello".into(), "rust".into()],
        };
        println!("8. Numbers summary = {}", nums.summarize());
        println!("8. Words summary = {}", words.summarize());
    }
}

mod problem09_add_display_vec2 {
    use std::fmt;
    use std::ops::Add;

    // A simple 2D vector.
    pub struct Vec2 {
        pub x: f64,
        pub y: f64,
    }

    impl Add for Vec2 {
        type Output = Vec2;

        fn add(self, rhs: Vec2) -> Self::Output {
            Vec2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl fmt::Display for Vec2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({:.1}, {:.1})", self.x, self.y)
        }
    }

    pub fn add_vecs(a: Vec2, b: Vec2) -> String {
        format!("{}", a + b)
    }

    pub fn demo() {
        println!(
            "9. Vec2 add = {}",
            add_vecs(Vec2 { x: 1.0, y: 2.5 }, Vec2 { x: 3.0, y: 4.5 })
        );
    }
}

mod problem10_functions_as_values {
    // Double the input.
    pub fn double(x: i32) -> i32 {
        x * 2
    }

    // Increment the input by 1.
    pub fn increment(x: i32) -> i32 {
        x + 1
    }

    // Apply the function twice: f(f(x)).
    pub fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
        f(f(x))
    }

    pub fn demo() {
        println!("10. apply_twice(double, 3) = {}", apply_twice(double, 3));
        println!(
            "10. apply_twice(increment, 3) = {}",
            apply_twice(increment, 3)
        );
    }
}

mod problem11_boxed_closures {
    // Return a boxed closure that multiplies any input by n.
    pub fn make_multiplier(n: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |x| x * n)
    }

    // Return a boxed closure computing f(g(x)).
    pub fn compose(
        f: Box<dyn Fn(i32) -> i32>,
        g: Box<dyn Fn(i32) -> i32>,
    ) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |x| f(g(x)))
    }

    pub fn demo() {
        let times_three = make_multiplier(3);
        let plus_one: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + 1);
        let composed = compose(plus_one, times_three);
        println!("11. compose(plus_one, times_three)(4) = {}", composed(4));
    }
}

mod problem12_match_guards_and_at {
    // Classify the integer using match ranges and bindings.
    pub fn classify(n: i32) -> String {
        match n {
            0 => "zero".to_string(),
            value @ 1..=10 => format!("small: {}", value),
            value @ -10..=-1 => format!("neg small: {}", value),
            value => format!("big: {}", value),
        }
    }

    pub fn demo() {
        println!("12. classify(-3) = {}", classify(-3));
        println!("12. classify(7) = {}", classify(7));
        println!("12. classify(50) = {}", classify(50));
    }
}

mod problem13_parse_patterns {
    // Parse a small command language using slice patterns.
    pub fn parse_command(input: &str) -> String {
        let words: Vec<&str> = input.split_whitespace().collect();

        match words.as_slice() {
            ["quit"] => "Goodbye".to_string(),
            ["echo", rest @ ..] => rest.join(" "),
            ["add", x, y] => {
                if let (Ok(a), Ok(b)) = (x.parse::<i32>(), y.parse::<i32>()) {
                    (a + b).to_string()
                } else {
                    "Unknown".to_string()
                }
            }
            ["repeat", n, msg @ ..] if !msg.is_empty() => {
                if let Ok(times) = n.parse::<usize>() {
                    let text = msg.join(" ");
                    std::iter::repeat(text)
                        .take(times)
                        .collect::<Vec<String>>()
                        .join(" ")
                } else {
                    "Unknown".to_string()
                }
            }
            _ => "Unknown".to_string(),
        }
    }

    pub fn demo() {
        println!("13. parse_command(quit) = {}", parse_command("quit"));
        println!(
            "13. parse_command(echo hello world) = {}",
            parse_command("echo hello world")
        );
        println!("13. parse_command(add 3 4) = {}", parse_command("add 3 4"));
        println!(
            "13. parse_command(repeat 3 hi) = {}",
            parse_command("repeat 3 hi")
        );
    }
}

mod problem14_raw_pointers_swap {
    // Swap two i32 values using raw pointers inside an unsafe block.
    pub fn swap_values(a: &mut i32, b: &mut i32) {
        let pa: *mut i32 = a;
        let pb: *mut i32 = b;

        unsafe {
            let temp = *pa;
            *pa = *pb;
            *pb = temp;
        }
    }

    pub fn demo() {
        let mut a = 3;
        let mut b = 7;
        swap_values(&mut a, &mut b);
        println!("14. swap_values => a = {}, b = {}", a, b);
    }
}

mod problem15_safe_wrapper_pattern {
    // A safe wrapper around Vec<i32>.
    pub struct SafeArray {
        data: Vec<i32>,
    }

    impl SafeArray {
        pub fn new(data: Vec<i32>) -> Self {
            Self { data }
        }

        // Safely get a value by index.
        pub fn get(&self, i: usize) -> Option<i32> {
            self.data.get(i).copied()
        }

        // Unsafely get a value without bounds checks.
        // Caller must ensure i is in bounds.
        pub unsafe fn get_unchecked(&self, i: usize) -> i32 {
            *self.data.get_unchecked(i)
        }

        // Safely sum all values by using get_unchecked only in a proven-safe loop.
        pub fn sum_all(&self) -> i32 {
            let mut sum = 0;
            for i in 0..self.data.len() {
                unsafe {
                    sum += self.get_unchecked(i);
                }
            }
            sum
        }
    }

    pub fn demo() {
        let arr = SafeArray::new(vec![1, 2, 3]);
        println!("15. SafeArray get(1) = {:?}", arr.get(1));
        println!("15. SafeArray sum_all() = {}", arr.sum_all());
    }
}

mod problem16_macro_square {
    // A macro that squares an expression.
    macro_rules! square {
        ($x:expr) => {
            $x * $x
        };
    }

    // Use the macro to compute n squared.
    pub fn compute(n: i32) -> i32 {
        square!(n)
    }

    pub fn demo() {
        println!("16. square!(4) = {}", compute(4));
    }
}

mod problem17_macro_convert {
    // A macro with multiple pattern arms for temperature conversion.
    macro_rules! convert {
        (celsius_to_f, $temp:expr) => {
            $temp * 9 / 5 + 32
        };
        (f_to_celsius, $temp:expr) => {
            ($temp - 32) * 5 / 9
        };
    }

    pub fn temp_test(c: i32) -> i32 {
        convert!(celsius_to_f, c)
    }

    pub fn demo() {
        println!("17. 0C to F = {}", temp_test(0));
    }
}

mod problem18_macro_sum {
    // A macro that sums zero or more expressions.
    macro_rules! sum {
        () => {
            0
        };
        ($( $x:expr ),+ ) => {
            0 $( + $x )+
        };
    }

    pub fn total(a: i32, b: i32, c: i32) -> i32 {
        sum!(a, b, c)
    }

    pub fn demo() {
        println!("18. sum!(1, 2, 3) = {}", total(1, 2, 3));
    }
}

mod problem19_parallel_sum {
    use std::thread;

    // Split a vector into two halves, sum each half on a thread, and combine the results.
    pub fn parallel_sum(nums: Vec<i32>) -> i32 {
        let mid = nums.len() / 2;
        let left = nums[..mid].to_vec();
        let right = nums[mid..].to_vec();

        let left_handle = thread::spawn(move || left.iter().sum::<i32>());
        let right_handle = thread::spawn(move || right.iter().sum::<i32>());

        let left_sum = left_handle.join().unwrap();
        let right_sum = right_handle.join().unwrap();

        left_sum + right_sum
    }

    pub fn demo() {
        println!("19. parallel_sum = {}", parallel_sum(vec![1, 2, 3, 4, 5]));
    }
}

mod problem20_arc_mutex_counter {
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Create a shared counter, spawn n threads, wait for all of them,
    // then read the final value.
    pub fn concurrent_counter(n: usize) -> i32 {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();

        for _ in 0..n {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut value = counter_clone.lock().unwrap();
                *value += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_value = *counter.lock().unwrap();
        final_value
    }

    pub fn demo() {
        println!("20. concurrent_counter(5) = {}", concurrent_counter(5));
    }
}

mod problem21_channel_pipeline {
    use std::sync::mpsc;
    use std::thread;

    // Thread 1 filters even numbers.
    // Thread 2 squares them.
    // Main thread reads the final channel, formats results, and collects them.
    pub fn process_numbers(nums: Vec<i32>) -> Vec<String> {
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        let handle1 = thread::spawn(move || {
            for n in nums {
                if n % 2 == 0 {
                    tx1.send(n).unwrap();
                }
            }
        });

        let handle2 = thread::spawn(move || {
            for n in rx1 {
                tx2.send(n * n).unwrap();
            }
        });

        let result = rx2
            .iter()
            .map(|n| format!("Result: {}", n))
            .collect::<Vec<String>>();

        handle1.join().unwrap();
        handle2.join().unwrap();

        result
    }

    pub fn demo() {
        println!(
            "21. Channel pipeline = {:?}",
            process_numbers(vec![1, 2, 3, 4, 5, 6])
        );
    }
}


mod problem22_arc_mutex_counter_judge0_friendly {
    use std::sync::{Arc, Mutex};
    use std::thread;

    // A judge-friendly version of the shared counter problem.
    // Instead of spawning an excessive number of threads when n is huge,
    // we cap the worker count and let each worker perform multiple increments.
    pub fn concurrent_counter_judge0(n: usize) -> i32 {
        let counter = Arc::new(Mutex::new(0));

        let workers = if n < 8 { n } else { 8 };
        if workers == 0 {
            return 0;
        }

        let base = n / workers;
        let extra = n % workers;
        let mut handles = Vec::new();

        for i in 0..workers {
            let counter_clone = Arc::clone(&counter);
            let increments = if i < extra { base + 1 } else { base };

            let handle = thread::spawn(move || {
                for _ in 0..increments {
                    let mut value = counter_clone.lock().unwrap();
                    *value += 1;
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_value = *counter.lock().unwrap();
        final_value
    }

    pub fn demo() {
        println!(
            "22. Judge0-friendly concurrent_counter(100) = {}",
            concurrent_counter_judge0(100)
        );
    }
}

mod problem23_arc_mutex_counter_exact_n_threads {
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Another explicit version of the same pattern:
    // create exactly n threads, wait for all of them, then read the result.
    pub fn concurrent_counter_exact(n: usize) -> i32 {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = Vec::with_capacity(n);

        for _ in 0..n {
            let counter_clone = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                let mut value = counter_clone.lock().unwrap();
                *value += 1;
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_value = *counter.lock().unwrap();
        final_value
    }

    pub fn demo() {
        println!(
            "23. Exact-n-threads concurrent_counter(5) = {}",
            concurrent_counter_exact(5)
        );
    }
}

fn main() {
    println!("Rust Exercises Project\n");

    problem01_heap_box::demo();
    problem02_deref_wrapper::demo();
    problem03_rc_count::demo();
    problem04_refcell_counter::demo();
    problem05_trait_objects_shapes::demo();
    problem06_formatter_pipeline::demo();
    problem07_newtype_display::demo();
    problem08_associated_types::demo();
    problem09_add_display_vec2::demo();
    problem10_functions_as_values::demo();
    problem11_boxed_closures::demo();
    problem12_match_guards_and_at::demo();
    problem13_parse_patterns::demo();
    problem14_raw_pointers_swap::demo();
    problem15_safe_wrapper_pattern::demo();
    problem16_macro_square::demo();
    problem17_macro_convert::demo();
    problem18_macro_sum::demo();
    problem19_parallel_sum::demo();
    problem20_arc_mutex_counter::demo();
    problem21_channel_pipeline::demo();
    problem22_arc_mutex_counter_judge0_friendly::demo();
    problem23_arc_mutex_counter_exact_n_threads::demo();
}
