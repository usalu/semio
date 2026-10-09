//! 🌉️ Foreign runtime receiving retains original caller authority outside static host state.
pub(crate) use crate::os_vcs::io::binary::entity_identity::control::{EntityIdentityAuthority,OriginalOperationReceiver,OriginalOperationReceiving};
#[path="📥️input/🦀️.rs"]
pub(crate) mod input;
#[path="📤️checkpoint/🦀️.rs"]
pub(crate) mod checkpoint;
#[path="📨️slot/🦀️.rs"]
pub(crate) mod slot;
#[path="🐎️wasmtime/🦀️.rs"]
pub(crate) mod wasmtime;
#[cfg(test)]
#[path="🧪️tests/🐎️wasmtime/🦀️.rs"]
mod tests;

mod return_types{pub use crate::actor_bindings::semio::framework::{effects,ui,types,pure};pub use crate::actor_bindings::exports::semio::framework::{reactor,codec};}
#[path="../../../🚪️io/🛂️authority/📤️return/🦀️.rs"]
pub(crate) mod return_walk;
