//! Contract invocation runner with result capture.

use soroban_sdk::{Env, InvokeError, Symbol, Val, Vec};

/// Wraps a contract invocation result for ergonomic assertions.
pub struct InvokeResult<T> {
    pub inner: Result<T, soroban_sdk::Error>,
}

impl<T: core::fmt::Debug> InvokeResult<T> {
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
    /// Returns `InvokeResult::Ok(value)` on success or `InvokeResult::Err(error)`
    /// if the contract traps or returns an error code.
    pub fn invoke<T>(
        &self,
        contract_id: &soroban_sdk::Address,
        func: &str,
        args: Vec<Val>,
    ) -> InvokeResult<T>
    where
        T: soroban_sdk::TryFromVal<Env, Val>,
    {
        let symbol = Symbol::new(self.env, func);
        let result = self
            .env
            .try_invoke_contract::<T, soroban_sdk::Error>(contract_id, &symbol, args);

        let inner = match result {
            Ok(Ok(val)) => Ok(val),
            Ok(Err(_)) => Err(soroban_sdk::Error::from_type_and_code(
                soroban_sdk::xdr::ScErrorType::Value,
                soroban_sdk::xdr::ScErrorCode::InvalidInput,
            )),
            Err(Ok(e)) => Err(e),
            Err(Err(InvokeError::Abort)) => panic!("contract invocation aborted"),
            Err(Err(InvokeError::Contract(_code))) => Err(soroban_sdk::Error::from_type_and_code(
                soroban_sdk::xdr::ScErrorType::Contract,
                soroban_sdk::xdr::ScErrorCode::ArithDomain, // TODO: propagate _code directly
            )),
        };

        InvokeResult { inner }
    }
}
