use super::*;

/// 🧬️ Registers the document schema tiff's declaration contributes — the contract every snapshot edit validates against;
/// a fixture editor runs without the plugin assembly that publishes it.
fn register_document_schema() {
    framework_schema::register_artifact_schema_descriptors(vec![crate::standards::v6_0::subsets::document::schema::tiff_artifact_schema_descriptor()]).expect("the tiff document schema registers");
}

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_tiff_any_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, TIFF_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<TiffAnyEditor as ArtifactEditor>::DIALECT, TIFF_ANY_DIALECT);
}

#[test]
fn large_raster_byte_order_edit_uses_compact_native_event() {
    register_document_schema();
    let mut snapshot = TiffSnapshot::default();
    snapshot.pixels = vec![7; 2 * 1_024 * 1_024];
    let event = editing::SnapshotEditEvent::SetValue { path: "/byteOrder".into(), value: dsl::DslValue::String("bigEndian".into()) };
    assert!(<TiffAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event, &snapshot));
    let emit = <TiffAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("byte-order edit emits");
    let [TiffMutation::ChangeByteOrder(payload)] = emit.artifact_mutations.as_slice() else { panic!("byte-order edit must use its native leaf") };
    assert_eq!(payload.byte_order, crate::schema::snapshot::TiffByteOrder::BigEndian);
    assert!(<TiffMutation as protocol::OpBinary>::encode_op(&emit.artifact_mutations[0]).expect("byte-order mutation encodes").len() < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
    let next = protocol::MutationDiff::apply(<TiffMutation as protocol::Mutation<TiffSnapshot>>::diff(&emit.artifact_mutations[0], &snapshot).diff(), &snapshot).expect("byte-order mutation applies");
    assert_eq!(next.byte_order, crate::schema::snapshot::TiffByteOrder::BigEndian);
    assert_eq!(next.pixels, snapshot.pixels);
    let inverse = <TiffMutation as protocol::Mutation<TiffSnapshot>>::inverse(&emit.artifact_mutations[0], &snapshot);
    assert!(inverse.iter().all(|mutation| <TiffMutation as protocol::OpBinary>::encode_op(mutation).is_ok_and(|bytes| bytes.len() < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES)));
    let restored = inverse.into_iter().fold(next, |current, mutation| protocol::MutationDiff::apply(<TiffMutation as protocol::Mutation<TiffSnapshot>>::diff(&mutation, &current).diff(), &current).expect("byte-order inverse applies"));
    assert_eq!(restored, snapshot);

    let native_base = crate::schema::demo_tiff_snapshot();
    let native_emit = <TiffAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &native_base).expect("native byte-order edit emits");
    let native_edited = protocol::MutationDiff::apply(<TiffMutation as protocol::Mutation<TiffSnapshot>>::diff(&native_emit.artifact_mutations[0], &native_base).diff(), &native_base).expect("native byte-order mutation applies");
    let native = crate::io::encode_tiff(&native_edited).expect("edited TIFF encodes");
    assert_eq!(&native[..2], b"MM");
    let reopened = crate::io::decode_tiff(&native).expect("edited native TIFF reopens");
    assert_eq!(reopened.byte_order, crate::schema::snapshot::TiffByteOrder::BigEndian);
    assert_eq!(reopened.pixels, native_edited.pixels);
}

#[test]
fn payload_detail_edits_publish_the_exact_requested_value() {
    register_document_schema();
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🧫️fixtures/🔣️.json"))).unwrap();
    let mut snapshot = TiffSnapshot::default();
    snapshot.pixels = vec![7, 9];
    let base: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&dsl::ToValue::to_value(&snapshot))).unwrap();
    for row in fixture["payload"]["cases"].as_array().unwrap() {
        let mut event = row["event"].clone();
        event["path"] = format!("/pixels{}", event["path"].as_str().unwrap()).into();
        if let Some(from) = event.get_mut("from") { *from = format!("/pixels{}", from.as_str().unwrap()).into(); }
        let event: editing::SnapshotEditEvent = pack::json::from_json_str(&event.to_string()).unwrap();
        let emitted = <TiffAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap_or_else(|error| panic!("{}: {error:?}", row["id"]));
        let mut next = snapshot.clone();
        for mutation in emitted.artifact_mutations {
            next = protocol::MutationDiff::apply(<TiffMutation as protocol::Mutation<TiffSnapshot>>::diff(&mutation, &next).diff(), &next).unwrap();
        }
        let mut expected = base.clone();
        *expected.pointer_mut("/pixels").unwrap() = row["expected"].clone();
        let actual: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&dsl::ToValue::to_value(&next))).unwrap();
        assert_eq!(actual, expected, "{}", row["id"]);
    }
}
