//! Test fixtures for accounts, balances, and contract state.
//!
//! Fixtures provide deterministic, named test actors so tests read like
//! plain English: `let alice = scenario.create_account("alice")`.

use soroban_sdk::{Address, Env};

/// A named test account with a deterministic address.
pub struct AccountFixture {
    pub name: String,
    pub address: Address,
}

impl AccountFixture {
    /// Creates a new account fixture with a deterministic address derived from `name`.
    ///
    /// # TODO
    /// Derive address deterministically from name bytes via `Address::generate` or similar.
    pub fn new(env: &Env, name: &str) -> Self {
        // TODO: generate a stable address from name so tests are reproducible
        let _ = env;
        let address = todo!("AccountFixture::new: generate deterministic address from name");
        Self {
            name: name.to_string(),
            address,
        }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}

/// Pre-funded account fixture with a specified token balance.
///
/// # TODO
/// Implement balance seeding via the token contract's `mint` function.
pub struct FundedAccount {
    pub account: AccountFixture,
    pub balance: i128,
}

impl FundedAccount {
    pub fn new(env: &Env, name: &str, balance: i128) -> Self {
        Self {
            account: AccountFixture::new(env, name),
            balance,
        }
    }
}
