//! ⏱️ Original native IO controls own generic receiving without a storage provider dependency.
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;
#[path="../../🦀️.rs"]
mod component;
pub use component::*;
