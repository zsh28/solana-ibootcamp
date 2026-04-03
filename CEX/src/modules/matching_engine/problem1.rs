// Problem 1: Order Matching
//
// Determine whether an incoming buy and sell order can trade.
//
// Match condition:
// - buy_price >= sell_price
//
// Trade outcomes when matched:
// - trade_price = sell_price (resting order's price)
// - trade_qty = min(buy_qty, sell_qty)
//
// If not matched, return (false, 0, 0).
//
// Worked example:
// - buy 105x10 vs sell 100x5 => (true, 100, 5)
fn try_match(buy_price: u64, buy_qty: u64, sell_price: u64, sell_qty: u64) -> (bool, u64, u64) {
    if buy_price >= sell_price {
        let trade_qty = buy_qty.min(sell_qty);
        (true, sell_price, trade_qty)
    } else {
        (false, 0, 0)
    }
}

pub fn run() {
    let result = try_match(105, 10, 100, 5);
    println!("[The Matching Engine - Problem 1] Match a Buy and Sell Order");
    println!("match result: {result:?}");
}

#[cfg(test)]
mod tests {
    use super::try_match;

    #[test]
    fn matches_and_uses_resting_price() {
        assert_eq!(try_match(105, 10, 100, 5), (true, 100, 5));
    }

    #[test]
    fn does_not_match_when_buy_below_sell() {
        assert_eq!(try_match(99, 10, 100, 5), (false, 0, 0));
    }

    #[test]
    fn handles_partial_fill_from_seller() {
        assert_eq!(try_match(100, 2, 100, 9), (true, 100, 2));
    }
}
