//! 📦️ The play snapshot packs as exactly its typed record: one record identity, byte-identical packs, a lossless round trip.

use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

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
