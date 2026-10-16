//! ⚙️ Codec glue — wiring only. Domain lives at the owner `🦀️.rs` files.
//!
//! This crate root is the former `protocol::codec` namespace; `io` hosts the `OpBinary` and `OpText` trait family beside it.


#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;

#[path = "../../🆔️ids/🦀️.rs"]
pub mod ids;
pub use self::ids::*;

#[path = "../../../🚪️io/🦀️.rs"]
pub mod io;
pub use io::{DiffBinary, DiffCodec, DiffText, OpBinary, OpBinaryReceiving, OpBinaryReceivingCursor, OpText};
