//! 🔎️ Keyed-row lookup exposes its canonical typed owner without retaining parent mutation glue.
#[path = "../../🦀️.rs"]
pub mod key;
pub use key::{RetainedKeyLookup, RetainedKeyMatch, RetainedKeyedRow};

#[cfg(test)]
pub(crate) use semio_framework_trace_memory_testing as test_allocation;
#[cfg(test)]
use semio_framework_value_testing_allocator as _;
