//! 🧪️ Public dynamic clone protocols exercise the genuine published cursor and physical allocator.
#![cfg(test)]
pub use semio_framework_value_retained_clone::*;
pub use semio_framework_value_retained_clone::retained_clone::dynamic::DslValueRetainedCloneCursor;
pub(crate) use semio_framework_value_testing_allocator::observe_retirement_allocations;
use std::mem::size_of;
#[path="protocol.rs"]
mod protocol;
#[path="🦀️.rs"]
mod tests;
