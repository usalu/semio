//! 🎮️ Controlled retirement laws retain their own test binary and allocator.
pub use semio_framework_value_retirement::*;
pub use semio_framework_value_retirement::retirement::*;
pub(crate) use semio_framework_value_testing_allocator::observe_retirement_allocations;
pub(crate) use semio_framework_value_testing_allocator as allocation_testing;
#[path="../../🧪️tests/⏱️effect/🦀️.rs"]
mod effect;
#[path="🦀️.rs"]
mod controlled_tests;
