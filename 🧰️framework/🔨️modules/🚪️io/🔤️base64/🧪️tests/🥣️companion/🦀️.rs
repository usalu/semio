//! 🥣️ Companion tests of io-base64: RFC 4648 vectors, the third-party differential oracle and the controlled allocation laws.
//!
//! 🧭️ The tests read the codec through its public API only; the test allocator that observes requested bytes is installed here because the companion owns the test binary.
#![cfg(test)]

pub use semio_framework_io_base64::*;

#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

#[path = "../🔬️unit/🦀️.rs"]
mod tests;
