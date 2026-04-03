mod modules;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }

    let module = args[1].as_str();
    let problem = args.get(2).map(String::as_str);

    match module {
        "1" | "orders-pricing" => modules::orders_pricing::run(problem),
        "2" | "order-book" => modules::order_book::run(problem),
        "3" | "matching-engine" => modules::matching_engine::run(problem),
        "4" | "accounts-settlement" => modules::accounts_settlement::run(problem),
        "5" | "market-data-analytics" => modules::market_data_analytics::run(problem),
        _ => {
            eprintln!("Invalid module: {module}");
            print_usage(&args[0]);
        }
    }
}

fn print_usage(bin: &str) {
    println!("Usage: {bin} <module> [problem]");
    println!("Modules:");
    println!("  1 or orders-pricing          (3 problems)");
    println!("  2 or order-book              (2 problems)");
    println!("  3 or matching-engine         (3 problems)");
    println!("  4 or accounts-settlement     (2 problems)");
    println!("  5 or market-data-analytics   (2 problems)");
    println!("Total problems: 12");
    println!("Examples:");
    println!("  cargo run -- 1");
    println!("  cargo run -- 1 2");
    println!("  cargo run -- matching-engine 3");
}
