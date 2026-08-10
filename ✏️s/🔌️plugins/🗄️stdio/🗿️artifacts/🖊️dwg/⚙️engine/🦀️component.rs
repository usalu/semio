//! ⚙️ DwgEngine — owns a real `DwgArtifact`.

use crate::artifacts::dwg::{DwgArtifact, DwgDiff, DwgMutation, DwgSnapshot, STDIO_DWG_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_dwg_snapshot() -> DwgSnapshot {
    DwgSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::dwg::io::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::DocumentCodec::of::<DwgSnapshot, DwgMutation>(STDIO_DWG_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (dwg).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.dwg",
        extension: Some("bin"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::dwg::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::dwg::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::dwg::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::dwg::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.dwg"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.dwg`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::dwg::schema::dwg_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.dwg` artifact engine.
pub struct DwgEngine {
    artifact_state: DwgArtifact,
    snapshot_state: DwgSnapshot,
}

impl DwgEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: DwgSnapshot) -> Self {
        let artifact_state = DwgArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}

impl protocol::ArtifactEngine for DwgEngine {
    type Artifact = DwgArtifact;
    type Snapshot = DwgSnapshot;
    type Mutation = DwgMutation;
    type Diff = DwgDiff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact_state
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot_state
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot_state);
        self.snapshot_state = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot_state);
        self.artifact_state.set_snapshot(self.snapshot_state.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot_state)
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_dwg_snapshot();
        assert_eq!(snapshot.schema, STDIO_DWG_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let stub = b"AC1018\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let snap = crate::artifacts::dwg::schema::snapshot::decode_dwg(stub).expect("decode stub");
        let text = store::DocumentDsl::print_dsl(&snap);
        let parsed = <DwgSnapshot as store::DocumentDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.version, "AC1018");
        assert_eq!(parsed.bytes, stub);
        let bytes = store::DocumentPack::encode_pack(&snap);
        let decoded = <DwgSnapshot as store::DocumentPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
}
//#endregion 🧪️Tests
