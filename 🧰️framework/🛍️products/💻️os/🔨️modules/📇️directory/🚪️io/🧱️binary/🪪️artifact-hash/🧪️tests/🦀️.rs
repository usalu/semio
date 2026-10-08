use super::*;

#[test]
fn artifact_hash_text_matches_the_neutral_vector_and_independent_formatting() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔐️descriptor-digest/🧫️fixtures/🔣️.json")).unwrap();
    for vector in fixture["hashVectors"].as_array().unwrap() {
        let bytes: Vec<u8> = vector["bytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
        let hash = ArtifactHash(bytes.as_slice().try_into().unwrap());
        let independent: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
        assert_eq!(artifact_hash_hex(&hash), independent);
        let admitted = parse_artifact_hash_hex(vector["text"].as_str().unwrap());
        assert_eq!(admitted.is_some(), vector["admitted"].as_bool().unwrap());
        if let Some(admitted) = admitted { assert_eq!(admitted, hash); }
    }
    println!("[DEBUG] Native hash text IO matched four neutral vectors and independent Rust formatting");
}

#[test]
fn edited_frontier_admission_has_typed_hash_identity_and_genesis_refusal() {
    let wire = EditedArtifactFrontierV1 {document_id:"document".into(),head_edit_ordinal:1,head_edit_id:"edit".into(),last_commit_seq:1,chain_sha256:"01".repeat(32)};
    let admitted = decode_edited_artifact_frontier_v1(&wire).unwrap();
    assert_eq!(admitted.chain_hash, ArtifactHash([1;32]));
    assert_eq!(encode_edited_artifact_frontier_v1(&admitted), Some(wire));
    let mut genesis = admitted; genesis.head_edit_ordinal = 0;
    assert!(encode_edited_artifact_frontier_v1(&genesis).is_none());
    println!("[DEBUG] Native edited frontier conversion admitted typed hash identity and refused genesis");
}
