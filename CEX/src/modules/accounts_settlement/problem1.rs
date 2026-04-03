// Problem 1: Account Management
//
// Part A - process_balance:
// - Deposits always succeed.
// - Withdrawals succeed only if balance >= amount, otherwise skip.
//
// Part B - validate_margin:
// - BUY requires enough quote balance: quote_bal >= price * qty
// - SELL requires enough base balance:  base_bal >= qty
//
// Worked examples:
// - process_balance(1000, &[(true,500),(false,200),(false,1500)]) => 1300
// - validate_margin(true, 10, 5000, 100, 40) => true
fn process_balance(initial: u64, operations: &[(bool, u64)]) -> u64 {
    let mut balance = initial;
    for &(is_deposit, amount) in operations {
        if is_deposit {
            balance += amount;
        } else if balance >= amount {
            balance -= amount;
        }
    }
    balance
}

fn validate_margin(is_buy: bool, base_bal: u64, quote_bal: u64, price: u64, qty: u64) -> bool {
    if is_buy {
        quote_bal >= price * qty
    } else {
        base_bal >= qty
    }
}

pub fn run() {
    let final_balance = process_balance(1000, &[(true, 500), (false, 200), (false, 1500)]);
    let can_buy = validate_margin(true, 10, 5000, 100, 40);

    println!("[Accounts & Settlement - Problem 1] Manage Balances & Validate Margin");
    println!("final_balance={final_balance}, can_buy={can_buy}");
}

#[cfg(test)]
mod tests {
    use super::{process_balance, validate_margin};

    #[test]
    fn applies_deposits_and_valid_withdrawals() {
        assert_eq!(
            process_balance(1000, &[(true, 500), (false, 200), (false, 1500)]),
            1300
        );
    }

    #[test]
    fn skips_withdrawals_when_insufficient() {
        assert_eq!(process_balance(100, &[(false, 150), (true, 10)]), 110);
    }

    #[test]
    fn validates_buy_and_sell_margin() {
        assert!(validate_margin(true, 10, 5000, 100, 40));
        assert!(!validate_margin(false, 10, 5000, 100, 15));
        assert!(validate_margin(false, 10, 5000, 100, 10));
    }
}
