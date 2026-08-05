//! ⚖️ Sourcing curate artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`).
//!
//! The app's typed `SourcingCurateCommand` enum — which used to share the old `📡️protocol` crate with
//! this codec — is an APP concern, not an artifact one: it now lives in `🎛️apps/🗂️curate/🦀️component.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

use crate::artifacts::curate::op::SourcingOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `SourcingOperation` to its binary state-patch form.
pub fn encode_op(operation: &SourcingOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `SourcingOperation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<SourcingOperation, protocol::ProtocolError> {
    SourcingOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::curate::CurateDocument;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = SourcingOperation::SetDocument { document: crate::artifacts::curate::engine::empty_document() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn curate_document_text_round_trips_through_a_vcs_store() {
        let document = CurateDocument { stock: crate::artifacts::curate::engine::sourcing_modules().iter().flat_map(|module| module.demo_kinds()).collect(), ..Default::default() };
        let envelope = store::create_document_envelope(crate::artifacts::curate::SOURCING_CURATE_SCHEMA, "sourcing-curate-test", document, None);
        let mut doc_store = store::DocumentStore::new(envelope);
        let mut next = doc_store.projection().expect("projection");
        crate::artifacts::curate::engine::curate_delta(&mut next, "beam-glulam-gl24h", 3);
        doc_store.dispatch(store::DocumentCommand::Apply { operations: vec![SourcingOperation::SetDocument { document: next }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }
}
//#endregion 🧪️Tests
