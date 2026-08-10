//! 🏗️ Block2dBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::block2d::{Block2dDiff, Block2dMutation, Block2dSnapshot};
use crate::artifacts::block2d::standards::v1::subsets::any::builder::Block2dBuilder as Block2dAnyBuilder;

#[derive(Clone, Debug)]
pub struct Block2dBuilder(Block2dAnyBuilder);

impl ArtifactBuilder for Block2dBuilder {
    type Snapshot = Block2dSnapshot;
    type Mutation = Block2dMutation;
    type Diff = Block2dDiff;
    fn empty() -> Self { Self(Block2dAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Block2dAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Block2dAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Block2dAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
