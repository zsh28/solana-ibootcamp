# CEX Bootcamp - Module 1 (Orders & Pricing)

This module introduces the core entry-layer logic of a centralized exchange (CEX): how incoming orders are validated, classified, economically evaluated, and prioritized for execution.

## What Module 1 is about

Module 1 focuses on three foundational problems:

1. **Order Intake Pipeline**
   - Validate incoming orders before they can reach the book.
   - Classify valid orders as:
     - `IMMEDIATE` (can execute now),
     - `RESTING` (must wait on the book), or
     - `REJECTED` (invalid input).

2. **Exchange Economics**
   - Compute core market/trade metrics:
     - spread,
     - midprice,
     - notional,
     - fee (in basis points).

3. **Price-Time Priority**
   - Implement the matching priority rule used by exchanges:
     - **Buy side:** highest price first, then earliest timestamp.
     - **Sell side:** lowest price first, then earliest timestamp.

## What we learned

- **Validation-first design:** protect the system by rejecting malformed orders early (bad type/side, zero qty, invalid limit price).
- **Deterministic classification:** clear branching rules for market vs. limit behavior and spread crossing.
- **Market microstructure basics:** why spread and midprice matter, and how notional and fee connect execution to exchange revenue.
- **Integer arithmetic in trading code:** fee/midprice calculations use integer division, which affects outputs at small sizes.
- **Priority sorting with indices:** sort order *indices* (not the original array) using custom comparators and tie-breakers.
- **Testing exchange logic:** unit tests lock in correctness for edge cases and reference examples.

## Implemented files

- `CEX/src/modules/orders_pricing/problem1.rs`
- `CEX/src/modules/orders_pricing/problem2.rs`
- `CEX/src/modules/orders_pricing/problem3.rs`

## How to run

- Run all Module 1 problems:
  - `cargo run -- 1`
- Run a specific problem:
  - `cargo run -- 1 1`
  - `cargo run -- 1 2`
  - `cargo run -- 1 3`
- Run tests:
  - `cargo test`
