//! ⚖️ Note artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! The app's typed `NoteCommand` enum — which used to share the old `📡️protocol` crate with this codec
//! — is an APP concern, not an artifact one: it now lives in `🎛️apps/🗒️note/🦀️component.rs`, assembled
//! from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

use crate::artifacts::note::op::NoteOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `NoteOperation` to its binary state-patch form.
pub fn encode_op(operation: &NoteOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `NoteOperation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<NoteOperation, protocol::ProtocolError> {
    NoteOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::note::NoteDocument;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = NoteOperation::SetGridSpacing { spacing: Some(24.0) };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn note_document_text_round_trips_store_with_applied_operation() {
        let envelope = store::create_document_envelope::<NoteDocument, NoteOperation>("note.document", "doc-text-test", crate::artifacts::note::engine::empty_note_document(), None);
        let mut doc_store = store::DocumentStore::new(envelope);
        doc_store.dispatch(store::DocumentCommand::Apply { operations: vec![NoteOperation::SetGridSpacing { spacing: Some(48.0) }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `NoteOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this file's
    /// existing pack round-trip law (same pattern as `mathematical_protocol`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};

        let envelope = store::create_document_envelope::<NoteDocument, NoteOperation>("note.document", "command-envelope-demo", crate::artifacts::note::engine::empty_note_document(), None);
        let mut doc_store = store::DocumentStore::new(envelope);
        doc_store.dispatch(store::DocumentCommand::Apply { operations: vec![NoteOperation::SetGridSpacing { spacing: Some(48.0) }], description: None }).expect("apply");
        let edit: &Edit<NoteOperation> = doc_store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<NoteDocument, NoteOperation>(edit, &DocumentId(doc_store.envelope().id.clone()), &SchemaId(doc_store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
