//! ⏱️ Original native IO controls own generic receiving without a storage provider dependency.
#[cfg(test)]
#[path="../../../../⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
pub(crate) mod test_allocation;
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:test_allocation::RequestedAllocator=test_allocation::RequestedAllocator;
#[path="../../🦀️.rs"]
mod component;
pub use component::*;
