//! 🏗️ Puzzle2dBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::puzzle2d::{Puzzle2dDiff, Puzzle2dMutation, Puzzle2dSnapshot};
use crate::artifacts::puzzle2d::standards::v1::subsets::any::builder::Puzzle2dBuilder as Puzzle2dAnyBuilder;

#[derive(Clone, Debug)]
pub struct Puzzle2dBuilder(Puzzle2dAnyBuilder);

impl ArtifactBuilder for Puzzle2dBuilder {
    type Snapshot = Puzzle2dSnapshot;
    type Mutation = Puzzle2dMutation;
    type Diff = Puzzle2dDiff;
    fn empty() -> Self { Self(Puzzle2dAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Puzzle2dAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Puzzle2dAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Puzzle2dAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
