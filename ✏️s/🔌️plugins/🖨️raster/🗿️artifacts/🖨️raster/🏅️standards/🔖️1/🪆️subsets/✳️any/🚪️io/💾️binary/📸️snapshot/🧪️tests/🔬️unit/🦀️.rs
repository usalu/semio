use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_model::io::binary::snapshot::*;
use semio_s_artifact_raster_raster_mutations_layers::create_layer;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::RasterOwnedMap;
use semio_s_artifact_raster_raster_model::{mint_raster_image_child, RasterLayerMask, RasterLayerNode, RasterTransform, RASTER_DOCUMENT_SCHEMA};

#[semio_framework_async_macros::async_test]
async fn mask_asset_and_transform_round_trip_through_text_and_pack() {
    let fixture:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../🧫️fixtures/🔬️contract/🎭️mask/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mask:RasterLayerMask=semio_framework_pack_json::from_json_str(&case["mask"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let actual:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&mask)).unwrap();
        assert_eq!(actual["imageKey"],case["mask"]["imageKey"]);
        for field in ["x","y","a","b","c","d"] {assert_eq!(actual["transform"][field].as_f64(),case["mask"]["transform"][field].as_f64());}
        let mut document=semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_document();
        if let RasterLayerNode::Pixel {mask:target,..}=&mut document.layers[0] {*target=Some(mask);}
        semio_os_store_test_support::assert_dsl_pack_equivalence_cold(&document,semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot);
        semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(document);
    }
}

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_and_agrees_with_dsl() {
    let document = semio_s_artifact_raster_raster_io::testing::raster_image_test_snapshot();
    semio_os_store_test_support::assert_dsl_pack_equivalence_cold(&document, semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot);
    let bytes = encode(&document);
    let decoded = decode(&bytes).expect("decode");
    assert_eq!(decoded, document);
    semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(decoded);
    semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(document);
}

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_representative_document() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧬️schema/🔑️content-hash/🧪️tests/🔣️.json")).unwrap();
    let image: semio_s_artifact_raster_raster_model::SemioImageSnapshot =
        semio_framework_pack_json::from_json_str(&fixture["image"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let child = mint_raster_image_child("asset-1", &image);
    assert_eq!(child.child_id, format!("raster-asset-{}", fixture["digest"].as_str().unwrap()));
    let mut assets = RasterOwnedMap::new();
    assets.insert("asset-1".into(), child).expect("bounded fixture operation succeeds");
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
    let mut document = RasterSnapshot {
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
    semio_os_store_test_support::assert_dsl_pack_equivalence_cold(&document, semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot);
    let spec = <RasterSnapshot as pack::snapshot::ArtifactPack>::record_spec().expect("raster declares its pack record spec");
    assert_ne!(pack::record::schema_hash(&spec), [0u8; 32]);
    let decoded = decode(&encode(&document)).expect("decode");
    let restored_image = semio_s_artifact_raster_raster_model::raster_image(&decoded.assets, "asset-1");
    semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(decoded);

    let mut changed = image.clone();
    changed.frames[0].rgba8[0] ^= 1;
    document.assets.get_mut("asset-1").unwrap().set_local_owner(std::sync::Arc::new(changed));
    let tampered = encode(&document);
    semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(document);
    assert_eq!(restored_image.as_ref(), Some(&image), "the pack carries the composed image content beside its handle");
    let refusal = match decode(&tampered) {
        Ok(snapshot) => {
            semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(snapshot);
            panic!("a changed image must not be restored under its old identity");
        }
        Err(refusal) => refusal,
    };
    assert!(refusal.to_string().contains("Raster image body does not match its child ID"));
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `RasterMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
/// existing pack round-trip laws (same pattern as `mathematical_pack`'s own
/// `command_envelope_round_trip_holds_for_an_applied_operation`).
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{os_store::document_materialize::create_document_envelope, os_store::store_schema::ArtifactCommand, ArtifactStore};

    let envelope = create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster-command-envelope-demo", semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_document(), None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new::<::semio_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    store.install_document_store_owners_exact(semio_s_artifact_raster_raster_host::raster_document_store_owners().expect("the Raster owner catalog funds")).map_err(|(error, _)| error).expect("the Raster owner catalog installs on a fresh store");
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
        }, &mut semio_plugin::artifact_app_laws::fixture_identity())
        .await
        .expect("apply");
    let edit: &Edit<RasterMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_os_store_test_support::assert_command_envelope_round_trip::<RasterSnapshot, RasterMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
    semio_os_store_test_support::close_plain_test_store(&mut store);
}
//#endregion 🔖️CommandEnvelopeTests

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
