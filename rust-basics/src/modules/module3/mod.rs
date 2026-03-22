mod subtask1;
mod subtask2;
mod subtask3;
mod subtask4;

pub fn run(subtask: Option<&str>) {
    match subtask {
        Some("1") => subtask1::run(),
        Some("2") => subtask2::run(),
        Some("3") => subtask3::run(),
        Some("4") => subtask4::run(),
        Some(other) => {
            eprintln!("Invalid subtask for module 3: {other}");
            print_usage();
        }
        None => run_all_subtasks(),
    }
}

fn run_all_subtasks() {
    println!("Module 3 - Running all subtasks");
    println!("-------------------------------");

    println!("Subtask 1 output:");
    subtask1::run();

    println!("Subtask 2 output:");
    subtask2::run();

    println!("Subtask 3 output:");
    subtask3::run();

    println!("Subtask 4 output:");
    subtask4::run();
}

fn print_usage() {
    println!("Run module 3 subtasks with:");
    println!("  cargo run -- 3 1");
    println!("  cargo run -- 3 2");
    println!("  cargo run -- 3 3");
    println!("  cargo run -- 3 4");
}
