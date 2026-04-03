// Problem 2: Volume Analytics
//
// Split taker volume by aggressor side:
// - side 0 => taker-buy volume
// - side 1 => taker-sell volume
//
// Then compute:
// buy_percentage = buy_vol * 100 / (buy_vol + sell_vol)
//
// Worked example:
// [(100,0), (50,1), (75,0), (25,1)] => (175, 75, 70)
fn analyze_volume(trades: &[(u64, u8)]) -> (u64, u64, u64) {
    let mut buy_vol = 0u64;
    let mut sell_vol = 0u64;

    for &(qty, side) in trades {
        if side == 0 {
            buy_vol += qty;
        } else {
            sell_vol += qty;
        }
    }

    let total = buy_vol + sell_vol;
    let buy_pct = if total == 0 { 0 } else { buy_vol * 100 / total };

    (buy_vol, sell_vol, buy_pct)
}

pub fn run() {
    let trades = vec![(100, 0), (50, 1), (75, 0), (25, 1)];
    let result = analyze_volume(&trades);
    println!("[Market Data & Analytics - Problem 2] Analyze Taker Buy/Sell Volume");
    println!("volume_split={result:?}");
}

#[cfg(test)]
mod tests {
    use super::analyze_volume;

    #[test]
    fn computes_worked_example() {
        let trades = vec![(100, 0), (50, 1), (75, 0), (25, 1)];
        assert_eq!(analyze_volume(&trades), (175, 75, 70));
    }

    #[test]
    fn handles_all_buy_and_all_sell() {
        assert_eq!(analyze_volume(&[(10, 0), (20, 0)]), (30, 0, 100));
        assert_eq!(analyze_volume(&[(10, 1), (20, 1)]), (0, 30, 0));
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(analyze_volume(&[]), (0, 0, 0));
    }
}
