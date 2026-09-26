//! 🪢️ The canonical checkpoint pair law, replayed from the language-agnostic fixture its TypeScript twin reads
//! (`🧫️fixtures/📇️directory/🪢️canonical-checkpoint-pair-v1.json`). Every body and digest in it was framed and hashed
//! by an independent encoder (Python's hashlib, `wp-wg10/gen-canonical-pair-fixture.py`), never by this decoder.

use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../../../🧫️fixtures/📇️directory/🪢️canonical-checkpoint-pair-v1.json")).expect("canonical pair fixture")
}

fn hex_bytes(value: &serde_json::Value) -> Vec<u8> {
    let text = value.as_str().expect("hex");
    (0..text.len()).step_by(2).map(|index| u8::from_str_radix(&text[index..index + 2], 16).expect("hex byte")).collect()
}

fn hash(value: &serde_json::Value) -> ArtifactHash {
    ArtifactHash::parse_hex(value.as_str().expect("hex32")).expect("hash")
}

fn frontier(value: &serde_json::Value) -> ArtifactFrontier {
    let mut chain = [0u8; 32];
    for (slot, byte) in chain.iter_mut().zip(value["chainHash"].as_array().expect("chain")) {
        *slot = byte.as_u64().expect("byte") as u8;
    }
    ArtifactFrontier {
        document_id: value["documentId"].as_str().expect("document").to_string(),
        head_edit_ordinal: value["headEditOrdinal"].as_u64().expect("ordinal"),
        head_edit_id: value["headEditId"].as_str().expect("head").to_string(),
        last_commit_seq: value["lastCommitSeq"].as_u64().expect("commit"),
        chain_hash: ArtifactHash(chain),
    }
}

fn part(value: &serde_json::Value) -> Vec<u8> {
    vec![value["byte"].as_u64().expect("byte") as u8; value["length"].as_u64().expect("length") as usize]
}

#[test]
fn the_fixture_limits_are_the_wire_constants() {
    let fixture = fixture();
    assert_eq!(fixture["mediaType"], CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1);
    assert_eq!(fixture["limits"]["headerBytes"], CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES as u64);
    assert_eq!(fixture["limits"]["recordBytes"], CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES as u64);
    assert_eq!(fixture["limits"]["records"], CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS as u64);
    assert_eq!(fixture["limits"]["pairBytes"], CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES as u64);
}

#[test]
fn every_fixture_pair_decodes_to_its_selection_and_its_verified_bytes() {
    for pair in fixture()["pairs"].as_array().expect("pairs") {
        let decoded = decode_canonical_checkpoint_pair_v1(&hex_bytes(&pair["bodyHex"])).unwrap_or_else(|refusal| panic!("{}: {refusal}", pair["id"]));
        let selection = &pair["selection"];
        assert_eq!(decoded.scope, DocumentScope::new(selection["spaceId"].as_str().unwrap(), selection["documentId"].as_str().unwrap()), "{}", pair["id"]);
        assert_eq!(decoded.descriptor_digest_v1, hash(&selection["descriptorDigestV1"]));
        assert_eq!(decoded.active_checkpoint_id, hash(&selection["checkpointId"]));
        assert_eq!(decoded.baseline_frontier, frontier(&selection["baselineFrontier"]));
        assert_eq!((decoded.pack.sha256, decoded.spr.sha256, decoded.aggregate_sha256), (hash(&pair["packSha256"]), hash(&pair["sprSha256"]), hash(&pair["aggregateSha256"])));
        assert_eq!((decoded.pack_bytes, decoded.spr_bytes), (part(&pair["pack"]), part(&pair["spr"])), "{}", pair["id"]);
    }
}

#[test]
fn every_fixture_refusal_is_named_and_never_half_accepted() {
    for refusal in fixture()["refusals"].as_array().expect("refusals") {
        let answer = decode_canonical_checkpoint_pair_v1(&hex_bytes(&refusal["bodyHex"]));
        assert_eq!(answer.map(|_| ()).map_err(CanonicalCheckpointPairRefusalV1::code), Err(refusal["refusal"].as_str().unwrap()), "{}", refusal["id"]);
    }
}

#[test]
fn a_pair_is_admitted_only_as_the_checkpoint_the_hub_authorized_for_the_open() {
    let fixture = fixture();
    for case in fixture["admissions"].as_array().expect("admissions") {
        let pair = fixture["pairs"].as_array().unwrap().iter().find(|pair| pair["id"] == case["pair"]).expect("admission pair");
        let decoded = decode_canonical_checkpoint_pair_v1(&hex_bytes(&pair["bodyHex"])).expect("fixture pair decodes");
        let expected = DocumentOpenCheckpointV1 {
            checkpoint_id: case["expected"]["checkpointId"].as_str().unwrap().to_string(),
            descriptor_digest_v1: case["expected"]["descriptorDigestV1"].as_str().unwrap().to_string(),
            baseline_frontier: frontier(&case["expected"]["baselineFrontier"]),
            aggregate_sha256: case["expected"]["aggregateSha256"].as_str().unwrap().to_string(),
        };
        let scope = DocumentScope::new(case["scope"]["spaceId"].as_str().unwrap(), case["scope"]["documentId"].as_str().unwrap());
        assert_eq!(decoded.admit(&scope, &expected).map_err(CanonicalCheckpointPairRefusalV1::code), case["refusal"].as_str().map_or(Ok(()), Err), "{}", case["id"]);
    }
}
