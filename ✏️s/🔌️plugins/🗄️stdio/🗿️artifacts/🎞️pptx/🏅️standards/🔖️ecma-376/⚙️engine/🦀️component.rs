//! ⚙️ Office OOXML engine — ZIP container of parts (shared).

use crate::artifacts::pptx::{schema::snapshot::PptxEntry, PptxArtifact, PptxDiff, PptxMutation, PptxSnapshot, STDIO_PPTX_DOCUMENT_SCHEMA};

fn to_zip(snap: &PptxSnapshot) -> crate::artifacts::zip::ZipSnapshot {
    crate::artifacts::zip::ZipSnapshot {
        schema: crate::artifacts::zip::STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        entries: snap.entries.iter().map(|e| crate::artifacts::zip::schema::snapshot::ZipEntry {
            name: e.name.clone(),
            data: e.data.clone(),
        }).collect(),
    }
}

fn from_zip(z: crate::artifacts::zip::ZipSnapshot) -> PptxSnapshot {
    PptxSnapshot {
        schema: STDIO_PPTX_DOCUMENT_SCHEMA.into(),
        entries: z.entries.into_iter().map(|e| PptxEntry { name: e.name, data: e.data }).collect(),
    }
}

pub fn encode_pptx(snap: &PptxSnapshot) -> Result<Vec<u8>, String> {
    crate::artifacts::zip::engine::encode_zip(&to_zip(snap), true)
}

pub fn decode_pptx(data: &[u8]) -> Result<PptxSnapshot, String> {
  Ok(from_zip(crate::artifacts::zip::engine::decode_zip(data)?))
}

pub fn empty_pptx_snapshot() -> PptxSnapshot { PptxSnapshot::default() }

pub fn register() {
    crate::artifacts::pptx::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::pptx::schema::pptx_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<PptxSnapshot, PptxMutation>(STDIO_PPTX_DOCUMENT_SCHEMA));
}

pub struct PptxEngine { artifact_state: PptxArtifact, snapshot_state: PptxSnapshot }
impl PptxEngine {
    pub fn new(snapshot: PptxSnapshot) -> Self {
        Self { artifact_state: PptxArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
impl protocol::ArtifactEngine for PptxEngine {
    type Artifact = PptxArtifact; type Snapshot = PptxSnapshot; type Mutation = PptxMutation; type Diff = PptxDiff;
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
