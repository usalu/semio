//! 🧾️ Wire-core glue — wiring only. Domain lives at the owner `🦀️.rs` files.
//!
//! Limits, record kinds, flags, merge and undo policies and the primitive codecs, with the scalar timestamp and id codec, the dictionaries and the retained wire fields beside them.


#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

#[path = "../../🦀️.rs"]
mod wire;
pub use wire::*;

#[path = "../../../🔢️scalar/🦀️.rs"]
pub mod scalar;
pub use scalar::*;

#[path = "../../../📖️dictionary/🦀️.rs"]
pub mod dictionary;
pub use dictionary::*;

#[path = "../../../⚙️codec/🛬️retained/🦀️.rs"]
pub mod retained;
pub use retained::{RetainedWireFieldDecode, RetainedWireBytesDecode, RetainedWireFieldStep, RetainedWireByteSource, OriginalWireSourceIdentity};
