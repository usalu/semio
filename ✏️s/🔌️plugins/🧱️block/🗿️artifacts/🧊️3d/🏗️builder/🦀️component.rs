//! 🏗️ Block3dBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::block3d::{Block3dDiff, Block3dMutation, Block3dSnapshot};
use crate::artifacts::block3d::standards::v1::builder::Block3dBuilder as Block3dRawBuilder;

#[derive(Clone, Debug)]
pub struct Block3dBuilder(Block3dRawBuilder);

impl ArtifactBuilder for Block3dBuilder {
    type Snapshot = Block3dSnapshot;
    type Mutation = Block3dMutation;
    type Diff = Block3dDiff;
    fn empty() -> Self { Self(Block3dRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Block3dRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Block3dRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Block3dRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
