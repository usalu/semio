//! 🎯️ Original selected retirement laws retain their own test binary and allocator.
pub use semio_framework_value_retirement::*;
pub use semio_framework_value_retirement::retirement::*;
pub(crate) use semio_framework_value_testing_allocator::observe_retirement_allocations;
#[path="🦀️.rs"]
mod original_selected_body_tests;
