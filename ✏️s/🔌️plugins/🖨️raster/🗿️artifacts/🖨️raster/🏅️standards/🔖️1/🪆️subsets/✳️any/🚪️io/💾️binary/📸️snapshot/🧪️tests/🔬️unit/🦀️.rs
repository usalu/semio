use crate::standards::v1::subsets::any::io::binary::snapshot::*;
use crate::mutations::create_layer;
use crate::op::RasterMutation;
use crate::RasterOwnedMap;
use crate::{RasterImageAsset, RasterLayerMask, RasterLayerNode, RasterTransform, RASTER_DOCUMENT_SCHEMA};

#[semio_framework_async_macros::async_test]
async fn mask_asset_and_transform_round_trip_through_text_and_pack() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧬️schema/🧫️fixtures/🎭️mask/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mask:RasterLayerMask=semio_framework_pack_json::from_json_str(&case["mask"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let actual:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&mask)).unwrap();
        assert_eq!(actual["imageKey"],case["mask"]["imageKey"]);
        for field in ["x","y","a","b","c","d"] {assert_eq!(actual["transform"][field].as_f64(),case["mask"]["transform"][field].as_f64());}
        let mut document=crate::standards::v1::subsets::any::io::text::snapshot::empty_raster_document();
        if let RasterLayerNode::Pixel {mask:target,..}=&mut document.layers[0] {*target=Some(mask);}
        store::os_store::test_support::assert_dsl_pack_equivalence_cold(&document,crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot);
        crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
    }
}

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_and_agrees_with_dsl() {
    let document = crate::standards::v1::subsets::any::schema::raster_image_test_snapshot();
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
    params.insert("brightness".into(), semio_framework_value::DslValue::float(0.06)).expect("bounded fixture operation succeeds");
    params.insert("label".into(), semio_framework_value::DslValue::String("Warm \"Curve\"".to_string())).expect("bounded fixture operation succeeds");
    params.insert("enabled".into(), semio_framework_value::DslValue::Bool(true)).expect("bounded fixture operation succeeds");
    params.insert("fallback".into(), semio_framework_value::DslValue::Null).expect("bounded fixture operation succeeds");
    params
        .insert(
            "curves".into(),
            semio_framework_value::DslValue::Array(vec![
                semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::float(0.0), semio_framework_value::DslValue::float(0.0)]),
                semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::float(0.25), semio_framework_value::DslValue::float(0.2)]),
                semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::float(1.0), semio_framework_value::DslValue::float(1.0)]),
            ]),
        )
        .expect("bounded fixture operation succeeds");
    params.insert("nested".into(), semio_framework_value::DslValue::Object(vec![("inner".to_string(), semio_framework_value::DslValue::float(1.5))])).expect("bounded fixture operation succeeds");
    let document = RasterSnapshot {
        schema: RASTER_DOCUMENT_SCHEMA.into(),
        id: "doc-1".into(),
        title: Some("Representative \"Doc\"".into()),
        assets,
        layers: vec![
            RasterLayerNode::Pixel {
                id: "pixel-1".into(),
                name: "Pixel One".into(),
                visible: true, locked: false,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: Some(RasterLayerMask { enabled: true, linked: false, invert: true, width: Some(64), height: None, image_key:Some("asset-1".into()), transform:RasterTransform {x:-4.0,y:2.0,..RasterTransform::default()} }),
                width: Some(256),
                height: Some(256),
                image_key: Some("asset-1".into()),
            },
            RasterLayerNode::Group {
                id: "group-1".into(),
                name: "Group / Nested".into(),
                visible: false, locked: false,
                opacity: 0.5,
                blend_mode: "screen".into(),
                transform: RasterTransform {x:1.0,y:-2.0,a:1.5,b:0.25,c:-0.5,d:0.75},
                mask: None,
                children: vec![
                    RasterLayerNode::Pixel {
                        id: "pixel-2".into(),
                        name: "Child Pixel".into(),
                        visible: true, locked: false,
                        opacity: 0.75,
                        blend_mode: "multiply".into(),
                        transform: RasterTransform::default(),
                        mask: None,
                        width: None,
                        height: None,
                        image_key: None,
                    },
                    RasterLayerNode::Group { id: "group-2".into(), name: "Nested Group".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() },
                ],
            },
            RasterLayerNode::Adjustment { id: "adjust-1".into(), name: "Curves & Co".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "curves".into(), params },
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

    let envelope = create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster-command-envelope-demo", crate::standards::v1::subsets::any::io::text::snapshot::empty_raster_document(), None);
    let mut store = ArtifactStore::new(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    // 🔐️ The history ledger refuses an insertion from a store without its domain owner catalog
    // ("edit history insertion requires its exact mutation retirement factory"): a raster store is
    // built with the artifact's own `raster_document_store_owners`, never bare.
    store.install_document_store_owners_exact(crate::host::owned::raster_document_store_owners());
    store
        .dispatch(ArtifactCommand::Apply {
            mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {
                parent_id: None,
                index: 0,
                layer: Box::new(RasterLayerNode::Pixel {
                    id: "command-envelope-pixel".into(),
                    name: "Command Envelope Pixel".into(),
                    visible: true, locked: false,
                    opacity: 1.0,
                    blend_mode: "normal".into(),
                    transform: RasterTransform::default(),
                    mask: None,
                    width: Some(32),
                    height: Some(32),
                    image_key: None,
                }),
            })],
            transaction: None,
        })
        .await
        .expect("apply");
    let edit: &Edit<RasterMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<RasterSnapshot, RasterMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
    store::os_store::test_support::close_plain_test_store(&mut store);
}
//#endregion 🔖️CommandEnvelopeTests
