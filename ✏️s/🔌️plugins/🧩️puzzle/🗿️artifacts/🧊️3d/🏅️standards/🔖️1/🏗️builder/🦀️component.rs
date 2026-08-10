//! 🏗️ Puzzle3dBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::puzzle3d::{Puzzle3dDiff, Puzzle3dMutation, Puzzle3dSnapshot};
use crate::artifacts::puzzle3d::standards::v1::subsets::any::builder::Puzzle3dBuilder as Puzzle3dAnyBuilder;

#[derive(Clone, Debug)]
pub struct Puzzle3dBuilder(Puzzle3dAnyBuilder);

impl ArtifactBuilder for Puzzle3dBuilder {
    type Snapshot = Puzzle3dSnapshot;
    type Mutation = Puzzle3dMutation;
    type Diff = Puzzle3dDiff;
    fn empty() -> Self { Self(Puzzle3dAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Puzzle3dAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Puzzle3dAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Puzzle3dAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
