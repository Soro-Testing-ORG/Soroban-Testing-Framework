# soroban-test-framework

A higher-level testing library for [Soroban](https://soroban.stellar.org) smart contracts.

Writing integration tests for Soroban contracts with the raw SDK is verbose and repetitive. This framework provides ergonomic abstractions — think Hardhat/Foundry, but for Soroban — so you can focus on what your contract should do, not on boilerplate setup.

## Features

- **`Scenario`** — fluent test environment builder: deploy contracts, create accounts, advance the ledger
- **`TestRunner`** — captures invocation results for clean assertions
- **`Fixtures`** — named, deterministic test accounts and pre-funded balances
- **`Mocks`** — stub cross-contract dependencies to isolate the contract under test
- **Assertion macros** — `assert_ok!`, `assert_err!`, `assert_event!`

## Status

🚧 **Early development.** The API is being designed and core modules are scaffolded. Contributions welcome — see [CONTRIBUTING.md](CONTRIBUTING.md).

## Quick Start

Add to your contract's `Cargo.toml`:

```toml
[dev-dependencies]
soroban-test-framework = { git = "https://github.com/your-org/soroban-test-framework" }
```

Write a test:

```rust
use soroban_test_framework::prelude::*;

#[test]
fn test_counter() {
    let scenario = Scenario::new();
    let contract_id = scenario.deploy(COUNTER_WASM);

    let result = scenario.runner().invoke::<u32>(&contract_id, "increment", vec![]);
    assert_ok!(result);
}
```

See the [`examples/`](examples/) directory for full working examples.

## Project Structure

```
crates/
  soroban-test-framework/   # Core library
    src/
      lib.rs                # Public API & prelude
      scenario.rs           # Test environment builder
      runner.rs             # Contract invocation runner
      fixtures.rs           # Account & state fixtures
      mocks.rs              # Mock contract stubs
      assertions.rs         # Assertion macros
examples/
  counter/                  # Minimal counter contract + tests
  token/                    # Fungible token contract + tests
docs/
  architecture.md           # Design decisions
  contributing-guide.md     # How to contribute
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). All skill levels welcome — issues are labeled by complexity.

## License

MIT
