// Problem 1: Candlesticks & VWAP
//
// From a chronological stream of trades (price, qty), compute:
// - open: first trade price
// - high: maximum trade price
// - low: minimum trade price
// - close: last trade price
// - volume: sum of quantities
// - vwap: sum(price * qty) / volume (integer division)
//
// Worked example:
// [(100,10), (105,5), (98,20), (102,15), (101,8)]
// => (100, 105, 98, 101, 58, 100)
fn build_candle(trades: &[(u64, u64)]) -> (u64, u64, u64, u64, u64, u64) {
    let open = trades[0].0;
    let close = trades[trades.len() - 1].0;
    let high = trades.iter().map(|(p, _)| *p).max().unwrap();
    let low = trades.iter().map(|(p, _)| *p).min().unwrap();
    let volume: u64 = trades.iter().map(|(_, q)| *q).sum();
    let total_value: u64 = trades.iter().map(|(p, q)| p * q).sum();
    let vwap = if volume == 0 { 0 } else { total_value / volume };

    (open, high, low, close, volume, vwap)
}

pub fn run() {
    let trades = vec![(100, 10), (105, 5), (98, 20), (102, 15), (101, 8)];
    let candle = build_candle(&trades);
    println!("[Market Data & Analytics - Problem 1] Build Candlestick Data & VWAP");
    println!("candle={candle:?}");
}

#[cfg(test)]
mod tests {
    use super::build_candle;

    #[test]
    fn computes_worked_example() {
        let trades = vec![(100, 10), (105, 5), (98, 20), (102, 15), (101, 8)];
        assert_eq!(build_candle(&trades), (100, 105, 98, 101, 58, 100));
    }

    #[test]
    fn computes_single_trade_candle() {
        let trades = vec![(250, 4)];
        assert_eq!(build_candle(&trades), (250, 250, 250, 250, 4, 250));
    }
}
