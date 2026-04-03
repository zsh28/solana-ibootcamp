fn order_economics(
    best_bid: u64,
    best_ask: u64,
    price: u64,
    qty: u64,
    fee_bps: u64,
) -> (u64, u64, u64, u64) {
    let spread = best_ask - best_bid;
    let midprice = (best_bid + best_ask) / 2;
    let notional = price * qty;
    let fee = notional * fee_bps / 10_000;

    (spread, midprice, notional, fee)
}

pub fn run() {
    let (spread, midprice, notional, fee) = order_economics(100, 105, 100, 10, 30);
    println!("[Orders & Pricing - Problem 2] Compute Spread, Notional & Fees");
    println!("spread={spread}, mid={midprice}, notional={notional}, fee={fee}");
}

#[cfg(test)]
mod tests {
    use super::order_economics;

    #[test]
    fn computes_worked_example() {
        assert_eq!(order_economics(100, 105, 100, 10, 30), (5, 102, 1000, 3));
    }

    #[test]
    fn handles_integer_division_for_mid_and_fee() {
        assert_eq!(order_economics(100, 101, 99, 3, 25), (1, 100, 297, 0));
    }
}
