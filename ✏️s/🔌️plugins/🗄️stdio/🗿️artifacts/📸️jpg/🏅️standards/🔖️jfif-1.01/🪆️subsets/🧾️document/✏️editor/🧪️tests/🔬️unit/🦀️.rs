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
}
