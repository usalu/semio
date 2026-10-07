//! ⚖️ Playbook artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! `protocol::OpBinary for PlaybookMutation` is handcrafted in the sibling `📝️text` facet
//! (`../../📝️text/🧬️mutations/🦀️.rs`) — P6: derive no longer emits OpText/OpBinary. This component only
//! adds the thin artifact-facing `encode_op`/`decode_op` wrappers plus the op text↔binary
//! equivalence law.
//!
//! The app's typed `PlaybookCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an EDITOR concern, not an artifact one: it now lives in `✏️editor/🦀️.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::op::PlaybookMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `PlaybookMutation` to its binary state-patch form.
pub fn encode_op(operation: &PlaybookMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `PlaybookMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<PlaybookMutation, protocol::ProtocolError> {
    PlaybookMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
use super::*;
pub use crate::mutations::{apply_playbook_mutation, change_title_operation, inverse_playbook_mutation, ChangeTitle, PlaybookMutation};

impl protocol::OpBinary for PlaybookMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}
}
