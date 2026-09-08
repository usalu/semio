
use super::*;
use crate::NoteSnapshot;

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let operation = crate::schema::mutations::change_grid_spacing(Some(24.0));
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn note_document_text_round_trips_store_with_applied_operation() {
    let envelope = store::create_document_envelope::<NoteSnapshot, NoteMutation>("note.document", "doc-text-test", crate::schema::empty_note_snapshot(), None);
    let mut doc_store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![crate::schema::mutations::change_grid_spacing(Some(48.0))], description: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&doc_store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&doc_store).await;
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `NoteMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
/// existing pack round-trip law (same pattern as `mathematical_protocol`'s own
/// `command_envelope_round_trip_holds_for_an_applied_operation`).
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use protocol::{ArtifactId, Edit, SchemaId};

    let envelope = store::create_document_envelope::<NoteSnapshot, NoteMutation>("note.document", "command-envelope-demo", crate::schema::empty_note_snapshot(), None);
    let mut doc_store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![crate::schema::mutations::change_grid_spacing(Some(48.0))], description: None }).await.expect("apply");
    let edit: &Edit<NoteMutation> = doc_store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<NoteSnapshot, NoteMutation>(edit, &ArtifactId(doc_store.envelope().id.clone()), &SchemaId(doc_store.envelope().schema.clone())).await;
}
//#endregion 🔖️CommandEnvelopeTests
