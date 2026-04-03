fn process_incoming_order(
    order_type: &str,
    side: &str,
    price: u64,
    qty: u64,
    best_bid: u64,
    best_ask: u64,
) -> &'static str {
    if order_type != "MARKET" && order_type != "LIMIT" {
        return "REJECTED";
    }

    if side != "BUY" && side != "SELL" {
        return "REJECTED";
    }

    if qty == 0 {
        return "REJECTED";
    }

    if order_type == "LIMIT" && price == 0 {
        return "REJECTED";
    }

    if order_type == "MARKET" {
        return "IMMEDIATE";
    }

    if side == "BUY" && price >= best_ask {
        return "IMMEDIATE";
    }

    if side == "SELL" && price <= best_bid {
        return "IMMEDIATE";
    }

    "RESTING"
}

pub fn run() {
    let status = process_incoming_order("LIMIT", "BUY", 110, 10, 100, 105);
    println!("[Orders & Pricing - Problem 1] Build the Order Intake Pipeline");
    println!("Example classification: {status}");
}

#[cfg(test)]
mod tests {
    use super::process_incoming_order;

    #[test]
    fn rejects_invalid_order_type() {
        assert_eq!(
            process_incoming_order("STOP", "BUY", 100, 10, 100, 105),
            "REJECTED"
        );
    }

    #[test]
    fn rejects_invalid_side() {
        assert_eq!(
            process_incoming_order("LIMIT", "HOLD", 100, 10, 100, 105),
            "REJECTED"
        );
    }

    #[test]
    fn rejects_zero_qty() {
        assert_eq!(
            process_incoming_order("MARKET", "BUY", 0, 0, 100, 105),
            "REJECTED"
        );
    }

    #[test]
    fn rejects_zero_price_limit_order() {
        assert_eq!(
            process_incoming_order("LIMIT", "SELL", 0, 10, 100, 105),
            "REJECTED"
        );
    }

    #[test]
    fn market_orders_are_immediate() {
        assert_eq!(
            process_incoming_order("MARKET", "SELL", 0, 5, 100, 105),
            "IMMEDIATE"
        );
    }

    #[test]
    fn limit_buy_classifies_correctly() {
        assert_eq!(
            process_incoming_order("LIMIT", "BUY", 110, 10, 100, 105),
            "IMMEDIATE"
        );
        assert_eq!(
            process_incoming_order("LIMIT", "BUY", 99, 10, 100, 105),
            "RESTING"
        );
    }

    #[test]
    fn limit_sell_classifies_correctly() {
        assert_eq!(
            process_incoming_order("LIMIT", "SELL", 100, 10, 100, 105),
            "IMMEDIATE"
        );
        assert_eq!(
            process_incoming_order("LIMIT", "SELL", 106, 10, 100, 105),
            "RESTING"
        );
    }
}
