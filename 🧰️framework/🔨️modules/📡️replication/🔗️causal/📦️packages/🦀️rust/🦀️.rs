//! 🔗️ Causal glue — wiring only. Domain lives at the owner `🦀️.rs` files.
//!
//! Envelopes, diffs, inverses and frontiers, with their binary and text codecs beside them.


#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;

#[path = "../../../🚪️io/💾️binary/🔗️causal/🦀️.rs"]
pub mod binary;

#[path = "../../../🚪️io/📝️text/🔗️causal/🦀️.rs"]
mod text;
