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
            eprintln!("Invalid subtask for module 9: {other}");
            print_usage();
        }
        None => run_all_subtasks(),
    }
}

fn run_all_subtasks() {
    println!("Module 9 - Running all subtasks");
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
    println!("Run module 9 subtasks with:");
    println!("  cargo run -- 9 1");
    println!("  cargo run -- 9 2");
    println!("  cargo run -- 9 3");
    println!("  cargo run -- 9 4");
}
