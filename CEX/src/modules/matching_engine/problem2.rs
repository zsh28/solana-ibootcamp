// Problem 2: Walking the Book
//
// Fill a buy order against ask levels sorted from cheapest to most expensive.
//
// Rules:
// - If limit_price == 0: MARKET buy (no price ceiling).
// - If limit_price > 0: stop once ask_price > limit_price.
// - At each level, fill min(remaining_qty, available_qty).
//
// Return:
// - total_filled
// - total_cost
// - remaining_qty
//
// Worked example:
// fill_buy_order(0, 8, &[(100,5),(101,10)]) => (8, 803, 0)
fn fill_buy_order(limit_price: u64, mut qty: u64, asks: &[(u64, u64)]) -> (u64, u64, u64) {
    let mut filled = 0u64;
    let mut cost = 0u64;

    for &(ask_price, available) in asks {
        if qty == 0 {
            break;
        }

        if limit_price > 0 && ask_price > limit_price {
            break;
        }

        let fill = qty.min(available);
        filled += fill;
        cost += fill * ask_price;
        qty -= fill;
    }

    (filled, cost, qty)
}

pub fn run() {
    let result = fill_buy_order(0, 8, &[(100, 5), (101, 10)]);
    println!("[The Matching Engine - Problem 2] Fill Buy Orders Against the Ask Book");
    println!("fill result: {result:?}");
}

#[cfg(test)]
mod tests {
    use super::fill_buy_order;

    #[test]
    fn fills_market_buy_across_multiple_levels() {
        assert_eq!(fill_buy_order(0, 8, &[(100, 5), (101, 10)]), (8, 803, 0));
    }

    #[test]
    fn respects_limit_price_ceiling() {
        assert_eq!(
            fill_buy_order(101, 12, &[(100, 5), (101, 5), (103, 5)]),
            (10, 1005, 2)
        );
    }

    #[test]
    fn returns_unfilled_when_all_asks_too_expensive() {
        assert_eq!(fill_buy_order(99, 5, &[(100, 5)]), (0, 0, 5));
    }
}
