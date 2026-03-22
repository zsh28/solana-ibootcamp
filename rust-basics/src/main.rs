mod modules;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }

    let module = args[1].as_str();
    let subtask = args.get(2).map(String::as_str);

    match module {
        "1" => modules::module1::run(subtask),
        "2" => modules::module2::run(subtask),
        "3" => modules::module3::run(subtask),
        "4" => modules::module4::run(subtask),
        "5" => modules::module5::run(subtask),
        "6" => modules::module6::run(subtask),
        "7" => modules::module7::run(subtask),
        "8" => modules::module8::run(subtask),
        "9" => modules::module9::run(subtask),
        "10" => modules::module10::run(subtask),
        "11" => modules::module11::run(subtask),
        "12" => modules::module12::run(subtask),
        _ => {
            eprintln!("Invalid module: {module}");
            print_usage(&args[0]);
        }
    }
}

fn print_usage(bin: &str) {
    println!("Usage: {bin} <module-number> [subtask-number]");
    println!("Run modules 1 through 12");
    println!("Examples:");
    println!("  cargo run -- 1      # run all subtasks in module 1");
    println!("  cargo run -- 1 3    # run module 1, subtask 3");
    println!("  cargo run -- 2      # run all subtasks in module 2");
    println!("  cargo run -- 2 1    # run module 2, subtask 1");
    println!("  cargo run -- 3      # run all subtasks in module 3");
    println!("  cargo run -- 3 2    # run module 3, subtask 2");
    println!("  cargo run -- 3 4    # run module 3, subtask 4");
    println!("  cargo run -- 4      # run all subtasks in module 4");
    println!("  cargo run -- 4 1    # run module 4, subtask 1");
    println!("  cargo run -- 5      # run all subtasks in module 5");
    println!("  cargo run -- 5 3    # run module 5, subtask 3");
    println!("  cargo run -- 6      # run all subtasks in module 6");
    println!("  cargo run -- 6 2    # run module 6, subtask 2");
    println!("  cargo run -- 7      # run all subtasks in module 7");
    println!("  cargo run -- 7 3    # run module 7, subtask 3");
    println!("  cargo run -- 8      # run all subtasks in module 8");
    println!("  cargo run -- 8 2    # run module 8, subtask 2");
    println!("  cargo run -- 9      # run all subtasks in module 9");
    println!("  cargo run -- 9 4    # run module 9, subtask 4");
    println!("  cargo run -- 10     # run all subtasks in module 10");
    println!("  cargo run -- 10 2   # run module 10, subtask 2");
    println!("  cargo run -- 11     # run all subtasks in module 11");
    println!("  cargo run -- 11 4   # run module 11, subtask 4");
    println!("  cargo run -- 12     # run all subtasks in module 12");
    println!("  cargo run -- 12 3   # run module 12, subtask 3");
}
