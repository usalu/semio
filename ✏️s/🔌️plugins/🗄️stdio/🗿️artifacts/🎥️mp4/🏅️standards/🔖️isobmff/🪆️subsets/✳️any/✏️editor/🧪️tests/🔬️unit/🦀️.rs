use super::*;

fn register_mp4_snapshot_schema() {
    semio_framework_schema::register_artifact_schema_descriptor(crate::standards::isobmff::subsets::any::schema::mp4_artifact_schema_descriptor());
}

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
    register_mp4_snapshot_schema();
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
    let restored = inverse.into_iter().fold(next.clone(), |current, mutation| protocol::MutationDiff::apply(<Mp4Mutation as protocol::Mutation<Mp4Snapshot>>::diff(&mutation, &current).diff(), &current).expect("ftyp inverse applies"));
    assert_eq!(restored, snapshot);
    let native = crate::standards::isobmff::subsets::any::io::encode_mp4(&next);
    let reopened = crate::standards::isobmff::subsets::any::io::decode_mp4(&native).expect("edited native MP4 reopens");
    assert_eq!(reopened.ftyp.minor_version, 42);
    assert_eq!(reopened.tracks[0].samples[0].data, snapshot.tracks[0].samples[0].data);
    let remove = editing::SnapshotEditEvent::RemoveValue { path: "/tracks/0/samples/0".into() };
    assert!(<Mp4Editor as editing::SnapshotEditingEditor>::snapshot_edit_is_admitted(&remove, &snapshot), "bounded admission must not clone or encode the addressed payload");
    assert!(<Mp4Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&remove, &snapshot).is_err(), "an oversized exact inverse must still be refused before publication");
    assert_eq!(snapshot.tracks[0].samples[0].data, vec![7; 2 * 1_024 * 1_024]);
}

#[test]
fn payload_detail_edits_publish_the_exact_requested_value() {
    register_mp4_snapshot_schema();
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🧫️fixtures/🔣️.json"))).unwrap();
    let mut snapshot = Mp4Snapshot::default();
    let mut track = Mp4Track::default();
    track.samples.push(Mp4Sample { data: vec![7, 9], duration: 1000, cts_offset: 0, sync: true });
    snapshot.tracks.push(track);
    let base: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&dsl::ToValue::to_value(&snapshot))).unwrap();
    for row in fixture["payload"]["cases"].as_array().unwrap() {
        let mut event = row["event"].clone();
        event["path"] = format!("/tracks/0/samples/0/data{}", event["path"].as_str().unwrap()).into();
        if let Some(from) = event.get_mut("from") { *from = format!("/tracks/0/samples/0/data{}", from.as_str().unwrap()).into(); }
        let event: editing::SnapshotEditEvent = pack::json::from_json_str(&event.to_string()).unwrap();
        let emitted = <Mp4Editor as editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, &snapshot).unwrap_or_else(|error| panic!("{}: {error:?}", row["id"]));
        let mut next = snapshot.clone();
        for mutation in emitted.artifact_mutations {
            next = protocol::MutationDiff::apply(<Mp4Mutation as protocol::Mutation<Mp4Snapshot>>::diff(&mutation, &next).diff(), &next).unwrap();
        }
        let mut expected = base.clone();
        *expected.pointer_mut("/tracks/0/samples/0/data").unwrap() = row["expected"].clone();
        let actual: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&dsl::ToValue::to_value(&next))).unwrap();
        assert_eq!(actual, expected, "{}", row["id"]);
    }
}
