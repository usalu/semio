//! 🧪️ Public original IO laws exercise the production control owner through its declared API.
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;
#[cfg(test)]
pub use semio_framework_io_control::*;
#[cfg(test)]
#[path="🦀️.rs"]
mod tests;
