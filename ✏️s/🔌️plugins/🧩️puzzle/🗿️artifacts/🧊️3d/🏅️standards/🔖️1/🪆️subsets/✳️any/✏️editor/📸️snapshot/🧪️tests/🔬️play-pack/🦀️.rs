//! 📦️ The play snapshot packs as exactly its typed record: one record identity, byte-identical packs, a lossless round trip.

use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

#[test]
fn scene_json_round_trip_preserves_the_complete_typed_document() {
    use semio_framework_value::{FromValue, ToValue};
    let text = include_str!("../🧫️fixtures/🔣️.json");
    let oracle: Puzzle3dSnapshot = serde_json::from_str(text).expect("independent serde document");
    let document: Puzzle3dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("native document");
    assert_eq!(document, oracle);
    let scene = crate::puzzle3d_scene_snapshot_from_document(&document);
    let exported = semio_framework_pack_json::to_json_string(&scene);
    let decoded = crate::Puzzle3dSceneSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&exported, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("export JSON"))).expect("scene decode");
    let reopened = crate::puzzle3d_snapshot_from_host_snapshot(&decoded).expect("reopen typed document");
    assert_eq!(reopened, document);
    assert_eq!(serde_json::to_value(&reopened).expect("oracle output"), serde_json::to_value(&oracle).expect("oracle expected"));
    assert_eq!(reopened.to_value(), document.to_value());
}

#[test]
fn play_snapshot_pack_shares_the_typed_record_identity_and_round_trips() {
    let spec = <crate::Puzzle3dPlaySnapshot as pack::snapshot::ArtifactPack>::record_spec().expect("play snapshot declares its record spec");
    assert_eq!(pack::record::schema_hash(&spec), pack::record::schema_hash(&Puzzle3dSnapshot::__dsl_spec()));
    assert_ne!(pack::record::schema_hash(&spec), [0u8; 32]);
    let play = crate::Puzzle3dPlaySnapshot::from_typed(Puzzle3dSnapshot::default());
    let bytes = pack::snapshot::ArtifactPack::encode_pack(&play);
    assert_eq!(bytes, pack::snapshot::ArtifactPack::encode_pack(play.typed()));
    assert_eq!(<crate::Puzzle3dPlaySnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).expect("decode play pack"), play);
}

#[test]
fn play_snapshot_diff_keeps_shared_source_and_cache_until_success() {
    use semio_s_artifact_puzzle_3d_diff::{Puzzle3dDiff, Puzzle3dObjectsDelta, Puzzle3dReferencesDelta};
    use semio_s_artifact_puzzle_3d_model::Puzzle3dArtifact;
    use std::sync::Arc;

    let text = include_str!("../🧫️fixtures/🔣️.json");
    let original: Puzzle3dSnapshot = serde_json::from_str(text).unwrap();
    let source = crate::Puzzle3dPlaySnapshot::from_typed(original.clone());
    let before = source.typed_arc();
    let mut candidate = source.clone();
    let before_view = candidate.value() as *const _;
    let rejected = Puzzle3dDiff {
        domain: Some("rejected".into()),
        objects: Some(Puzzle3dObjectsDelta::removal_by_id("missing", 0)),
        references: Some(Puzzle3dReferencesDelta::removal_by_id("missing", 0)),
        ..Default::default()
    };
    let error = protocol::apply_diff_cold_into(&rejected, &mut candidate).unwrap_err();
    assert_eq!(error.code, "mutation.apply.missing-target");
    assert_eq!(error.target, ["document", "objects", "removed", "0"]);
    assert!(Arc::ptr_eq(&before, &candidate.typed_arc()));
    assert_eq!(before_view, candidate.value() as *const _);
    assert_eq!(candidate.typed(), &original);

    let mut expected: serde_json::Value = serde_json::from_str(text).unwrap();
    expected["domain"] = "edited".into();
    expected["objects"] = serde_json::json!([]);
    let applied = Puzzle3dDiff {
        domain: Some("edited".into()),
        objects: Some(Puzzle3dObjectsDelta::removal_by_id("assembly", 0)),
        ..Default::default()
    };
    protocol::apply_diff_cold_into(&applied, &mut candidate).unwrap();
    let expected: Puzzle3dSnapshot = serde_json::from_value(expected).unwrap();
    assert_eq!(candidate.typed(), &expected);
    assert_eq!(candidate.value(), crate::Puzzle3dPlaySnapshot::from_typed(expected.clone()).value());
    assert_eq!(source.typed(), &original);
    assert_ne!(source.value(), candidate.value());
    assert!(Arc::ptr_eq(&before, &source.typed_arc()));

    let replacement = Puzzle3dDiff {
        artifact: Some(Box::new(Puzzle3dArtifact::from_snapshot(original))),
        ..applied
    };
    protocol::apply_diff_cold_into(&replacement, &mut candidate).unwrap();
    assert_eq!(candidate.typed(), &expected);
}
