//! ⚖️ Note artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! The app's typed `NoteCommand` enum — which used to share the old `📡️protocol` crate with this codec
//! — is an APP concern, not an artifact one: it now lives in `🎛️apps/🗒️note/🦀️component.rs`, assembled
//! from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

use crate::artifacts::note::schema::mutations::text::NoteMutation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
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
mod tests {
    use super::*;
    use crate::artifacts::note::NoteSnapshot;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = crate::artifacts::note::schema::mutations::change_grid_spacing(Some(24.0));
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn note_document_text_round_trips_store_with_applied_operation() {
        let envelope = store::create_document_envelope::<NoteSnapshot, NoteMutation>("note.document", "doc-text-test", crate::artifacts::note::engine::empty_note_snapshot(), None);
        let mut doc_store = store::ArtifactStore::new(envelope);
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![crate::artifacts::note::schema::mutations::change_grid_spacing(Some(48.0))], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&doc_store);
        store::os_store::test_support::assert_document_pack_round_trip(&doc_store);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `NoteMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing pack round-trip law (same pattern as `mathematical_protocol`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{ArtifactId, Edit, SchemaId};

        let envelope = store::create_document_envelope::<NoteSnapshot, NoteMutation>("note.document", "command-envelope-demo", crate::artifacts::note::engine::empty_note_snapshot(), None);
        let mut doc_store = store::ArtifactStore::new(envelope);
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![crate::artifacts::note::schema::mutations::change_grid_spacing(Some(48.0))], description: None }).expect("apply");
        let edit: &Edit<NoteMutation> = doc_store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<NoteSnapshot, NoteMutation>(edit, &ArtifactId(doc_store.envelope().id.clone()), &SchemaId(doc_store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semio_protocol_conformance {
    use super::*;

    #[test]
    fn component_protocol_semio_is_protocol_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
        assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
        let _ = COMPONENT_PROTOCOL_PATH;
    }
    #[test]
    fn verify_protocol_bytes_against_encoded_spr() {
        use crate::artifacts::note::schema::mutations::text::NoteMutation;
        let operation = crate::artifacts::note::schema::mutations::change_grid_visible(Some(false));
        let bytes = encode_op(&operation).expect("encode op");
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes spr bytes");
    }

}

