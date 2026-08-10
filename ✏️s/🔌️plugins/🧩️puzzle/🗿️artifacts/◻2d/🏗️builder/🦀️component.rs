//! 🏗️ Puzzle2dBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::puzzle2d::{Puzzle2dDiff, Puzzle2dMutation, Puzzle2dSnapshot};
use crate::artifacts::puzzle2d::standards::v1::builder::Puzzle2dBuilder as Puzzle2dRawBuilder;

#[derive(Clone, Debug)]
pub struct Puzzle2dBuilder(Puzzle2dRawBuilder);

impl ArtifactBuilder for Puzzle2dBuilder {
    type Snapshot = Puzzle2dSnapshot;
    type Mutation = Puzzle2dMutation;
    type Diff = Puzzle2dDiff;
    fn empty() -> Self { Self(Puzzle2dRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Puzzle2dRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Puzzle2dRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Puzzle2dRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
