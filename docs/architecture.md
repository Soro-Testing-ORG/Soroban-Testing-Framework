# Architecture

## Overview

`soroban-test-framework` is a thin ergonomic layer over the `soroban-sdk` test utilities. It does not replace the SDK — it wraps it with patterns that make tests readable and maintainable.

## Core Abstractions

### `Scenario`

The entry point for every test. Owns the `Env` and exposes factory methods for deploying contracts and creating accounts. One `Scenario` per test function.

```
Scenario
  └── Env (soroban_sdk::Env)
  └── deploy(wasm) -> Address
  └── create_account(name) -> AccountFixture
  └── advance_ledger(n)
  └── runner() -> TestRunner
```

### `TestRunner`

Wraps `env.invoke_contract(...)` and captures the result as `InvokeResult<T>`. This lets assertions be written without unwrapping manually.

### `Fixtures`

`AccountFixture` generates a deterministic `Address` from a human-readable name. This makes test output readable: you see "alice" in panics, not a hex address.

`FundedAccount` extends `AccountFixture` with a seeded token balance.

### `MockContract`

Registers a stub contract at a given address. Stubs map function names to closures that return controlled `Val`s. Useful for isolating the contract under test from oracle or token dependencies.

### Assertion Macros

Thin macros over `assert_eq!` / `panic!` that produce clear failure messages specific to contract testing:

- `assert_ok!` — unwraps `InvokeResult::Ok`
- `assert_err!` — matches `InvokeResult::Err` against an expected error code
- `assert_event!` — checks `env.events().all()` for a matching event

## Design Principles

1. **Zero magic** — every abstraction is a thin wrapper. Contributors can always drop down to raw `Env` when needed.
2. **Readable tests** — test code should read like a specification, not like SDK glue.
3. **No runtime overhead** — all abstractions are zero-cost in production; the framework is `dev-dependencies` only.
