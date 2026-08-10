//! 🏗️ Block5dBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::block5d::{Block5dDiff, Block5dMutation, Block5dSnapshot};
use crate::artifacts::block5d::standards::v1::builder::Block5dBuilder as Block5dRawBuilder;

#[derive(Clone, Debug)]
pub struct Block5dBuilder(Block5dRawBuilder);

impl ArtifactBuilder for Block5dBuilder {
    type Snapshot = Block5dSnapshot;
    type Mutation = Block5dMutation;
    type Diff = Block5dDiff;
    fn empty() -> Self { Self(Block5dRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Block5dRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Block5dRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Block5dRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
