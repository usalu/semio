//! 🪤️ Companion tests of os-channel: channel codec unit tests and document archive decoding.
//!
//! 🧭️ The tests are mounted unchanged from the production tree and reach the production crate through its public API only.
#![cfg(test)]

#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

pub use semio_framework_os_channel::*;
use protocol::wire::command_ingress::{FixedCommandPage, CommandPageSet, PagedCommand, COMMAND_PAGE_MAXIMUM_BYTES, INVOCATION_RESULT_PACK_MAXIMUM_BYTES};

#[path = "../🔬️unit/🦀️.rs"]
mod tests;

#[path = "../../📦️archive/🛬️decode/🧪️tests"]
pub mod archive_decoding {
    pub use semio_framework_os_channel::archive_decoding::*;
    use semio_framework_os_channel::archive_schema::*;
    use semio_framework_value::{NativeDecodeControl, RetainedCloneGrant, ValueError, ValueRefusalKind};
    use semio_framework_value::retained_clone::RetainedCloneProgress;
    use semio_framework_io_control::NativeSnapshotBodyWallet;

    #[path = "🦀️.rs"]
    mod tests;
}
