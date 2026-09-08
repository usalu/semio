//! ⚖️ DAG artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! This component only carries the artifact-facing `encode_op`/`decode_op` wrappers plus the op
//! text↔binary equivalence law. The app's typed `DagCommand` enum — which used to share the old
//! `📡️protocol` crate with this codec — is an APP concern, not an artifact one: it now lives in
//! `✏️editor/🦀️.rs`, assembled from the `🎮️commands/*` payload modules by
//! `semio_framework_plugin::app_commands!`. `DagNodeGraphEditOp` (the old protocol crate's batched
//! sub-operation enum for `nodeGraphEdit`) moved with it, into `🎮️commands/🕸️set-algorithm/🦀️.rs`
//! alongside the command it's a field of.

use crate::op::DagMutation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `DagMutation` to its binary command form.
pub fn encode_op(operation: &DagMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `DagMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<DagMutation, protocol::ProtocolError> {
    DagMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-protocol-conformance/🦀️.rs"]
mod semio_protocol_conformance;
