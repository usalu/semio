use super::*;

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
}
