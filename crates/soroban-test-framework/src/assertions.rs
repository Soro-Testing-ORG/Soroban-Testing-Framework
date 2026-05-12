//! Assertion macros and helpers for Soroban contract tests.
//!
//! These complement `assert_eq!` / `assert!` with contract-specific patterns
//! like checking for specific error codes or event emissions.

/// Asserts a contract invocation returned `Ok`, panicking with a clear message on failure.
///
/// # Example
/// ```rust,ignore
/// assert_ok!(scenario.invoke(id, "increment", args));
/// ```
#[macro_export]
macro_rules! assert_ok {
    ($result:expr) => {
        match $result.inner {
            Ok(v) => v,
            Err(e) => panic!("expected Ok, got Err: {:?}", e),
        }
    };
}

/// Asserts a contract invocation returned `Err` with the given error code.
///
/// # Example
/// ```rust,ignore
/// assert_err!(scenario.invoke(id, "withdraw", args), ContractError::Unauthorized);
/// ```
#[macro_export]
macro_rules! assert_err {
    ($result:expr, $expected:expr) => {
        match $result.inner {
            Err(e) => assert_eq!(e, $expected.into(), "wrong error code"),
            Ok(_) => panic!("expected Err({:?}), got Ok", $expected),
        }
    };
}

/// Asserts that a specific event was emitted during the last invocation.
///
/// # TODO
/// Implement event matching against `env.events().all()`.
#[macro_export]
macro_rules! assert_event {
    ($env:expr, $contract_id:expr, $topics:expr) => {
        // TODO: filter env.events().all() for matching contract + topics
        let _ = ($env, $contract_id, $topics);
        todo!("assert_event: match emitted events")
    };
}
