// Problem 2: Trade Settlement
//
// For each trade (price, qty):
// - cost = price * qty
// - Buyer:  base += qty,  quote -= cost
// - Seller: base -= qty,  quote += cost
//
// Process trades in order and return final balances:
// (buyer_base, buyer_quote, seller_base, seller_quote)
//
// Worked example:
// settle_trades(0, 50000, 1000, 0, &[(100,50),(101,25)]) => (75, 42475, 925, 7525)
fn settle_trades(
    mut buyer_base: u64,
    mut buyer_quote: u64,
    mut seller_base: u64,
    mut seller_quote: u64,
    trades: &[(u64, u64)],
) -> (u64, u64, u64, u64) {
    for &(price, qty) in trades {
        let cost = price * qty;

        buyer_base += qty;
        buyer_quote -= cost;
        seller_base -= qty;
        seller_quote += cost;
    }

    (buyer_base, buyer_quote, seller_base, seller_quote)
}

pub fn run() {
    let result = settle_trades(0, 50000, 1000, 0, &[(100, 50), (101, 25)]);
    println!("[Accounts & Settlement - Problem 2] Settle Trades Between Parties");
    println!("final balances: {result:?}");
}

#[cfg(test)]
mod tests {
    use super::settle_trades;

    #[test]
    fn settles_worked_example() {
        assert_eq!(
            settle_trades(0, 50000, 1000, 0, &[(100, 50), (101, 25)]),
            (75, 42475, 925, 7525)
        );
    }

    #[test]
    fn preserves_asset_conservation() {
        let initial = (10, 10000, 20, 5000);
        let final_bal = settle_trades(
            initial.0,
            initial.1,
            initial.2,
            initial.3,
            &[(100, 5), (90, 2)],
        );

        let initial_base_total = initial.0 + initial.2;
        let initial_quote_total = initial.1 + initial.3;
        let final_base_total = final_bal.0 + final_bal.2;
        let final_quote_total = final_bal.1 + final_bal.3;

        assert_eq!(initial_base_total, final_base_total);
        assert_eq!(initial_quote_total, final_quote_total);
    }

    #[test]
    fn no_trades_means_no_changes() {
        assert_eq!(settle_trades(1, 2, 3, 4, &[]), (1, 2, 3, 4));
    }
}
