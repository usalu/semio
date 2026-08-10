//! 🏗️ Block2dBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::block2d::{Block2dDiff, Block2dMutation, Block2dSnapshot};
use crate::artifacts::block2d::standards::v1::builder::Block2dBuilder as Block2dRawBuilder;

#[derive(Clone, Debug)]
pub struct Block2dBuilder(Block2dRawBuilder);

impl ArtifactBuilder for Block2dBuilder {
    type Snapshot = Block2dSnapshot;
    type Mutation = Block2dMutation;
    type Diff = Block2dDiff;
    fn empty() -> Self { Self(Block2dRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Block2dRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Block2dRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Block2dRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
