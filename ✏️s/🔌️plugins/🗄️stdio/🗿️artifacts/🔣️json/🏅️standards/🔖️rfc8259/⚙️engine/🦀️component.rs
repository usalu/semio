//! ⚙️ JsonEngine — owns a real `JsonArtifact`.

use crate::artifacts::json::{JsonArtifact, JsonDiff, JsonMutation, JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_json_snapshot() -> JsonSnapshot {
    JsonSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::json::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<JsonSnapshot, JsonMutation>(STDIO_JSON_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.json",
        extension: Some("json"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::json::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::json::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::json::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::json::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.json"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.json`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::json::schema::json_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.json` artifact engine.
pub struct JsonEngine {
    artifact_state: JsonArtifact,
    snapshot_state: JsonSnapshot,
}

impl JsonEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: JsonSnapshot) -> Self {
        let artifact_state = JsonArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_json_snapshot();
        assert_eq!(snapshot.schema, STDIO_JSON_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_json_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <JsonSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    #[test]
    fn nontrivial_nested_value_round_trip() {
        use crate::artifacts::json::schema::snapshot::{JsonMember, JsonValue};
        let value = JsonValue::Object(vec![
            JsonMember { key: "name".into(), value: JsonValue::String("semio".into()) },
            JsonMember { key: "count".into(), value: JsonValue::Number { lexeme: "42".into() } },
            JsonMember { key: "ratio".into(), value: JsonValue::Number { lexeme: "3.5".into() } },
            JsonMember { key: "active".into(), value: JsonValue::Bool(true) },
            JsonMember { key: "missing".into(), value: JsonValue::Null },
            JsonMember { key: "tags".into(), value: JsonValue::Array(vec![JsonValue::String("a".into()), JsonValue::String("b".into()), JsonValue::String("c".into())]) },
            JsonMember {
                key: "nested".into(),
                value: JsonValue::Object(vec![JsonMember {
                    key: "deep".into(),
                    value: JsonValue::Object(vec![JsonMember {
                        key: "deeper".into(),
                        value: JsonValue::Array(vec![
                            JsonValue::Number { lexeme: "1".into() },
                            JsonValue::Number { lexeme: "2".into() },
                            JsonValue::Number { lexeme: "3".into() },
                        ]),
                    }]),
                }]),
            },
        ]);
        let snap = JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value };
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.value, snap.value);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <JsonSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded.value, snap.value);
    }
}
//#endregion 🧪️Tests
