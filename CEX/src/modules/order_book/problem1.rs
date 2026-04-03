use std::collections::BTreeMap;

// Problem 1: Aggregate Orders into a Book
//
// Build one side of the order book from raw orders.
// Raw orders can share the same price and must be aggregated into price levels.
//
// Rules:
// - Aggregate quantity by price.
// - Buy side (is_buy=true): sort levels DESC by price.
// - Sell side (is_buy=false): sort levels ASC by price.
// - best_price is the first level's price after sorting.
//
// Worked example (buy side):
// orders: [(100,10), (100,20), (101,5)]
// aggregate: {100:30, 101:5}
// sorted: [(101,5), (100,30)]
// best bid: 101
fn aggregate_and_best(orders: &[(u64, u64)], is_buy: bool) -> (Vec<(u64, u64)>, u64) {
    let mut map = BTreeMap::new();

    for &(price, qty) in orders {
        *map.entry(price).or_insert(0u64) += qty;
    }

    let levels: Vec<(u64, u64)> = if is_buy {
        map.into_iter().rev().collect()
    } else {
        map.into_iter().collect()
    };

    let best_price = levels.first().map(|(price, _)| *price).unwrap_or(0);
    (levels, best_price)
}

pub fn run() {
    let (levels, best) = aggregate_and_best(&[(100, 10), (100, 20), (101, 5)], true);
    println!("[The Order Book - Problem 1] Aggregate Orders into a Book");
    println!("levels={levels:?}, best_price={best}");
}

#[cfg(test)]
mod tests {
    use super::aggregate_and_best;

    #[test]
    fn aggregates_and_sorts_buy_side() {
        let (levels, best) = aggregate_and_best(&[(100, 10), (100, 20), (101, 5)], true);
        assert_eq!(levels, vec![(101, 5), (100, 30)]);
        assert_eq!(best, 101);
    }

    #[test]
    fn aggregates_and_sorts_sell_side() {
        let (levels, best) = aggregate_and_best(&[(103, 2), (102, 1), (103, 3)], false);
        assert_eq!(levels, vec![(102, 1), (103, 5)]);
        assert_eq!(best, 102);
    }

    #[test]
    fn handles_empty_orders() {
        let (levels, best) = aggregate_and_best(&[], true);
        assert!(levels.is_empty());
        assert_eq!(best, 0);
    }
}
