use super::*;

/// 🧫️ Every language-agnostic vector: a valid source parses and re-emits byte-identically, every
/// invalid source is refused (the `schemaValid` ones only by this parser's own canonical-form and
/// progress laws — the JSON Schema accepts them, which the Ajv oracle in `🌎️hub/🧪️tests` asserts).
#[test]
fn document_check_in_v1_matches_language_neutral_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("check-in corpus");
    for vector in fixture["valid"]["requests"].as_array().unwrap() {
        let source = vector["source"].as_str().unwrap();
        let request = DocumentCheckInV1::parse_canonical_json(source).unwrap_or_else(|| panic!("valid request {} refused", vector["name"]));
        assert_eq!(request.canonical_json().as_deref(), Some(source), "request {} re-emits canonically", vector["name"]);
    }
    for vector in fixture["valid"]["statuses"].as_array().unwrap() {
        let source = vector["source"].as_str().unwrap();
        let status = DocumentCheckInStatusV1::parse_canonical_json(source).unwrap_or_else(|| panic!("valid status {} refused", vector["name"]));
        assert_eq!(status.canonical_json().as_deref(), Some(source), "status {} re-emits canonically", vector["name"]);
        assert_eq!(status.phase.is_terminal(), matches!(status.phase, DocumentCheckInPhaseV1::Ready | DocumentCheckInPhaseV1::Failed | DocumentCheckInPhaseV1::Cancelled));
    }
    for vector in fixture["invalid"]["requests"].as_array().unwrap() {
        assert!(DocumentCheckInV1::parse_canonical_json(vector["source"].as_str().unwrap()).is_none(), "invalid request {} accepted", vector["name"]);
    }
    for vector in fixture["invalid"]["statuses"].as_array().unwrap() {
        assert!(DocumentCheckInStatusV1::parse_canonical_json(vector["source"].as_str().unwrap()).is_none(), "invalid status {} accepted", vector["name"]);
    }
}

/// 🔁️ The edited-frontier grammar round-trips the directory authority frontier and never names genesis.
#[test]
fn edited_artifact_frontier_round_trips_and_refuses_genesis() {
    let wire = EditedArtifactFrontierV1 { document_id: "map-a".into(), head_edit_ordinal: 3, head_edit_id: "edit-3".into(), last_commit_seq: 2, chain_sha256: "22".repeat(32) };
    let frontier = wire.artifact_frontier().expect("edited frontier");
    assert_eq!(EditedArtifactFrontierV1::of_artifact_frontier(&frontier), Some(wire));
    let genesis = super::super::ArtifactFrontier { document_id: "map-a".into(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: super::super::ArtifactHash([0; 32]) };
    assert_eq!(EditedArtifactFrontierV1::of_artifact_frontier(&genesis), None);
}
