//! 🏗️ DagBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::dag::{DagDiff, DagMutation, DagSnapshot};
use crate::artifacts::dag::standards::v1::subsets::any::builder::DagBuilder as DagAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct DagBuilder(DagAnyBuilder);

impl ArtifactBuilder for DagBuilder {
    type Snapshot = DagSnapshot;
    type Mutation = DagMutation;
    type Diff = DagDiff;
    fn empty() -> Self { Self(DagAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(DagAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(DagAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(DagAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
