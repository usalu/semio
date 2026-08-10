//! ⚙️ Office OOXML engine — ZIP container of parts (shared).

use crate::artifacts::docx::{schema::snapshot::DocxEntry, DocxArtifact, DocxDiff, DocxMutation, DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};

fn to_zip(snap: &DocxSnapshot) -> crate::artifacts::zip::ZipSnapshot {
    crate::artifacts::zip::ZipSnapshot {
        schema: crate::artifacts::zip::STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        entries: snap.entries.iter().map(|e| crate::artifacts::zip::schema::snapshot::ZipEntry {
            name: e.name.clone(),
            data: e.data.clone(),
        }).collect(),
    }
}

fn from_zip(z: crate::artifacts::zip::ZipSnapshot) -> DocxSnapshot {
    DocxSnapshot {
        schema: STDIO_DOCX_DOCUMENT_SCHEMA.into(),
        entries: z.entries.into_iter().map(|e| DocxEntry { name: e.name, data: e.data }).collect(),
    }
}

pub fn encode_docx(snap: &DocxSnapshot) -> Result<Vec<u8>, String> {
    crate::artifacts::zip::engine::encode_zip(&to_zip(snap), true)
}

pub fn decode_docx(data: &[u8]) -> Result<DocxSnapshot, String> {
  Ok(from_zip(crate::artifacts::zip::engine::decode_zip(data)?))
}

pub fn empty_docx_snapshot() -> DocxSnapshot { DocxSnapshot::default() }

pub fn register() {
    crate::artifacts::docx::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::docx::schema::docx_artifact_schema_descriptor());
    store::register_document_codec(store::DocumentCodec::of::<DocxSnapshot, DocxMutation>(STDIO_DOCX_DOCUMENT_SCHEMA));
}

pub struct DocxEngine { artifact_state: DocxArtifact, snapshot_state: DocxSnapshot }
impl DocxEngine {
    pub fn new(snapshot: DocxSnapshot) -> Self {
        Self { artifact_state: DocxArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
impl protocol::ArtifactEngine for DocxEngine {
    type Artifact = DocxArtifact; type Snapshot = DocxSnapshot; type Mutation = DocxMutation; type Diff = DocxDiff;
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
