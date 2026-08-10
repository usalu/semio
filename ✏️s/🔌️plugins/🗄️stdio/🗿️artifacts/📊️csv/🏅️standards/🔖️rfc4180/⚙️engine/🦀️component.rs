//! ⚙️ CsvEngine — owns a real `CsvArtifact`.

use crate::artifacts::csv::{CsvArtifact, CsvDiff, CsvMutation, CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_csv_snapshot() -> CsvSnapshot {
    CsvSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::csv::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::DocumentCodec::of::<CsvSnapshot, CsvMutation>(STDIO_CSV_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.csv",
        extension: Some("csv"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::csv::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::csv::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::csv::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::csv::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.csv"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.csv`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::csv::schema::csv_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.csv` artifact engine.
pub struct CsvEngine {
    artifact_state: CsvArtifact,
    snapshot_state: CsvSnapshot,
}

impl CsvEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: CsvSnapshot) -> Self {
        let artifact_state = CsvArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}

impl protocol::ArtifactEngine for CsvEngine {
    type Artifact = CsvArtifact;
    type Snapshot = CsvSnapshot;
    type Mutation = CsvMutation;
    type Diff = CsvDiff;

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
        let snapshot = empty_csv_snapshot();
        assert_eq!(snapshot.schema, STDIO_CSV_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_csv_snapshot();
        let text = store::DocumentDsl::print_dsl(&snap);
        let parsed = <CsvSnapshot as store::DocumentDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::DocumentPack::encode_pack(&snap);
        let decoded = <CsvSnapshot as store::DocumentPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
}
//#endregion 🧪️Tests
