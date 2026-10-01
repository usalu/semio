use super::*;

#[test]
fn owned_mime_constraint_matches_independent_serde_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📜️definition/🔣️.json")).unwrap();
    for row in vectors["cases"].as_array().unwrap() {
        let mut source: serde_json::Value = serde_json::from_str(ARTIFACT_DEFINITION_SCHEMA).unwrap();
        source["representations"][0]["mimes"] = row["mimes"].clone();
        let expected = source["representations"].as_array().unwrap().iter().all(|representation| representation["mimes"].as_array().unwrap().is_empty());
        let admitted = validate_definition_schema(&serde_json::to_string(&source).unwrap()).is_ok();
        assert_eq!(admitted, expected);
        assert_eq!(admitted, row["accepted"].as_bool().unwrap());
    }
    definition().unwrap();
    assert!(formats().unwrap().iter().all(|format| format.mimes.is_empty()));
}

#[test]
fn pack_only_snapshot_has_no_relational_codec() {
    let codec = store::ArtifactCodec::bare::<EpwSnapshot, EpwMutation>(STDIO_EPW_DOCUMENT_SCHEMA);
    assert!(codec.snapshot_sqlite.is_none());
    declaration(definition().unwrap()).unwrap();
}
