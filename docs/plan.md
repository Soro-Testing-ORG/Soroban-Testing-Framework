# Build Plan

Ordered from most foundational to least critical.

---

## Phase 0 — Skeleton ✅ (done)
Workspace compiles, all modules exist with public API, CI configured, docs, issue templates.

## Phase 1 — Core Runtime ✅ (done)
*A test can actually use the framework end-to-end.*

| # | Task | Status |
|---|------|--------|
| 1.1 | `Scenario::new()` + `deploy(wasm)` | ✅ |
| 1.2 | `AccountFixture::new(name)` — deterministic address | ✅ |
| 1.3 | `TestRunner::invoke<T>()` — wraps `try_invoke_contract` | ✅ |
| 1.4 | `assert_ok!` / `assert_err!` macros | ✅ |
| 1.5 | Counter example updated to use the framework | ✅ |

## Phase 2 — Fixtures & State Setup
*Tests can set up realistic pre-conditions without boilerplate.*

| # | Task | Status |
|---|------|--------|
| 2.1 | `Scenario::advance_ledger(n)` | 🔲 issue #1 |
| 2.2 | `FundedAccount` — seed token balance via mint | 🔲 issue #4 |
| 2.3 | Token example updated to use the framework | 🔲 |

## Phase 3 — Mocks
*Contracts with cross-contract dependencies can be tested in isolation.*

| # | Task | Status |
|---|------|--------|
| 3.1 | `MockContract::new(env, address)` | 🔲 |
| 3.2 | `MockContract::stub(func, handler)` | 🔲 |

## Phase 4 — Event Assertions
*Tests can verify contract events were emitted correctly.*

| # | Task | Status |
|---|------|--------|
| 4.1 | `assert_event!(env, contract_id, topics)` | 🔲 issue #3 |

## Phase 5 — Developer Experience Polish
*Framework is pleasant to use and ready for external adoption.*

| # | Task | Status |
|---|------|--------|
| 5.1 | Error code propagation in `InvokeError::Contract` | 🔲 issue #5 |
| 5.2 | `Scenario::builder()` fluent chaining API | 🔲 |
| 5.3 | Full `cargo doc` examples on every public item | 🔲 |
| 5.4 | crates.io publish prep | 🔲 |
