//! ⚙️ BinaryEngine — owns a real `BinaryArtifact`.

use crate::artifacts::binary::{BinaryArtifact, BinaryDiff, BinaryMutation, BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_binary_snapshot() -> BinarySnapshot {
    BinarySnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs, the artifact schema descriptor, and every composer entry (which supersedes
/// the pre-migration per-leaf `io::register()` no-ops -- see `🎹️composer::register`).
pub fn register() {
    crate::artifacts::binary::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<BinarySnapshot, BinaryMutation>(STDIO_BINARY_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.binary",
        extension: Some("bin"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::binary::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::binary::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::binary::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::binary::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.binary"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.binary`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::binary::schema::binary_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.binary` artifact engine.
pub struct BinaryEngine {
    artifact_state: BinaryArtifact,
    snapshot_state: BinarySnapshot,
}

impl BinaryEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: BinarySnapshot) -> Self {
        let artifact_state = BinaryArtifact::from_snapshot(snapshot.clone());
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
        let snapshot = empty_binary_snapshot();
        assert_eq!(snapshot.schema, STDIO_BINARY_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_binary_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <BinarySnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <BinarySnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
}
//#endregion 🧪️Tests
