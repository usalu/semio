//! ⚙️ DxfEngine — owns a real `DxfArtifact`.

use crate::artifacts::dxf::{DxfArtifact, DxfDiff, DxfMutation, DxfSnapshot, STDIO_DXF_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_dxf_snapshot() -> DxfSnapshot {
    DxfSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::dxf::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<DxfSnapshot, DxfMutation>(STDIO_DXF_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.dxf",
        extension: Some("dxf"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::dxf::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::dxf::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::dxf::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::dxf::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.dxf"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.dxf`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::dxf::schema::dxf_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.dxf` artifact engine.
pub struct DxfEngine {
    artifact_state: DxfArtifact,
    snapshot_state: DxfSnapshot,
}

impl DxfEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: DxfSnapshot) -> Self {
        let artifact_state = DxfArtifact::from_snapshot(snapshot.clone());
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
        let snapshot = empty_dxf_snapshot();
        assert_eq!(snapshot.schema, STDIO_DXF_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_dxf_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <DxfSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <DxfSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
}
//#endregion 🧪️Tests
