use crate::binary::*;
use semio_s_artifact_draw_drawing_model::{create_drawing_shape_layer_rect, layer_id};
use semio_s_artifact_draw_drawing_model::default_drawing_document;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::DRAWING_DOCUMENT_SCHEMA;

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let document = default_drawing_document("doc-text-test", None);
    let operation = semio_s_artifact_draw_drawing_structure::create_layer::mutation::create_layer(None, Some(document.layers.len()), create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Op Binary Test")).to_string().into()).expect("nonempty authored identity"), "Op Binary Test"));
    semio_os_store_test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_a_store_with_an_applied_operation() {
    let initial = default_drawing_document("doc-text-test", None);
    let envelope = store::os_store::document_materialize::create_document_envelope::<DrawingSnapshot, DrawingMutation>(DRAWING_DOCUMENT_SCHEMA, "doc-text-test", initial, None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut doc_store = store::ArtifactStore::new::<::semio_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    doc_store.install_document_store_owners_exact(::semio_os_kernel::os_store::document_store_owners::funded_bounded_artifact_store_owners().expect("funded bounded document owners"));
    let layer = create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Added Rect")).to_string().into()).expect("nonempty authored identity"), "Added Rect");
    let layer_id_value = layer_id(&layer).to_string();
    doc_store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![semio_s_artifact_draw_drawing_structure::create_layer::mutation::create_layer(None, None, layer)], transaction: None }, &mut semio_plugin::artifact_app_laws::fixture_identity()).await.expect("apply add layer");
    doc_store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![semio_s_artifact_draw_drawing_style::set_layer_opacity::mutation::set_layer_opacity(layer_id_value.into(), 0.5)], transaction: None }, &mut semio_plugin::artifact_app_laws::fixture_identity()).await.expect("apply set opacity");
    semio_os_store_test_support::assert_document_text_round_trip(&doc_store).await;
    semio_os_store_test_support::assert_document_pack_round_trip(&doc_store).await;
    semio_os_store_test_support::assert_live_equals_replay(&doc_store).await;
    semio_os_store_test_support::close_plain_test_store(&mut doc_store);
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `DrawingMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
/// existing pack round-trip law.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use protocol::{ArtifactId, Edit, SchemaId};

    let initial = default_drawing_document("doc-text-test", None);
    let envelope = store::os_store::document_materialize::create_document_envelope::<DrawingSnapshot, DrawingMutation>(DRAWING_DOCUMENT_SCHEMA, "doc-text-test", initial, None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut doc_store = store::ArtifactStore::new::<::semio_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    doc_store.install_document_store_owners_exact(::semio_os_kernel::os_store::document_store_owners::funded_bounded_artifact_store_owners().expect("funded bounded document owners"));
    let layer = create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Added Rect")).to_string().into()).expect("nonempty authored identity"), "Added Rect");
    let layer_id_value = layer_id(&layer).to_string();
    doc_store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![semio_s_artifact_draw_drawing_structure::create_layer::mutation::create_layer(None, None, layer)], transaction: None }, &mut semio_plugin::artifact_app_laws::fixture_identity()).await.expect("apply add layer");
    doc_store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![semio_s_artifact_draw_drawing_style::set_layer_opacity::mutation::set_layer_opacity(layer_id_value.into(), 0.5)], transaction: None }, &mut semio_plugin::artifact_app_laws::fixture_identity()).await.expect("apply set opacity");
    let edit: &Edit<DrawingMutation> = doc_store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_os_store_test_support::assert_command_envelope_round_trip::<DrawingSnapshot, DrawingMutation>(edit, &ArtifactId(doc_store.envelope().id.clone()), &SchemaId(doc_store.envelope().schema.clone())).await;
    semio_os_store_test_support::close_plain_test_store(&mut doc_store);
}
//#endregion 🔖️CommandEnvelopeTests

#[test]
fn original_operation_receives_nested_drawing_pages_from_the_declared_range() {
    use crate::DrawingMutation;
    use semio_s_artifact_draw_drawing_model::{
        DrawingLayerNode, PathSegment, create_drawing_group_layer, create_drawing_path_layer,
        identity::DrawingIdentity,
    };

    let group_identity = DrawingIdentity::admit("layout-group".to_string().into()).unwrap();
    let child_identity = DrawingIdentity::admit("layout-path".to_string().into()).unwrap();
    let segments = std::iter::once(PathSegment::Move { to: [0.0, 0.0] })
        .chain((1..=180).map(|x| PathSegment::Line { to: [f64::from(x), 1.5] }))
        .collect();
    let child = create_drawing_path_layer(child_identity, "Path", segments);
    let mut group = create_drawing_group_layer(group_identity, "Group");
    let DrawingLayerNode::Group(body) = &mut group else { panic!("group factory"); };
    body.children = std::iter::once(child).collect();
    let operation: DrawingMutation =
        semio_s_artifact_draw_drawing_structure::create_layer::mutation::create_layer(None, None, group);

    assert_original_drawing_range(operation);
}

#[test]
fn original_operation_receives_renamed_layer_identifiers() {
    let operation = DrawingMutation::RenameLayer(semio_s_artifact_draw_drawing_metadata::rename_layer::mutation::RenameLayer {
        layer_id: "Path".into(),
        new_name: "Retained Path".into(),
    });
    assert_original_drawing_range(operation);
}

#[test]
fn original_operation_receives_empty_group() {
    use semio_s_artifact_draw_drawing_model::{create_drawing_group_layer, identity::DrawingIdentity};
    let id = DrawingIdentity::admit("layout-group".to_string().into()).unwrap();
    let group = create_drawing_group_layer(id, "Group");
    assert_original_drawing_range(semio_s_artifact_draw_drawing_structure::create_layer::mutation::create_layer(None, None, group));
}

#[test]
fn original_operation_receives_addressed_rectangle() {
    let layer = create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit("rectangle".to_string().into()).unwrap(), "Rectangle");
    let op = semio_s_artifact_draw_drawing_structure::create_layer::mutation::create_layer(Some("parent".into()), Some(1), layer);
    assert_original_drawing_range(op);
}

fn assert_original_drawing_range(operation: DrawingMutation) {
    use semio_framework_value::RetainedCloneGrant;
    let mut original = vec![255];
    original.extend(encode_op(&operation).unwrap());
    let end = original.len();
    original.push(255);
    let quantum = RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: 65_536,
        maximum_capacity_bytes: 1 << 20,
        maximum_release_bytes: 1 << 20,
        maximum_depth: 128,
    };
    let stock = dsl::original_receiving::ReceivingStock {
        stock: RetainedCloneGrant {
            maximum_items: 1 << 20,
            maximum_copy_bytes: 1 << 29,
            maximum_capacity_bytes: 1 << 29,
            maximum_release_bytes: 1 << 29,
            ..quantum
        },
        quantum,
        fuel: 1 << 30,
    };
    let received = dsl::original_receiving::receive_to_completion::<DrawingMutation>(&original, (1, end), stock).unwrap_or_else(|error| {
        use semio_framework_dsl_record::DslVariants;
        panic!("original Drawing operation {:?}: {error:?}", operation.to_named_record());
    });
    assert_eq!(received, operation);
}
