//! 🔗️ Causal glue — wiring only. Domain lives at the owner `🦀️.rs` files.
//!
//! Envelopes, diffs, inverses and frontiers, with their binary and text codecs beside them.


#[cfg(test)]
pub(crate) use semio_framework_trace_memory_testing as test_allocation;
#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;

#[path = "../../../🚪️io/💾️binary/🔗️causal/🦀️.rs"]
pub mod binary;

#[path = "../../../🚪️io/📝️text/🔗️causal/🦀️.rs"]
mod text;
