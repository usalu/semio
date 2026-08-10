//! ⚙️ Office OOXML engine — ZIP container of parts (shared).

use crate::artifacts::bcf::{schema::snapshot::BcfEntry, BcfArtifact, BcfDiff, BcfMutation, BcfSnapshot, STDIO_BCF_DOCUMENT_SCHEMA};

fn to_zip(snap: &BcfSnapshot) -> crate::artifacts::zip::ZipSnapshot {
    crate::artifacts::zip::ZipSnapshot {
        schema: crate::artifacts::zip::STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        entries: snap.entries.iter().map(|e| crate::artifacts::zip::schema::snapshot::ZipEntry {
            name: e.name.clone(),
            data: e.data.clone(),
        }).collect(),
    }
}

fn from_zip(z: crate::artifacts::zip::ZipSnapshot) -> BcfSnapshot {
    BcfSnapshot {
        schema: STDIO_BCF_DOCUMENT_SCHEMA.into(),
        entries: z.entries.into_iter().map(|e| BcfEntry { name: e.name, data: e.data }).collect(),
    }
}

pub fn encode_bcf(snap: &BcfSnapshot) -> Result<Vec<u8>, String> {
    crate::artifacts::zip::engine::encode_zip(&to_zip(snap), true)
}

pub fn decode_bcf(data: &[u8]) -> Result<BcfSnapshot, String> {
  Ok(from_zip(crate::artifacts::zip::engine::decode_zip(data)?))
}

pub fn empty_bcf_snapshot() -> BcfSnapshot { BcfSnapshot::default() }

pub fn register() {
    crate::artifacts::bcf::io::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::bcf::schema::bcf_artifact_schema_descriptor());
    store::register_document_codec(store::DocumentCodec::of::<BcfSnapshot, BcfMutation>(STDIO_BCF_DOCUMENT_SCHEMA));
}

pub struct BcfEngine { artifact_state: BcfArtifact, snapshot_state: BcfSnapshot }
impl BcfEngine {
    pub fn new(snapshot: BcfSnapshot) -> Self {
        Self { artifact_state: BcfArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
impl protocol::ArtifactEngine for BcfEngine {
    type Artifact = BcfArtifact; type Snapshot = BcfSnapshot; type Mutation = BcfMutation; type Diff = BcfDiff;
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
