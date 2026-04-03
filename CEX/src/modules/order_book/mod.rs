mod problem1;
mod problem2;

pub fn run(problem: Option<&str>) {
    match problem {
        Some("1") => problem1::run(),
        Some("2") => problem2::run(),
        Some(other) => {
            eprintln!("Invalid problem for order-book: {other}");
            print_usage();
        }
        None => run_all_problems(),
    }
}

fn run_all_problems() {
    println!("The Order Book - Running all problems");
    println!("------------------------------------");

    println!("Problem 1 output:");
    problem1::run();

    println!("Problem 2 output:");
    problem2::run();
}

fn print_usage() {
    println!("Run order-book problems with:");
    println!("  cargo run -- 2 1");
    println!("  cargo run -- 2 2");
}
