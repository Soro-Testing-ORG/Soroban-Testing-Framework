//! Mock contract utilities for isolating contract-under-test from dependencies.
//!
//! `MockContract` lets you register a stub implementation for any contract
//! address so cross-contract calls return controlled values during tests.

use soroban_sdk::{Address, Env, Val};

/// A stub contract that returns pre-configured responses for specific function calls.
///
/// # Example
/// ```rust,ignore
/// let mock = MockContract::new(&env, oracle_address);
/// mock.stub("get_price", |_args| Ok(1_000_000i128.into_val(&env)));
/// ```
pub struct MockContract {
    pub address: Address,
    // TODO: store stubs as a map of function name -> handler closure
}

impl MockContract {
    /// Registers a mock at the given address in the test environment.
    ///
    /// # TODO
    /// Register a no-op or stub contract via `env.register_contract`.
    pub fn new(env: &Env, address: Address) -> Self {
        let _ = env;
        Self { address }
    }

    /// Configures a stub response for a specific function.
    ///
    /// # Arguments
    /// * `func` - The function name to stub.
    /// * `handler` - Closure receiving call args and returning a `Val`.
    ///
    /// # TODO
    /// Store handler and wire it into the registered contract implementation.
    pub fn stub<F>(&mut self, func: &str, handler: F)
    where
        F: Fn(soroban_sdk::Vec<Val>) -> Val + 'static,
    {
        let _ = (func, handler);
        todo!("MockContract::stub: register function handler")
    }
}
