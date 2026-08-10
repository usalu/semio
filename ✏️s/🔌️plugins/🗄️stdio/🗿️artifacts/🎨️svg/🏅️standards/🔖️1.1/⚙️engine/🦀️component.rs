//! ⚙️ SvgEngine — owns a real `SvgArtifact`.

use crate::artifacts::svg::{SvgArtifact, SvgDiff, SvgMutation, SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_svg_snapshot() -> SvgSnapshot {
    SvgSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::svg::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<SvgSnapshot, SvgMutation>(STDIO_SVG_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.svg",
        extension: Some("svg"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::svg::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::svg::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::svg::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::svg::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.svg"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.svg`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::svg::schema::svg_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.svg` artifact engine.
pub struct SvgEngine {
    artifact_state: SvgArtifact,
    snapshot_state: SvgSnapshot,
}

impl SvgEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: SvgSnapshot) -> Self {
        let artifact_state = SvgArtifact::from_snapshot(snapshot.clone());
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
        let snapshot = empty_svg_snapshot();
        assert_eq!(snapshot.schema, STDIO_SVG_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_svg_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <SvgSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <SvgSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
}
//#endregion 🧪️Tests
