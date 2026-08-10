//! ⚙️ BinaryEngine — owns a real `BinaryArtifact`.

use crate::artifacts::binary::{BinaryArtifact, BinaryDiff, BinaryMutation, BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_binary_snapshot() -> BinarySnapshot {
    BinarySnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::binary::io::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::DocumentCodec::of::<BinarySnapshot, BinaryMutation>(STDIO_BINARY_DOCUMENT_SCHEMA));
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

impl protocol::ArtifactEngine for BinaryEngine {
    type Artifact = BinaryArtifact;
    type Snapshot = BinarySnapshot;
    type Mutation = BinaryMutation;
    type Diff = BinaryDiff;

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
        let snapshot = empty_binary_snapshot();
        assert_eq!(snapshot.schema, STDIO_BINARY_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_binary_snapshot();
        let text = store::DocumentDsl::print_dsl(&snap);
        let parsed = <BinarySnapshot as store::DocumentDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::DocumentPack::encode_pack(&snap);
        let decoded = <BinarySnapshot as store::DocumentPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
}
//#endregion 🧪️Tests
