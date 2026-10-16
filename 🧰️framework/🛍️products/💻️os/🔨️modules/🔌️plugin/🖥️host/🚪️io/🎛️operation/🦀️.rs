//! 🌉️ Foreign runtime receiving retains original caller authority outside static host state.

#[cfg(any(test, feature = "testkit"))]
#[path = "../../🧪️tests/🛂️receiving/🦀️.rs"]
pub mod test_native_authority;
pub use semio_framework_os_vcs::io::binary::entity_identity::control::{OriginalOperationReceiver, OriginalOperationReceiving};
#[cfg(any(test, feature = "testkit"))] pub(crate) use semio_framework_os_vcs::io::binary::entity_identity::control::{EntityIdentityAuthority};
#[path="📨️slot/🦀️.rs"]
pub(crate) mod slot;
#[path="🐎️wasmtime/🦀️.rs"]
pub(crate) mod wasmtime;
#[cfg(test)]
#[path="🧪️tests/🐎️wasmtime/🦀️.rs"]
mod tests;

mod return_types{pub use semio_framework_plugin_host_wit::actor_bindings::semio::framework::{effects,ui,types,pure};pub use semio_framework_plugin_host_wit::actor_bindings::exports::semio::framework::{reactor,codec};}
#[path="../../../🚪️io/🛂️authority/📤️return/🦀️.rs"]
pub(crate) mod return_walk;

pub use wasmtime::{OperationHostStore, ReceiveError, allocation_kind, receive, receive_retained, register, request, return_bytes};
pub use slot::{OperationReply, OperationRequest, OperationSlot};
pub use return_walk::{walk_snapshot_file_result, walk_snapshot_payload_result};
