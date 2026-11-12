//! ⏱️ Owns the original camera storage module and its complete physical corpus in place.
#[path = "../../🦀️.rs"]
mod storage;
pub use storage::*;

#[cfg(test)]
use semio_framework_value_testing_allocator as _;
