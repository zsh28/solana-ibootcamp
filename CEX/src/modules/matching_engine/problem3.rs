// Problem 3: The Complete Matching Engine
//
// Process orders in sequence. For each order:
// - match against opposite side while crossing is possible,
// - record trades in execution order,
// - rest any unfilled remainder on its own side.
//
// Input order format: (side, price, qty)
// - side 0 = BUY
// - side 1 = SELL
//
// Trade price is always the resting order's price.
fn run_matching_engine(orders: &[(u8, u64, u64)]) -> Vec<(u64, u64)> {
    let mut bids: Vec<(u64, u64)> = Vec::new();
    let mut asks: Vec<(u64, u64)> = Vec::new();
    let mut trades: Vec<(u64, u64)> = Vec::new();

    for &(side, price, qty) in orders {
        let mut remaining = qty;

        if side == 0 {
            asks.sort_by_key(|&(p, _)| p);

            while remaining > 0 && !asks.is_empty() && asks[0].0 <= price {
                let trade_price = asks[0].0;
                let trade_qty = remaining.min(asks[0].1);
                trades.push((trade_price, trade_qty));
                remaining -= trade_qty;
                asks[0].1 -= trade_qty;
                if asks[0].1 == 0 {
                    asks.remove(0);
                }
            }

            if remaining > 0 {
                bids.push((price, remaining));
            }
        } else {
            bids.sort_by(|a, b| b.0.cmp(&a.0));

            while remaining > 0 && !bids.is_empty() && bids[0].0 >= price {
                let trade_price = bids[0].0;
                let trade_qty = remaining.min(bids[0].1);
                trades.push((trade_price, trade_qty));
                remaining -= trade_qty;
                bids[0].1 -= trade_qty;
                if bids[0].1 == 0 {
                    bids.remove(0);
                }
            }

            if remaining > 0 {
                asks.push((price, remaining));
            }
        }
    }

    trades
}

pub fn run() {
    let orders = vec![(0, 100, 10), (1, 100, 5), (1, 100, 3)];
    let trades = run_matching_engine(&orders);

    println!("[The Matching Engine - Problem 3] Build the Complete Matching Engine");
    println!("trades: {trades:?}");
}

#[cfg(test)]
mod tests {
    use super::run_matching_engine;

    #[test]
    fn matches_simple_sequence() {
        let orders = vec![(0, 100, 10), (1, 100, 5), (1, 100, 3)];
        assert_eq!(run_matching_engine(&orders), vec![(100, 5), (100, 3)]);
    }

    #[test]
    fn matches_across_multiple_ask_levels_for_buy() {
        let orders = vec![(1, 100, 5), (1, 101, 5), (0, 103, 8)];
        assert_eq!(run_matching_engine(&orders), vec![(100, 5), (101, 3)]);
    }

    #[test]
    fn preserves_execution_order_of_trades() {
        let orders = vec![
            (1, 100, 2),
            (1, 101, 2),
            (0, 101, 4),
            (0, 100, 1),
            (1, 100, 1),
        ];
        assert_eq!(
            run_matching_engine(&orders),
            vec![(100, 2), (101, 2), (100, 1)]
        );
    }
}
