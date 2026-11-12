//! 🥣️ Companion tests of io-base64: RFC 4648 vectors, the third-party differential oracle and the controlled allocation laws.
//!
//! 🧭️ The tests read the codec through its public API only; the test allocator that observes requested bytes is installed here because the companion owns the test binary.
#![cfg(test)]

pub use semio_framework_io_base64::*;

use semio_framework_value_testing_allocator as _;

#[path = "../🔬️unit/🦀️.rs"]
mod tests;
