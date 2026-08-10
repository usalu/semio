//! ⚙️ Office OOXML engine — ZIP container of parts (shared).

use crate::artifacts::xlsx::{schema::snapshot::XlsxEntry, XlsxArtifact, XlsxDiff, XlsxMutation, XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};

fn to_zip(snap: &XlsxSnapshot) -> crate::artifacts::zip::ZipSnapshot {
    crate::artifacts::zip::ZipSnapshot {
        schema: crate::artifacts::zip::STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        entries: snap.entries.iter().map(|e| crate::artifacts::zip::schema::snapshot::ZipEntry {
            name: e.name.clone(),
            data: e.data.clone(),
        }).collect(),
    }
}

fn from_zip(z: crate::artifacts::zip::ZipSnapshot) -> XlsxSnapshot {
    XlsxSnapshot {
        schema: STDIO_XLSX_DOCUMENT_SCHEMA.into(),
        entries: z.entries.into_iter().map(|e| XlsxEntry { name: e.name, data: e.data }).collect(),
    }
}

pub fn encode_xlsx(snap: &XlsxSnapshot) -> Result<Vec<u8>, String> {
    crate::artifacts::zip::engine::encode_zip(&to_zip(snap), true)
}

pub fn decode_xlsx(data: &[u8]) -> Result<XlsxSnapshot, String> {
  Ok(from_zip(crate::artifacts::zip::engine::decode_zip(data)?))
}

pub fn empty_xlsx_snapshot() -> XlsxSnapshot { XlsxSnapshot::default() }

pub fn register() {
    crate::artifacts::xlsx::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::xlsx::schema::xlsx_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<XlsxSnapshot, XlsxMutation>(STDIO_XLSX_DOCUMENT_SCHEMA));
}

pub struct XlsxEngine { artifact_state: XlsxArtifact, snapshot_state: XlsxSnapshot }
impl XlsxEngine {
    pub fn new(snapshot: XlsxSnapshot) -> Self {
        Self { artifact_state: XlsxArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
impl protocol::ArtifactEngine for XlsxEngine {
    type Artifact = XlsxArtifact; type Snapshot = XlsxSnapshot; type Mutation = XlsxMutation; type Diff = XlsxDiff;
    fn artifact(&self) -> &Self::Artifact { &self.artifact_state }
    fn snapshot(&self) -> &Self::Snapshot { &self.snapshot_state }
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
