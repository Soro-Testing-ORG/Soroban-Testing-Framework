//! High-level test scenario builder.
//!
//! A `Scenario` wraps the Soroban `Env` and provides a fluent API for
//! setting up contract deployments, accounts, and state before assertions.

use soroban_sdk::{Address, Env};

use crate::fixtures::AccountFixture;

/// Central test orchestrator. Create one per test.
pub struct Scenario {
    pub env: Env,
}

impl Scenario {
    /// Creates a new `Scenario` with a fresh Soroban test environment.
    pub fn new() -> Self {
        Self {
            env: Env::default(),
        }
    }

    /// Creates a named account fixture with a deterministic address.
    ///
    /// # Example
    /// ```rust,ignore
    /// let alice = scenario.create_account("alice");
    /// ```
    pub fn create_account(&self, name: &str) -> AccountFixture {
        AccountFixture::new(&self.env, name)
    }

    /// Advances the ledger by `n` sequences, simulating time passing.
    pub fn advance_ledger(&self, n: u32) {
        // TODO: implement ledger advancement using env.ledger().set(...)
        let _ = n;
        todo!("advance_ledger: update env ledger sequence and timestamp")
    }

    /// Deploys a contract and returns its address.
    ///
    /// # Arguments
    /// * `wasm_bytes` - The compiled WASM bytes of the contract.
    pub fn deploy(&self, wasm_bytes: &[u8]) -> Address {
        // TODO: register and deploy contract via env.register_contract_wasm
        let _ = wasm_bytes;
        todo!("deploy: register contract WASM and return address")
    }
}

impl Default for Scenario {
    fn default() -> Self {
        Self::new()
    }
}
