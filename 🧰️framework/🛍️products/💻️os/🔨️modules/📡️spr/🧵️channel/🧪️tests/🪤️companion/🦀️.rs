//! 🪤️ Companion tests of os-channel: channel codec unit tests and document archive decoding.
//!
//! 🧭️ The tests are mounted unchanged from the production tree and reach the production crate through its public API only.
#![cfg(test)]

use semio_framework_value_testing_allocator as _;

pub use semio_os_channel::*;
use protocol::wire::command_ingress::{FixedCommandPage, CommandPageSet, PagedCommand, COMMAND_PAGE_MAXIMUM_BYTES, INVOCATION_RESULT_PACK_MAXIMUM_BYTES};

#[path = "../🔬️unit/🦀️.rs"]
mod tests;

#[path = "../../📦️archive/🛬️decode/🧪️tests"]
pub mod archive_decoding {
    pub use semio_os_channel::archive_decoding::*;
    use semio_os_channel::archive_schema::*;
    use semio_framework_value::{RetainedCloneGrant, ValueError, ValueRefusalKind};
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
    use semio_framework_value::retained_clone::RetainedCloneProgress;
    use semio_framework_io_control::NativeSnapshotBodyWallet;

    #[path = "🦀️.rs"]
    mod tests;
}
