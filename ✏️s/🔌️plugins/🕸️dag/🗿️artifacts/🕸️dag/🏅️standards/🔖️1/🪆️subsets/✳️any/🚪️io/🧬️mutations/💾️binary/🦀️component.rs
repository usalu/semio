//! ⚖️ DAG artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! This component only carries the artifact-facing `encode_op`/`decode_op` wrappers plus the op
//! text↔binary equivalence law. The app's typed `DagCommand` enum — which used to share the old
//! `📡️protocol` crate with this codec — is an APP concern, not an artifact one: it now lives in
//! `✏️editor/🦀️component.rs`, assembled from the `🎮️commands/*` payload modules by
//! `semio_framework_plugin::app_commands!`. `DagNodeGraphEditOp` (the old protocol crate's batched
//! sub-operation enum for `nodeGraphEdit`) moved with it, into `🎮️commands/🕸️set-algorithm/🦀️component.rs`
//! alongside the command it's a field of.

use crate::artifacts::dag::op::DagMutation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


/// 📦️ Encodes a `DagMutation` to its binary command form.
pub async fn encode_op(operation: &DagMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `DagMutation` from its binary command form.
pub async fn decode_op(bytes: &[u8]) -> Result<DagMutation, protocol::ProtocolError> {
    DagMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let operation = crate::artifacts::dag::mutations::delete_node("node-1".into());
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semio_protocol_conformance {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn component_protocol_semio_is_protocol_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
        assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
        let _ = COMPONENT_PROTOCOL_PATH;
    }

    #[semio_framework_async_macros::async_test]
    async fn verify_protocol_bytes_against_encoded_spr() {
        let operation = crate::artifacts::dag::mutations::delete_node("node-1".into());
        let bytes = encode_op(&operation).expect("encode op");
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes spr bytes");
    }
}

