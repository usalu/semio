//! ⚖️ Note artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! The app's typed `NoteCommand` enum — which used to share the old `📡️protocol` crate with this codec
//! — is an APP concern, not an artifact one: it now lives in `✏️editor/🦀️.rs`, assembled
//! from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

use crate::standards::v1::subsets::any::io::mutations::text::NoteMutation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `NoteMutation` to its binary state-patch form.
pub fn encode_op(operation: &NoteMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `NoteMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<NoteMutation, protocol::ProtocolError> {
    NoteMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-protocol-conformance/🦀️.rs"]
mod semio_protocol_conformance;
