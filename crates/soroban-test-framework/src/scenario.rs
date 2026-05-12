//! High-level test scenario builder.

use soroban_sdk::{Address, Env};

use crate::fixtures::AccountFixture;
use crate::runner::TestRunner;

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

    /// Deploys a contract from raw WASM bytes and returns its address.
    pub fn deploy(&self, wasm_bytes: &[u8]) -> Address {
        self.env.register_contract_wasm(None, wasm_bytes)
    }

    /// Creates a named account fixture with a deterministic address.
    pub fn create_account(&self, name: &str) -> AccountFixture {
        AccountFixture::new(&self.env, name)
    }

    /// Advances the ledger by `n` sequences, simulating time passing.
    pub fn advance_ledger(&self, n: u32) {
        // TODO: implement ledger advancement using env.ledger().set(...)
        let _ = n;
        todo!("advance_ledger: update env ledger sequence and timestamp")
    }

    /// Returns a `TestRunner` bound to this scenario's environment.
    pub fn runner(&self) -> TestRunner<'_> {
        TestRunner::new(&self.env)
    }
}

impl Default for Scenario {
    fn default() -> Self {
        Self::new()
    }
}
