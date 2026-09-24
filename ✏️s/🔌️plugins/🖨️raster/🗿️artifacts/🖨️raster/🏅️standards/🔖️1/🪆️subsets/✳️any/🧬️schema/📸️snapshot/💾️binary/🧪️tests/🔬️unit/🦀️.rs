use super::*;
use crate::mutations::create_layer;
use crate::op::RasterMutation;
use crate::RasterOwnedMap;
use crate::{RasterImageAsset, RasterLayerMask, RasterLayerNode, RasterTransform, RASTER_DOCUMENT_SCHEMA};

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_and_agrees_with_dsl() {
    let document = crate::standards::v1::subsets::any::schema::semio_fixture_snapshot();
    store::os_store::test_support::assert_dsl_pack_equivalence_cold(&document, crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot);
    let bytes = encode(&document);
    let decoded = decode(&bytes).expect("decode");
    assert_eq!(decoded, document);
    // 🧹️ Both documents own a populated asset pool, so they reach the artifact's retirement seam
    // rather than `RasterOwnedMap`'s fail-closed `Drop`.
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(decoded);
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_representative_document() {
    let mut assets = RasterOwnedMap::new();
    assets.insert("asset-1".into(), crate::image_asset_child_handle("asset-1", &RasterImageAsset { mime: "image/png".into(), data: b"abc".to_vec() }).with_local_owner(std::sync::Arc::new(semio_s_artifact_stdio_semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot::default()))).expect("bounded fixture operation succeeds");
    let mut params = RasterOwnedMap::new();
    params.insert("brightness".into(), dsl::DslValue::float(0.06)).expect("bounded fixture operation succeeds");
    params.insert("label".into(), dsl::DslValue::String("Warm \"Curve\"".to_string())).expect("bounded fixture operation succeeds");
    params.insert("enabled".into(), dsl::DslValue::Bool(true)).expect("bounded fixture operation succeeds");
    params.insert("fallback".into(), dsl::DslValue::Null).expect("bounded fixture operation succeeds");
    params
        .insert(
            "curves".into(),
            dsl::DslValue::Array(vec![
                dsl::DslValue::Array(vec![dsl::DslValue::float(0.0), dsl::DslValue::float(0.0)]),
                dsl::DslValue::Array(vec![dsl::DslValue::float(0.25), dsl::DslValue::float(0.2)]),
                dsl::DslValue::Array(vec![dsl::DslValue::float(1.0), dsl::DslValue::float(1.0)]),
            ]),
        )
        .expect("bounded fixture operation succeeds");
    params.insert("nested".into(), dsl::DslValue::Object(vec![("inner".to_string(), dsl::DslValue::float(1.5))])).expect("bounded fixture operation succeeds");
    let document = RasterSnapshot {
        schema: RASTER_DOCUMENT_SCHEMA.into(),
        id: "doc-1".into(),
        title: Some("Representative \"Doc\"".into()),
        assets,
        layers: vec![
            RasterLayerNode::Pixel {
                id: "pixel-1".into(),
                name: "Pixel One".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: Some(RasterLayerMask { enabled: true, linked: false, invert: true, width: Some(64), height: None }),
                width: Some(256),
                height: Some(256),
                image_key: Some("asset-1".into()),
            },
            RasterLayerNode::Group {
                id: "group-1".into(),
                name: "Group / Nested".into(),
                visible: false,
                opacity: 0.5,
                blend_mode: "screen".into(),
                transform: RasterTransform { x: 1.0, y: -2.0, scale_x: 1.5, scale_y: 0.5, rotation: 12.0 },
                mask: None,
                children: vec![
                    RasterLayerNode::Pixel {
                        id: "pixel-2".into(),
                        name: "Child Pixel".into(),
                        visible: true,
                        opacity: 0.75,
                        blend_mode: "multiply".into(),
                        transform: RasterTransform::default(),
                        mask: None,
                        width: None,
                        height: None,
                        image_key: None,
                    },
                    RasterLayerNode::Group { id: "group-2".into(), name: "Nested Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() },
                ],
            },
            RasterLayerNode::Adjustment { id: "adjust-1".into(), name: "Curves & Co".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "curves".into(), params },
        ],
    };
    store::os_store::test_support::assert_dsl_pack_equivalence_cold(&document, crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot);
    let spec = <RasterSnapshot as store::ArtifactPack>::record_spec().expect("raster declares its pack record spec");
    assert_ne!(store::os_pack::schema_hash(&spec), [0u8; 32]);
    let decoded = decode(&encode(&document)).expect("decode");
    let image = |snapshot: &RasterSnapshot| snapshot.assets.iter().next().and_then(|(_, child)| child.local_owner::<semio_s_artifact_stdio_semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot>()).map(|image| image.as_ref().clone());
    let (before, after) = (image(&document), image(&decoded));
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(decoded);
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
    assert!(before.is_some(), "the fixture asset carries its composed image");
    assert_eq!(after, before, "the pack carries the composed image content beside its handle");
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `RasterMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
/// existing pack round-trip laws (same pattern as `mathematical_pack`'s own
/// `command_envelope_round_trip_holds_for_an_applied_operation`).
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

    let envelope = create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster-command-envelope-demo", crate::standards::v1::subsets::any::schema::empty_raster_document(), None);
    let mut store = ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    // 🔐️ The history ledger refuses an insertion from a store without its domain owner catalog
    // ("edit history insertion requires its exact mutation retirement factory"): a raster store is
    // built with the artifact's own `raster_document_store_owners`, never bare.
    store.install_document_store_owners_exact(crate::spr::raster_document_store_owners());
    store
        .dispatch(ArtifactCommand::Apply {
            mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {
                parent_id: None,
                index: 0,
                layer: Box::new(RasterLayerNode::Pixel {
                    id: "command-envelope-pixel".into(),
                    name: "Command Envelope Pixel".into(),
                    visible: true,
                    opacity: 1.0,
                    blend_mode: "normal".into(),
                    transform: RasterTransform::default(),
                    mask: None,
                    width: Some(32),
                    height: Some(32),
                    image_key: None,
                }),
            })],
            description: None,
        })
        .await
        .expect("apply");
    let edit: &Edit<RasterMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<RasterSnapshot, RasterMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
    store::os_store::test_support::close_plain_test_store(&mut store);
}
//#endregion 🔖️CommandEnvelopeTests
