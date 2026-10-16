use super::*;

#[test]
fn native_document_codec_requires_only_declared_snapshot_capabilities() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let relational = row["relational"].as_bool().unwrap();
        let codec = if relational { bare_artifact_codec::<DemoSnapshot, DemoMutation>("capability.relational") } else { bare_artifact_codec::<RetainedTextSnapshot, RetainedTextMutation>("capability.pack") };
        assert_eq!(!codec.extension.is_empty(), row["native"].as_bool().unwrap());
        assert_eq!(codec.snapshot_sqlite.is_some(), relational);
        assert_eq!(codec.snapshot_sqlite.is_some(), row["sqlite"].as_bool().unwrap());
        if relational {
            let required = artifact_codec::<DemoSnapshot, DemoMutation>("capability.relational");
            assert!(required.snapshot_sqlite.is_some());
            assert_eq!(codec.snapshot_sqlite.as_ref().unwrap().schema, required.snapshot_sqlite.as_ref().unwrap().schema);
        }
    }
}
