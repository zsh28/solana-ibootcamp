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

---

# CEX Bootcamp - Module 2 (The Order Book)

This module covers how a centralized exchange represents liquidity on the bid/ask book, and how to derive depth-based signals from that structure.

## What Module 2 is about

Module 2 focuses on two core problems:

1. **Aggregate Orders into a Book**
   - Convert raw `(price, qty)` orders into aggregated price levels.
   - Build one side of the book at a time:
     - **Bids** sorted descending (highest first),
     - **Asks** sorted ascending (lowest first).
   - Return both the sorted levels and the side's best price.

2. **Analyze Order Book Depth**
   - Build a depth-limited snapshot (top `N` levels) for bids and asks.
   - Compute top-of-book quantity pressure with imbalance:
     - `imbalance = bid_qty * 100 / (bid_qty + ask_qty)`.

## What we learned

- **Price-level aggregation:** multiple orders at the same price collapse into one level with summed quantity.
- **Book ordering semantics:** bid/ask sides use opposite sorting directions because "best" means different things.
- **Depth as liquidity lens:** top `N` levels approximate immediate executable liquidity and slippage risk.
- **Imbalance as directional signal:** >50 implies buy-side pressure, <50 implies sell-side pressure.
- **Robust edge handling:** safe behavior for empty books / zero total quantity avoids runtime errors.
- **Deterministic tests for microstructure logic:** examples and edge cases are validated with unit tests.

## Implemented files

- `CEX/src/modules/order_book/problem1.rs`
- `CEX/src/modules/order_book/problem2.rs`

## How to run

- Run all Module 2 problems:
  - `cargo run -- 2`
- Run a specific problem:
  - `cargo run -- 2 1`
  - `cargo run -- 2 2`
- Run tests:
  - `cargo test`
