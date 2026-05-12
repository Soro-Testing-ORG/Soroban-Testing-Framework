//! Minimal fungible token contract — demonstrates multi-account testing patterns.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Balance(Address),
}

#[derive(Debug, PartialEq)]
pub enum TokenError {
    InsufficientBalance = 1,
}

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    /// Mints `amount` tokens to `to`. In production this would be access-controlled.
    pub fn mint(env: Env, to: Address, amount: i128) {
        let key = DataKey::Balance(to);
        let current: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        env.storage().persistent().set(&key, &(current + amount));
    }

    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(account))
            .unwrap_or(0)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        let from_key = DataKey::Balance(from.clone());
        let from_bal: i128 = env.storage().persistent().get(&from_key).unwrap_or(0);
        assert!(from_bal >= amount, "insufficient balance");
        env.storage().persistent().set(&from_key, &(from_bal - amount));

        let to_key = DataKey::Balance(to);
        let to_bal: i128 = env.storage().persistent().get(&to_key).unwrap_or(0);
        env.storage().persistent().set(&to_key, &(to_bal + amount));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_mint_and_balance() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let alice = Address::generate(&env);
        client.mint(&alice, &1000);
        assert_eq!(client.balance(&alice), 1000);
    }

    #[test]
    fn test_transfer() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        client.mint(&alice, &500);
        client.transfer(&alice, &bob, &200);

        assert_eq!(client.balance(&alice), 300);
        assert_eq!(client.balance(&bob), 200);
    }
}
