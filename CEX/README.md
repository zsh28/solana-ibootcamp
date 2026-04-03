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

---

# CEX Bootcamp - Module 3 (The Matching Engine)

This module implements the execution core of the exchange: deciding when orders cross, how fills are computed while walking price levels, and how a full matching loop generates trades over time.

## What Module 3 is about

Module 3 focuses on three matching problems:

1. **Match a Buy and Sell Order**
   - Validate the crossing condition: `buy_price >= sell_price`.
   - Use resting-price execution:
     - trade price = resting order price,
     - trade qty = `min(buy_qty, sell_qty)`.
   - Return `(matched, trade_price, trade_qty)`.

2. **Fill Buy Orders Against the Ask Book**
   - Walk ask levels in ascending price.
   - Support both:
     - **Market buy** (`limit_price == 0`),
     - **Limit buy** (`limit_price > 0`, stop when ask is too expensive).
   - Return `(total_filled, total_cost, remaining_qty)`.

3. **Build the Complete Matching Engine**
   - Process a stream of buy/sell orders in arrival order.
   - Match against opposite side while crossing is possible.
   - Record trades in exact execution order.
   - Rest any remaining quantity back onto the book.

## What we learned

- **Crossing logic fundamentals:** a trade exists only when bid meets or exceeds ask.
- **Resting-price execution rule:** aggressive orders trade at book liquidity prices, not their own limit.
- **Partial fill mechanics:** quantities are consumed incrementally with `min(remaining, available)`.
- **Book walking behavior:** market/limit semantics directly control how far matching continues.
- **Stateful engine loops:** maintaining bids/asks across events is key for realistic matching behavior.
- **Execution-order correctness:** trade logs must preserve exact sequence for downstream settlement/analytics.

## Implemented files

- `CEX/src/modules/matching_engine/problem1.rs`
- `CEX/src/modules/matching_engine/problem2.rs`
- `CEX/src/modules/matching_engine/problem3.rs`

## How to run

- Run all Module 3 problems:
  - `cargo run -- 3`
- Run a specific problem:
  - `cargo run -- 3 1`
  - `cargo run -- 3 2`
  - `cargo run -- 3 3`
- Run tests:
  - `cargo test`

---

# CEX Bootcamp - Module 4 (Accounts & Settlement)

This module focuses on exchange account correctness: updating balances safely, validating whether orders are affordable, and settling executed trades while preserving total assets.

## What Module 4 is about

Module 4 focuses on two core problems:

1. **Manage Balances & Validate Margin**
   - Apply deposit/withdraw operations with insufficient-withdraw protection.
   - Enforce margin checks before order acceptance:
     - **Buy** requires quote funds (`price * qty`),
     - **Sell** requires base inventory (`qty`).

2. **Settle Trades Between Parties**
   - Process each trade `(price, qty)` sequentially.
   - Apply double-entry balance changes:
     - Buyer gets `+qty` base and pays `-(price*qty)` quote,
     - Seller gives `-qty` base and receives `+(price*qty)` quote.
   - Return final balances for both sides.

## What we learned

- **Balance safety rules:** deposits are always additive; withdrawals must pass available-funds checks.
- **Pre-trade risk control:** margin validation prevents orders that cannot be funded.
- **Asset-specific constraints:** buys consume quote currency, sells consume base asset.
- **Deterministic settlement:** processing trades in strict sequence yields reproducible account state.
- **Conservation principle:** total base and total quote are preserved across buyer+seller after settlement.
- **Edge-case robustness:** no-op behavior for empty trade lists and skipped invalid withdrawals.

## Implemented files

- `CEX/src/modules/accounts_settlement/problem1.rs`
- `CEX/src/modules/accounts_settlement/problem2.rs`

## How to run

- Run all Module 4 problems:
  - `cargo run -- 4`
- Run a specific problem:
  - `cargo run -- 4 1`
  - `cargo run -- 4 2`
- Run tests:
  - `cargo test`
