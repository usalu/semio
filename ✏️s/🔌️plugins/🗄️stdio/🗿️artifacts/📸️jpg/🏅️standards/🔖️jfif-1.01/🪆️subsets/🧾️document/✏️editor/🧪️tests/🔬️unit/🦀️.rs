use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_jpg_any_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, JPG_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<JpgAnyEditor as ArtifactEditor>::DIALECT, JPG_ANY_DIALECT);
}

#[test]
fn large_raster_quality_edit_uses_compact_native_event() {
    let mut snapshot = JpgSnapshot::default();
    snapshot.pixels = vec![7; 2 * 1_024 * 1_024];
    let event = editing::SnapshotEditEvent::SetValue { path: "/reEncodeQuality".into(), value: dsl::DslValue::Number(dsl::Number::UInt(75)) };
    assert!(<JpgAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event, &snapshot));
    let emit = <JpgAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("quality edit emits");
    let [JpgMutation::ChangeReEncodeQuality(payload)] = emit.artifact_mutations.as_slice() else { panic!("quality edit must use its native leaf") };
    assert_eq!(payload.quality, Some(75));
    assert!(<JpgMutation as protocol::OpBinary>::encode_op(&emit.artifact_mutations[0]).expect("quality mutation encodes").len() < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
    let next = protocol::MutationDiff::apply(<JpgMutation as protocol::Mutation<JpgSnapshot>>::diff(&emit.artifact_mutations[0], &snapshot).diff(), &snapshot).expect("quality mutation applies");
    assert_eq!(next.re_encode_quality, Some(75));
    assert_eq!(next.pixels, snapshot.pixels);
    let inverse = <JpgMutation as protocol::Mutation<JpgSnapshot>>::inverse(&emit.artifact_mutations[0], &snapshot);
    assert!(inverse.iter().all(|mutation| <JpgMutation as protocol::OpBinary>::encode_op(mutation).is_ok_and(|bytes| bytes.len() < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES)));
    let restored = inverse.into_iter().fold(next, |current, mutation| protocol::MutationDiff::apply(<JpgMutation as protocol::Mutation<JpgSnapshot>>::diff(&mutation, &current).diff(), &current).expect("quality inverse applies"));
    assert_eq!(restored, snapshot);

    let native_base = crate::schema::demo_jpg_snapshot();
    let native_emit = <JpgAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &native_base).expect("native quality edit emits");
    let native_edited = protocol::MutationDiff::apply(<JpgMutation as protocol::Mutation<JpgSnapshot>>::diff(&native_emit.artifact_mutations[0], &native_base).diff(), &native_base).expect("native quality mutation applies");
    let base_bytes = crate::io::encode_jpg(&native_base).expect("base JPEG encodes");
    let edited_bytes = crate::io::encode_jpg(&native_edited).expect("edited JPEG encodes");
    assert_ne!(edited_bytes, base_bytes, "the quality edit must affect the native JPEG export");
    let reopened = crate::io::decode_jpg(&edited_bytes).expect("edited native JPEG reopens");
    assert_eq!((reopened.width, reopened.height), (native_edited.width, native_edited.height));
}

#[test]
fn payload_detail_edits_publish_the_exact_requested_value() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🧫️fixtures/🔣️.json"))).unwrap();
    let mut snapshot = JpgSnapshot::default();
    snapshot.pixels = vec![7, 9];
    let base: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&dsl::ToValue::to_value(&snapshot))).unwrap();
    for row in fixture["payload"]["cases"].as_array().unwrap() {
        let mut event = row["event"].clone();
        event["path"] = format!("/pixels{}", event["path"].as_str().unwrap()).into();
        if let Some(from) = event.get_mut("from") { *from = format!("/pixels{}", from.as_str().unwrap()).into(); }
        let event: editing::SnapshotEditEvent = pack::json::from_json_str(&event.to_string()).unwrap();
        let emitted = <JpgAnyEditor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap_or_else(|error| panic!("{}: {error:?}", row["id"]));
        let mut next = snapshot.clone();
        for mutation in emitted.artifact_mutations {
            next = protocol::MutationDiff::apply(<JpgMutation as protocol::Mutation<JpgSnapshot>>::diff(&mutation, &next).diff(), &next).unwrap();
        }
        let mut expected = base.clone();
        *expected.pointer_mut("/pixels").unwrap() = row["expected"].clone();
        let actual: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&dsl::ToValue::to_value(&next))).unwrap();
        assert_eq!(actual, expected, "{}", row["id"]);
    }
}
