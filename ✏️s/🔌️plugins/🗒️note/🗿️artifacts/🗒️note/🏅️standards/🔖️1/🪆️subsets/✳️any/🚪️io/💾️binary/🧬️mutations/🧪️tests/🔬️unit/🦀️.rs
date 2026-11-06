use crate::standards::v1::subsets::any::io::binary::mutations::*;
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
    let envelope = store::os_store::document_materialize::create_document_envelope::<NoteSnapshot, NoteMutation>("note.document", "doc-text-test", crate::standards::v1::subsets::any::io::text::snapshot::empty_note_snapshot(), None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut doc_store = crate::host::owned::new_note_store(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    doc_store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![crate::schema::mutations::change_grid_spacing(Some(48.0))], transaction: None }, &mut semio_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
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

    let envelope = store::os_store::document_materialize::create_document_envelope::<NoteSnapshot, NoteMutation>("note.document", "command-envelope-demo", crate::standards::v1::subsets::any::io::text::snapshot::empty_note_snapshot(), None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut doc_store = crate::host::owned::new_note_store(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    doc_store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![crate::schema::mutations::change_grid_spacing(Some(48.0))], transaction: None }, &mut semio_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    let edit: &Edit<NoteMutation> = doc_store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<NoteSnapshot, NoteMutation>(edit, &ArtifactId(doc_store.envelope().id.clone()), &SchemaId(doc_store.envelope().schema.clone())).await;
}
//#endregion 🔖️CommandEnvelopeTests

/// \u{1f4e5}\uFE0F The real declared mutation codec and its typed original cursor agree with the independent semantic payload corpus.
#[test]
fn original_note_mutation_receiving_keeps_declared_payload_and_refuses_foreign_input(){
 use protocol::original_receiving::{OpBinaryReceiving,OpBinaryReceivingCursor,RetainedCloneGrant};
 use semio_framework_value::{retirement::controlled::ControlledRetirement,NativeDecodeControl};
 let law:serde_json::Value=serde_json::from_str(include_str!("receiving.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let expected=crate::schema::mutations::rename_note::<NoteMutation>(row["newTitle"].as_str().map(str::to_owned));let source=encode_op(&expected).unwrap();
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
  let mut accept=|_|true;let mut native=NativeDecodeControl::new(16<<20,&mut accept);let(mut cursor,progress)=NoteMutation::admit_receiving(&source,(0,source.len()),grant).result.unwrap().unwrap();assert!(progress.fits(grant));
  let foreign=source.clone();assert!(cursor.demands(&foreign,grant.maximum_copy_bytes).is_err());
  let denied=RetainedCloneGrant{maximum_items:0,..grant};let turn=cursor.step(&source,&mut native,grant.maximum_copy_bytes,denied);assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap(),Default::default());
  for _ in 0..10000{if cursor.is_ready(){break}let turn=cursor.step(&source,&mut native,grant.maximum_copy_bytes,grant);assert!(turn.result.unwrap().fits(grant));}
  assert!(cursor.is_ready());let mut output=None;assert!(cursor.take_into(&mut output,grant).result.unwrap().fits(grant));assert_eq!(output.as_ref(),Some(&expected));
  let NoteMutation::RenameNote(payload)=output.as_ref().unwrap()else{panic!("concrete receiving changed its declared operation")};assert_eq!(serde_json::to_value(payload).unwrap(),serde_json::json!({"newTitle":row["newTitle"]}));
  for _ in 0..10000{if cursor.terminal_is_empty(){break}assert!(cursor.close_step(grant).result.unwrap().progress().fits(grant));}assert!(cursor.terminal_is_empty());
  let mut output=ControlledRetirement::new(output.take().unwrap()).ok().unwrap();for _ in 0..10000{if output.terminal_is_empty(){break}assert!(output.step(grant).unwrap().progress().fits(grant));}assert!(output.terminal_is_empty());
 }
}
