//! ⏱️ Original native IO controls own generic receiving without a storage provider dependency.
#[cfg(test)]
pub(crate) use semio_framework_trace_memory_testing as test_allocation;
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:test_allocation::RequestedAllocator=test_allocation::RequestedAllocator;
#[path="../../🦀️.rs"]
mod component;
pub use component::*;
