//! 🏗️ Puzzle5dBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::puzzle5d::{Puzzle5dDiff, Puzzle5dMutation, Puzzle5dSnapshot};
use crate::artifacts::puzzle5d::standards::v1::subsets::any::builder::Puzzle5dBuilder as Puzzle5dAnyBuilder;

#[derive(Clone, Debug)]
pub struct Puzzle5dBuilder(Puzzle5dAnyBuilder);

impl ArtifactBuilder for Puzzle5dBuilder {
    type Snapshot = Puzzle5dSnapshot;
    type Mutation = Puzzle5dMutation;
    type Diff = Puzzle5dDiff;
    fn empty() -> Self { Self(Puzzle5dAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Puzzle5dAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Puzzle5dAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Puzzle5dAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
