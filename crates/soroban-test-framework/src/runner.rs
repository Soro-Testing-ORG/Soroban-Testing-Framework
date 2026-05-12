//! Contract invocation runner with result capture.
//!
//! `TestRunner` wraps contract calls and captures both successful results
//! and expected errors, making assertions cleaner in tests.

use soroban_sdk::{Env, IntoVal, Val};

/// Wraps a contract invocation result for ergonomic assertions.
pub struct InvokeResult<T> {
    pub inner: Result<T, soroban_sdk::Error>,
}

impl<T> InvokeResult<T> {
    /// Asserts the invocation succeeded and returns the value.
    pub fn unwrap_ok(self) -> T {
        self.inner.expect("expected Ok but got Err")
    }

    /// Asserts the invocation failed with any error.
    pub fn unwrap_err(self) -> soroban_sdk::Error {
        self.inner.expect_err("expected Err but got Ok")
    }
}

/// Executes contract function calls against a Soroban test environment.
pub struct TestRunner<'a> {
    pub env: &'a Env,
}

impl<'a> TestRunner<'a> {
    pub fn new(env: &'a Env) -> Self {
        Self { env }
    }

    /// Invokes a contract function and captures the result.
    ///
    /// # Arguments
    /// * `contract_id` - The deployed contract address.
    /// * `func` - The function name to call.
    /// * `args` - Arguments as a `soroban_sdk::Vec<Val>`.
    ///
    /// # TODO
    /// Implement actual invocation via `env.invoke_contract`.
    pub fn invoke<T: soroban_sdk::TryFromVal<Env, Val>>(
        &self,
        contract_id: &soroban_sdk::Address,
        func: &str,
        args: soroban_sdk::Vec<Val>,
    ) -> InvokeResult<T> {
        // TODO: call env.invoke_contract(contract_id, &Symbol::new(env, func), args)
        let _ = (contract_id, func, args);
        todo!("invoke: call env.invoke_contract and wrap result")
    }
}
