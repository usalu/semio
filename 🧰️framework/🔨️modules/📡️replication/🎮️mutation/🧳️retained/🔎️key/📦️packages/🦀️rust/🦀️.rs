//! 🔎️ Keyed-row lookup exposes its canonical typed owner without retaining parent mutation glue.
#[path = "../../🦀️.rs"]
pub mod key;
pub use key::{RetainedKeyLookup, RetainedKeyMatch, RetainedKeyedRow};

#[cfg(test)]
pub(crate) use semio_framework_trace_memory_testing as test_allocation;
#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;
