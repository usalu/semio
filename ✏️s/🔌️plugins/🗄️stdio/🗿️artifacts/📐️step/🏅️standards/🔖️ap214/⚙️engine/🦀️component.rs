//! ⚙️ StepEngine — owns a real `StepArtifact`.

use crate::artifacts::step::{StepArtifact, StepMutation, StepSnapshot, STDIO_STEP_DOCUMENT_SCHEMA};

//#region 🔖️Submodules
/// 📐 Shared ISO 10303-21 tokenizer + generic graph — public, importable cross-artifact (ifc reuses it).
#[path = "📐️part21/🦀️component.rs"]
pub mod part21;
/// 🧱 BrepMesh analyzer view, derived from the generic graph — never persisted itself.
#[path = "🧱️brep/🦀️component.rs"]
pub mod brep;
/// 🪜 Shared CC ladder classification + FILE_SCHEMA/PRODUCT-chain scans, reused by all six
/// `✳️ccN` subset analyzers (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES).
#[path = "🪜️ladder/🦀️component.rs"]
pub mod ladder;
//#endregion 🔖️Submodules

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_step_snapshot() -> StepSnapshot {
    StepSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::step::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    register_subset_validators();
    store::register_document_codec(store::ArtifactCodec::of::<StepSnapshot, StepMutation>(STDIO_STEP_DOCUMENT_SCHEMA));
}

/// 📌️ Registers the `SubsetValidator` of every real (non-`✳️any`) ap214 subset — the six ISO
/// 10303-214 conformance classes (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES).
pub fn register_subset_validators() {
    crate::artifacts::step::standards::v_ap214::subsets::cc1::composer::register();
    crate::artifacts::step::standards::v_ap214::subsets::cc2::composer::register();
    crate::artifacts::step::standards::v_ap214::subsets::cc3::composer::register();
    crate::artifacts::step::standards::v_ap214::subsets::cc4::composer::register();
    crate::artifacts::step::standards::v_ap214::subsets::cc5::composer::register();
    crate::artifacts::step::standards::v_ap214::subsets::cc6::composer::register();
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.step",
        extension: Some("step"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::step::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::step::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::step::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::step::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.step"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.step`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::step::schema::step_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.step` artifact engine.
pub struct StepEngine {
    artifact_state: StepArtifact,
    snapshot_state: StepSnapshot,
}

impl StepEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: StepSnapshot) -> Self {
        let artifact_state = StepArtifact::from_snapshot(snapshot.clone());
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
        let snapshot = empty_step_snapshot();
        assert_eq!(snapshot.schema, STDIO_STEP_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_step_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <StepSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
}
//#endregion 🧪️Tests
