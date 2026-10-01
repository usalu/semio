use super::*;

#[path = "../🪶️native-encoding/🧪️tests/🦀️.rs"]
mod native_encoding_tests;

impl ArtifactDsl for RetainedTextSnapshot {
    const EXTENSION: &'static str = "retained-text";
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        Ok(Self { text: text.to_string() })
    }
    fn print_dsl(&self) -> String {
        self.text.clone()
    }
}

#[test]
fn native_document_codec_requires_only_declared_snapshot_capabilities() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let relational = row["relational"].as_bool().unwrap();
        let codec = if relational {
            ArtifactCodec::bare::<DemoSnapshot, DemoMutation>("capability.relational")
        } else {
            ArtifactCodec::bare::<RetainedTextSnapshot, RetainedTextMutation>("capability.pack")
        };
        assert_eq!(!codec.extension.is_empty(), row["native"].as_bool().unwrap());
        assert_eq!(codec.snapshot_sqlite.is_some(), relational);
        assert_eq!(codec.snapshot_sqlite.is_some(), row["sqlite"].as_bool().unwrap());
        if relational {
            let required = ArtifactCodec::of::<DemoSnapshot, DemoMutation>("capability.relational");
            assert!(required.snapshot_sqlite.is_some());
            assert_eq!(codec.snapshot_sqlite.as_ref().unwrap().schema, required.snapshot_sqlite.as_ref().unwrap().schema);
        }
    }
}
