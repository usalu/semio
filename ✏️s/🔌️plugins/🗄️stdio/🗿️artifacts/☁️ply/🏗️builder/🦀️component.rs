//! 🏗️ PlyBuilder (final, artifact-level) — delegates to the 1.0 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::ply::{PlyDiff, PlyMutation, PlySnapshot};
use crate::artifacts::ply::standards::v1_0::builder::PlyBuilder as PlyRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct PlyBuilder(PlyRawBuilder);

impl ArtifactBuilder for PlyBuilder {
    type Snapshot = PlySnapshot;
    type Mutation = PlyMutation;
    type Diff = PlyDiff;
    fn empty() -> Self { Self(PlyRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(PlyRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(PlyRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(PlyRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
