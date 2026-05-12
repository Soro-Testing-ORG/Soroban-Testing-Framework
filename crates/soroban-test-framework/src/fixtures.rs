//! Test fixtures for accounts and contract state.

use soroban_sdk::{Address, Env, TryFromVal};

/// A named test account with a deterministic address.
///
/// The address is derived from the name so the same name always produces
/// the same address within a test, making failure messages readable.
pub struct AccountFixture {
    pub name: String,
    pub address: Address,
}

impl AccountFixture {
    /// Creates a new account fixture with a deterministic address derived from `name`.
    ///
    /// The 32-byte contract ID is built by repeating the name's bytes cyclically,
    /// giving a stable, unique-enough address for test isolation.
    pub fn new(env: &Env, name: &str) -> Self {
        let bytes = name.as_bytes();
        let mut id = [0u8; 32];
        for (i, slot) in id.iter_mut().enumerate() {
            *slot = bytes[i % bytes.len()];
        }
        let sc_addr = soroban_sdk::xdr::ScAddress::Contract(soroban_sdk::xdr::Hash(id));
        let address = Address::try_from_val(env, &sc_addr).expect("valid contract address");
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
