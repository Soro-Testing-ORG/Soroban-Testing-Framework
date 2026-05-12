//! # soroban-test-framework
//!
//! A higher-level testing library for Soroban smart contracts.
//!
//! Provides ergonomic abstractions over the raw `soroban-sdk` test environment,
//! inspired by Hardhat/Foundry patterns but tailored for the Soroban/Stellar ecosystem.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use soroban_test_framework::prelude::*;
//!
//! #[test]
//! fn test_my_contract() {
//!     let scenario = Scenario::new();
//!     let alice = scenario.create_account("alice");
//!
//!     let contract_id = scenario.deploy::<MyContract>(alice.address());
//!     scenario.invoke(contract_id, "initialize", (alice.address(),));
//!
//!     assert_ok!(scenario.invoke(contract_id, "increment", ()));
//! }
//! ```

pub mod assertions;
pub mod fixtures;
pub mod mocks;
pub mod runner;
pub mod scenario;

/// Re-exports for ergonomic wildcard imports in tests.
pub mod prelude {
    pub use crate::assertions::*;
    pub use crate::fixtures::AccountFixture;
    pub use crate::mocks::MockContract;
    pub use crate::runner::TestRunner;
    pub use crate::scenario::Scenario;
}
