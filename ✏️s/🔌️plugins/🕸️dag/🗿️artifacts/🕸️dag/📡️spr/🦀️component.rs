//! ⚖️ DAG artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! This component only carries the artifact-facing `encode_op`/`decode_op` wrappers plus the op
//! text↔binary equivalence law. The app's typed `DagCommand` enum — which used to share the old
//! `📡️protocol` crate with this codec — is an APP concern, not an artifact one: it now lives in
//! `🎛️apps/🕸️dag/🦀️component.rs`, assembled from the `🎮️commands/*` payload modules by
//! `semio_framework_plugin::app_commands!`. `DagNodeGraphEditOp` (the old protocol crate's batched
//! sub-operation enum for `nodeGraphEdit`) moved with it, into `🎮️commands/🕸️graph/🦀️component.rs`
//! alongside the command it's a field of.

use crate::artifacts::dag::op::DagOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `DagOperation` to its binary command form.
pub fn encode_op(operation: &DagOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `DagOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<DagOperation, protocol::ProtocolError> {
    DagOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = DagOperation::SetNodes { nodes: Vec::new() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests
