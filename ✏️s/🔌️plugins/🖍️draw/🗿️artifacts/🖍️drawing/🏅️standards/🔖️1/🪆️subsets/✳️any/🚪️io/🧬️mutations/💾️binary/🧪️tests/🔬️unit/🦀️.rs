use super::*;
use crate::schema::{create_drawing_shape_layer_rect, default_drawing_document, layer_id};
use crate::{DrawingSnapshot, DRAWING_DOCUMENT_SCHEMA};

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let document = default_drawing_document("doc-text-test", None);
    let operation = crate::mutations::create_layer(None, Some(document.layers.len()), create_drawing_shape_layer_rect("Op Binary Test"));
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_a_store_with_an_applied_operation() {
    let initial = default_drawing_document("doc-text-test", None);
    let envelope = store::create_document_envelope::<DrawingSnapshot, DrawingMutation>(DRAWING_DOCUMENT_SCHEMA, "doc-text-test", initial, None);
    let mut doc_store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    let layer = create_drawing_shape_layer_rect("Added Rect");
    let layer_id_value = layer_id(&layer).to_string();
    doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![crate::mutations::create_layer(None, None, layer)], description: Some("add rect".into()) }).await.expect("apply add layer");
    doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![crate::mutations::set_layer_opacity(layer_id_value, 0.5)], description: Some("set opacity".into()) }).await.expect("apply set opacity");
    store::os_store::test_support::assert_document_text_round_trip(&doc_store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&doc_store).await;
    store::os_store::test_support::assert_live_equals_replay(&doc_store).await;
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `DrawingMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
/// existing pack round-trip law.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use protocol::{ArtifactId, Edit, SchemaId};

    let initial = default_drawing_document("doc-text-test", None);
    let envelope = store::create_document_envelope::<DrawingSnapshot, DrawingMutation>(DRAWING_DOCUMENT_SCHEMA, "doc-text-test", initial, None);
    let mut doc_store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    let layer = create_drawing_shape_layer_rect("Added Rect");
    let layer_id_value = layer_id(&layer).to_string();
    doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![crate::mutations::create_layer(None, None, layer)], description: Some("add rect".into()) }).await.expect("apply add layer");
    doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![crate::mutations::set_layer_opacity(layer_id_value, 0.5)], description: Some("set opacity".into()) }).await.expect("apply set opacity");
    let edit: &Edit<DrawingMutation> = doc_store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<DrawingSnapshot, DrawingMutation>(edit, &ArtifactId(doc_store.envelope().id.clone()), &SchemaId(doc_store.envelope().schema.clone())).await;
}
//#endregion 🔖️CommandEnvelopeTests
