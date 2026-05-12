# Contributing to soroban-test-framework

Thanks for your interest in contributing! This document covers everything you need to get started.

## Prerequisites

- [Rust](https://rustup.rs/) (stable, 1.74+)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup) (`cargo install --locked soroban-cli`)

## Getting Started

```bash
git clone https://github.com/your-org/soroban-test-framework
cd soroban-test-framework
cargo build
cargo test
```

## Project Layout

| Path | Purpose |
|------|---------|
| `crates/soroban-test-framework/src/scenario.rs` | Test environment builder |
| `crates/soroban-test-framework/src/runner.rs` | Contract invocation & result capture |
| `crates/soroban-test-framework/src/fixtures.rs` | Account and state fixtures |
| `crates/soroban-test-framework/src/mocks.rs` | Mock/stub cross-contract calls |
| `crates/soroban-test-framework/src/assertions.rs` | Assertion macros |
| `examples/counter/` | Minimal example contract |
| `examples/token/` | Fungible token example |

## How to Contribute

1. **Find an issue** — browse open issues. Issues are labeled by complexity:
   - `complexity: trivial` — small, self-contained (docs, minor fixes)
   - `complexity: medium` — standard feature or involved bug fix
   - `complexity: high` — complex feature or refactor

2. **Comment on the issue** to let others know you're working on it.

3. **Fork and branch** — use a descriptive branch name like `feat/scenario-ledger-advance` or `fix/runner-error-capture`.

4. **Write tests** — all new functionality must include tests. Run `cargo test` before opening a PR.

5. **Open a PR** — fill in the PR template. Link the issue with `Closes #<number>`.

## Code Style

- Run `cargo fmt` before committing.
- Run `cargo clippy -- -D warnings` and fix any warnings.
- Keep functions small and focused. Prefer explicit over clever.
- Document public items with `///` doc comments.

## Good First Issues

Look for issues tagged `good first issue`. These are scoped to a single module and have clear acceptance criteria.

## Questions?

Open a [GitHub Discussion](https://github.com/your-org/soroban-test-framework/discussions) or comment on the relevant issue.
