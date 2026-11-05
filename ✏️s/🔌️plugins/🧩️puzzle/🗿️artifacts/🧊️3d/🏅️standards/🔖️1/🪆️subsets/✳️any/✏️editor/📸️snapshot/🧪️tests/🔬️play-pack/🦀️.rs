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
