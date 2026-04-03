mod problem1;
mod problem2;

pub fn run(problem: Option<&str>) {
    match problem {
        Some("1") => problem1::run(),
        Some("2") => problem2::run(),
        Some(other) => {
            eprintln!("Invalid problem for accounts-settlement: {other}");
            print_usage();
        }
        None => run_all_problems(),
    }
}

fn run_all_problems() {
    println!("Accounts & Settlement - Running all problems");
    println!("-------------------------------------------");

    println!("Problem 1 output:");
    problem1::run();

    println!("Problem 2 output:");
    problem2::run();
}

fn print_usage() {
    println!("Run accounts-settlement problems with:");
    println!("  cargo run -- 4 1");
    println!("  cargo run -- 4 2");
}
