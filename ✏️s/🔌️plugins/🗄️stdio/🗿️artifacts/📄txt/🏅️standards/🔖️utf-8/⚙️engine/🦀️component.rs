//! ⚙️ TxtEngine — owns a real `TxtArtifact`.

use crate::artifacts::txt::{TxtArtifact, TxtDiff, TxtMutation, TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_txt_snapshot() -> TxtSnapshot {
    TxtSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::txt::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<TxtSnapshot, TxtMutation>(STDIO_TXT_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.txt",
        extension: Some("txt"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::txt::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::txt::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::txt::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::txt::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.txt"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.txt`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::txt::schema::txt_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.txt` artifact engine.
pub struct TxtEngine {
    artifact_state: TxtArtifact,
    snapshot_state: TxtSnapshot,
}

impl TxtEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: TxtSnapshot) -> Self {
        let artifact_state = TxtArtifact::from_snapshot(snapshot.clone());
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
        let snapshot = empty_txt_snapshot();
        assert_eq!(snapshot.schema, STDIO_TXT_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_txt_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <TxtSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <TxtSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    #[test]
    fn nontrivial_multiline_unicode_round_trip() {
        let body = "Hello, \u{4e16}\u{754c}!\nLine two with an emoji \u{1f389}.\nTab\there.\n".to_string();
        let snap = TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text: body.clone() };
        let dsl_text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <TxtSnapshot as store::ArtifactDsl>::parse_dsl(&dsl_text).expect("parse");
        assert_eq!(parsed.text, body);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <TxtSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded.text, body);
    }
}
//#endregion 🧪️Tests
