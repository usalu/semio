use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_mp4_editor();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, MP4_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Mp4Editor as ArtifactEditor>::DIALECT, MP4_DIALECT);
}

#[test]
fn large_payload_metadata_edit_uses_compact_native_event_and_exact_inverse_admission() {
    let mut snapshot = Mp4Snapshot::default();
    let mut track = Mp4Track::default();
    track.track_id = 1;
    track.samples.push(Mp4Sample { data: vec![7; 2 * 1_024 * 1_024], duration: 1_000, cts_offset: 0, sync: true });
    snapshot.tracks.push(track);
    let event = editing::SnapshotEditEvent::SetValue { path: "/ftyp/minorVersion".into(), value: dsl::DslValue::Number(dsl::Number::UInt(42)) };
    assert!(<Mp4Editor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&event, &snapshot));
    let emit = <Mp4Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).expect("ftyp edit emits");
    let [Mp4Mutation::SetFtyp(payload)] = emit.artifact_mutations.as_slice() else { panic!("large-payload ftyp edit must remain compact") };
    assert_eq!(payload.ftyp.minor_version, 42);
    let bytes = <Mp4Mutation as protocol::OpBinary>::encode_op(&emit.artifact_mutations[0]).expect("compact event encodes");
    assert!(bytes.len() < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
    let next = protocol::MutationDiff::apply(<Mp4Mutation as protocol::Mutation<Mp4Snapshot>>::diff(&emit.artifact_mutations[0], &snapshot).diff(), &snapshot).expect("compact event applies");
    assert_eq!(next.ftyp.minor_version, 42);
    assert_eq!(next.tracks[0].samples[0].data, snapshot.tracks[0].samples[0].data);
    let inverse = <Mp4Mutation as protocol::Mutation<Mp4Snapshot>>::inverse(&emit.artifact_mutations[0], &snapshot);
    assert!(inverse.iter().all(|mutation| <Mp4Mutation as protocol::OpBinary>::encode_op(mutation).is_ok_and(|bytes| bytes.len() < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES)));
    let remove = editing::SnapshotEditEvent::RemoveValue { path: "/tracks/0/samples/0".into() };
    assert!(!<Mp4Editor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&remove, &snapshot), "a compact removal with an oversized exact inverse must be refused before publication");
}
