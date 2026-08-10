//! 🏗️ RemodelBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::remodel::{RemodelDiff, RemodelMutation, RemodelSnapshot};
use crate::artifacts::remodel::standards::v1::subsets::any::builder::RemodelBuilder as RemodelAnyBuilder;

#[derive(Clone, Debug)]
pub struct RemodelBuilder(RemodelAnyBuilder);

impl ArtifactBuilder for RemodelBuilder {
    type Snapshot = RemodelSnapshot;
    type Mutation = RemodelMutation;
    type Diff = RemodelDiff;
    fn empty() -> Self { Self(RemodelAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(RemodelAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(RemodelAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(RemodelAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
