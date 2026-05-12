//! Simple counter contract — used as a minimal example for the testing framework.

#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

const KEY: &str = "count";

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

    pub fn reset(env: Env) {
        env.storage().instance().set(&KEY, &0u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;
    // TODO: replace raw env usage below with soroban-test-framework once implemented
    // use soroban_test_framework::prelude::*;

    #[test]
    fn test_increment() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CounterContract);
        let client = CounterContractClient::new(&env, &contract_id);

        assert_eq!(client.increment(), 1);
        assert_eq!(client.increment(), 2);
        assert_eq!(client.get(), 2);
    }

    #[test]
    fn test_reset() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CounterContract);
        let client = CounterContractClient::new(&env, &contract_id);

        client.increment();
        client.reset();
        assert_eq!(client.get(), 0);
    }
}
