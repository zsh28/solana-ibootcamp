// Problem 2: Book Depth & Imbalance
//
// Build a depth-limited snapshot of the book and compute imbalance.
//
// Input assumptions:
// - bid_levels are already sorted DESC (best bid first)
// - ask_levels are already sorted ASC (best ask first)
//
// Steps:
// 1) Take top N (depth) levels from each side.
// 2) Sum quantities for those top levels.
// 3) Compute imbalance percentage:
//    imbalance = bid_qty * 100 / (bid_qty + ask_qty)
//
// Worked example (depth=2):
// bids: [(102,50), (101,30), ...] => bid_qty=80
// asks: [(103,20), (104,45), ...] => ask_qty=65
// imbalance = 80*100/145 = 55
fn book_snapshot(
    bid_levels: &[(u64, u64)],
    ask_levels: &[(u64, u64)],
    depth: usize,
) -> (Vec<(u64, u64)>, Vec<(u64, u64)>, u64) {
    let top_bids: Vec<(u64, u64)> = bid_levels.iter().take(depth).copied().collect();
    let top_asks: Vec<(u64, u64)> = ask_levels.iter().take(depth).copied().collect();

    let bid_qty: u64 = top_bids.iter().map(|(_, q)| *q).sum();
    let ask_qty: u64 = top_asks.iter().map(|(_, q)| *q).sum();

    let total_qty = bid_qty + ask_qty;
    let imbalance: u64 = if total_qty == 0 {
        0
    } else {
        bid_qty * 100 / total_qty
    };

    (top_bids, top_asks, imbalance)
}

pub fn run() {
    let bids = vec![(102, 50), (101, 30), (100, 25)];
    let asks = vec![(103, 20), (104, 45), (105, 10)];
    let (top_bids, top_asks, imbalance) = book_snapshot(&bids, &asks, 2);

    println!("[The Order Book - Problem 2] Analyze Order Book Depth");
    println!("top_bids={top_bids:?}, top_asks={top_asks:?}, imbalance={imbalance}");
}

#[cfg(test)]
mod tests {
    use super::book_snapshot;

    #[test]
    fn computes_worked_example() {
        let bids = vec![(102, 50), (101, 30), (100, 25)];
        let asks = vec![(103, 20), (104, 45), (105, 10)];
        assert_eq!(
            book_snapshot(&bids, &asks, 2),
            (vec![(102, 50), (101, 30)], vec![(103, 20), (104, 45)], 55)
        );
    }

    #[test]
    fn respects_requested_depth() {
        let bids = vec![(102, 5), (101, 4)];
        let asks = vec![(103, 3), (104, 2)];
        assert_eq!(
            book_snapshot(&bids, &asks, 1),
            (vec![(102, 5)], vec![(103, 3)], 62)
        );
    }

    #[test]
    fn handles_zero_liquidity() {
        let bids: Vec<(u64, u64)> = vec![];
        let asks: Vec<(u64, u64)> = vec![];
        assert_eq!(book_snapshot(&bids, &asks, 5), (vec![], vec![], 0));
    }
}
