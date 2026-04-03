// Problem 3: Price-Time Priority
//
// Exchange matching priority rules:
// - Buy side: highest price first, then earliest timestamp
// - Sell side: lowest price first, then earliest timestamp
//
// Task:
// - sort_buy_orders(orders)  -> Vec<usize>
// - sort_sell_orders(orders) -> Vec<usize>
//
// Each order is (price, timestamp).
// Return indices into the original input in priority order.
//
// Worked example (buy side):
// [(100, 1), (101, 2), (100, 3)] => [1, 0, 2]
fn sort_buy_orders(orders: &[(u64, u64)]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..orders.len()).collect();

    indices.sort_by(|&a, &b| {
        orders[b]
            .0
            .cmp(&orders[a].0)
            .then(orders[a].1.cmp(&orders[b].1))
    });

    indices
}

fn sort_sell_orders(orders: &[(u64, u64)]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..orders.len()).collect();

    indices.sort_by(|&a, &b| {
        orders[a]
            .0
            .cmp(&orders[b].0)
            .then(orders[a].1.cmp(&orders[b].1))
    });

    indices
}

pub fn run() {
    let buy_orders = vec![(100, 1), (101, 2), (100, 3)];
    let sell_orders = vec![(105, 2), (104, 3), (105, 1)];
    let buy_priority = sort_buy_orders(&buy_orders);
    let sell_priority = sort_sell_orders(&sell_orders);

    println!("[Orders & Pricing - Problem 3] Sort by Price-Time Priority");
    println!("buy priority indices: {buy_priority:?}");
    println!("sell priority indices: {sell_priority:?}");
}

#[cfg(test)]
mod tests {
    use super::{sort_buy_orders, sort_sell_orders};

    #[test]
    fn sorts_buys_by_desc_price_then_asc_time() {
        let orders = vec![(100, 1), (101, 2), (100, 3)];
        assert_eq!(sort_buy_orders(&orders), vec![1, 0, 2]);
    }

    #[test]
    fn sorts_sells_by_asc_price_then_asc_time() {
        let orders = vec![(105, 2), (104, 3), (105, 1)];
        assert_eq!(sort_sell_orders(&orders), vec![1, 2, 0]);
    }

    #[test]
    fn handles_empty_input() {
        let orders: Vec<(u64, u64)> = Vec::new();
        assert!(sort_buy_orders(&orders).is_empty());
        assert!(sort_sell_orders(&orders).is_empty());
    }
}
