//! ⚖️ Sourcing curation artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`).
//!
//! The app's typed `SourcingCurationCommand` enum — which used to share the old `📡️protocol` crate with
//! this codec — is an APP concern, not an artifact one: it now lives in `🎛️apps/🗂️curation/🦀️.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::mutations::SourcingMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `SourcingMutation` to its binary state-patch form.
pub fn encode_op(operation: &SourcingMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `SourcingMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<SourcingMutation, protocol::ProtocolError> {
    SourcingMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
