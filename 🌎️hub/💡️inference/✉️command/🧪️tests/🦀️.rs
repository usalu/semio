//! 🧪️ Literal boundary vectors exercise the production decoder and canonical protocol writer.

use super::*;

#[test]
fn inference_command_exact_decoder_executes_neutral_bounds_canonical_eof_and_actor_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/✉️inference-command-v1/🔣️.json")).unwrap();
    let source: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/🧾️inference-wal-proof-v1/🔣️.json")).unwrap();
    assert_eq!(fixture["limits"]["commandBytes"], COMMAND_MAX_BYTES);
    assert_eq!(fixture["limits"]["textBytes"], TEXT_MAX_BYTES);
    assert_eq!(fixture["limits"]["dependencyCount"], DEPENDENCY_MAX_COUNT);
    assert_eq!(fixture["limits"]["payloadBytes"], PAYLOAD_MAX_BYTES);
    assert_eq!(fixture["limits"]["integerMaximum"], SAFE_INTEGER_MAX);
    let base = crate::inference::wal::tests::envelope(&source);
    for vector in fixture["vectors"].as_array().unwrap() {
        let mut command = base.clone();
        let change = vector["change"].as_str().unwrap();
        match change {
            "text-max-plus-one" => command.actor.0 = "a".repeat(TEXT_MAX_BYTES + 1),
            "control-text" => command.actor.0.push('\u{1}'),
            "empty-schema" => command.diff.schema.0.clear(),
            "dependency-max-plus-one" => command.dependencies = (0..=DEPENDENCY_MAX_COUNT).map(|index| protocol::MutationId(format!("{index:032x}"))).collect(),
            "duplicate-dependency" => command.dependencies = vec![protocol::MutationId("e".repeat(32)); 2],
            "diff-max-plus-one" => command.diff.payload = vec![0; PAYLOAD_MAX_BYTES + 1],
            "inverse-max-plus-one" => command.inverse.payload = vec![0; PAYLOAD_MAX_BYTES + 1],
            "hlc-actor-max-plus-one" => command.timestamp.actor = SAFE_INTEGER_MAX + 1,
            "hlc-time-max-plus-one" => command.timestamp.physical_ms = SAFE_INTEGER_MAX + 1,
            "hlc-logical-max-plus-one" => command.timestamp.logical = SAFE_INTEGER_MAX + 1,
            "different-actor" => command.actor.0 = command.actor.0.replacen('a', "e", 1),
            "different-scope" => command.document_id.0 = command.document_id.0.replacen('c', "e", 1),
            "different-mutation" => command.mutation_id.0 = "e".repeat(32),
            "none" | "trailing" | "truncated" | "oversize" | "overlong-varint" | "overflow-varint" | "invalid-utf8" => {}
            _ => panic!("unknown command fixture vector"),
        }
        let mut bytes = Vec::new();
        protocol::encode_envelope(&command, &mut bytes);
        match change {
            "trailing" => bytes.push(0),
            "truncated" => { bytes.pop(); }
            "oversize" => bytes.resize(COMMAND_MAX_BYTES + 1, 0),
            "overlong-varint" => { bytes[0] |= 128; bytes.insert(1, 0); }
            "overflow-varint" => { bytes.splice(..1, [255, 255, 255, 255, 255, 255, 255, 255, 255, 2]); }
            "invalid-utf8" => bytes[1] = 255,
            _ => {}
        }
        let outcome = match CanonicalInferenceCommandV1::decode(&bytes) {
            Err(_) => "rejected",
            Ok(decoded) if !decoded.matches_identity(&base.mutation_id.0, &base.document_id.0, &base.actor.0) => "identity-denied",
            Ok(_) => "canonical",
        };
        assert_eq!(outcome, vector["expected"].as_str().unwrap(), "{}", vector["name"]);
        if change == "none" {
            assert_eq!(crate::inference::sha256(&bytes), source["commandHash"].as_str().unwrap());
        }
    }
}
