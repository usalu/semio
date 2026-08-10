//! 🏗️ Puzzle3dBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::puzzle3d::{Puzzle3dDiff, Puzzle3dMutation, Puzzle3dSnapshot};
use crate::artifacts::puzzle3d::standards::v1::builder::Puzzle3dBuilder as Puzzle3dRawBuilder;

#[derive(Clone, Debug)]
pub struct Puzzle3dBuilder(Puzzle3dRawBuilder);

impl ArtifactBuilder for Puzzle3dBuilder {
    type Snapshot = Puzzle3dSnapshot;
    type Mutation = Puzzle3dMutation;
    type Diff = Puzzle3dDiff;
    fn empty() -> Self { Self(Puzzle3dRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Puzzle3dRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Puzzle3dRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Puzzle3dRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
