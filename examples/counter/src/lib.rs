//! Simple counter contract — minimal example for the testing framework.

#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol};

const KEY: Symbol = soroban_sdk::symbol_short!("count");

#[contract]
pub struct CounterContract;

#[contractimpl]
impl CounterContract {
    pub fn increment(env: Env) -> u32 {
        let count: u32 = env.storage().instance().get(&KEY).unwrap_or(0);
        let next = count + 1;
        env.storage().instance().set(&KEY, &next);
        next
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&KEY).unwrap_or(0)
    }

    /// Resets the counter to zero.
    ///
    /// # TODO (contributor issue)
    /// Add a test for `reset` using `soroban_test_framework::prelude::*` that:
    /// 1. Deploys the contract via `Scenario::deploy(COUNTER_WASM)`
    /// 2. Calls `increment` once
    /// 3. Calls `reset`
    /// 4. Asserts `get` returns 0 using `assert_ok!`
    pub fn reset(env: Env) {
        env.storage().instance().set(&KEY, &0u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::vec;
    use soroban_test_framework::prelude::*;
    use soroban_test_framework::{assert_ok};

    fn deploy_counter() -> (Scenario, soroban_sdk::Address) {
        let scenario = Scenario::new();
        // register_contract is used here because we don't have the compiled WASM in tests;
        // in a real integration test you'd use scenario.deploy(COUNTER_WASM)
        let id = scenario.env.register_contract(None, CounterContract);
        (scenario, id)
    }

    #[test]
    fn test_increment() {
        let (scenario, id) = deploy_counter();
        let runner = scenario.runner();

        let first = assert_ok!(runner.invoke::<u32>(&id, "increment", vec![&scenario.env]));
        assert_eq!(first, 1);

        let second = assert_ok!(runner.invoke::<u32>(&id, "increment", vec![&scenario.env]));
        assert_eq!(second, 2);
    }

    #[test]
    fn test_get() {
        let (scenario, id) = deploy_counter();
        let runner = scenario.runner();

        assert_ok!(runner.invoke::<u32>(&id, "increment", vec![&scenario.env]));
        let count = assert_ok!(runner.invoke::<u32>(&id, "get", vec![&scenario.env]));
        assert_eq!(count, 1);
    }

    // TODO: test_reset — see CounterContract::reset doc comment above
}
