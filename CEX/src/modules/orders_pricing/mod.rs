mod problem1;
mod problem2;
mod problem3;

pub fn run(problem: Option<&str>) {
    match problem {
        Some("1") => problem1::run(),
        Some("2") => problem2::run(),
        Some("3") => problem3::run(),
        Some(other) => {
            eprintln!("Invalid problem for orders-pricing: {other}");
            print_usage();
        }
        None => run_all_problems(),
    }
}

fn run_all_problems() {
    println!("Orders & Pricing - Running all problems");
    println!("-------------------------------------");

    println!("Problem 1 output:");
    problem1::run();

    println!("Problem 2 output:");
    problem2::run();

    println!("Problem 3 output:");
    problem3::run();
}

fn print_usage() {
    println!("Run orders-pricing problems with:");
    println!("  cargo run -- 1 1");
    println!("  cargo run -- 1 2");
    println!("  cargo run -- 1 3");
}
