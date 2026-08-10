//! 🏗️ DagBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::dag::{DagDiff, DagMutation, DagSnapshot};
use crate::artifacts::dag::standards::v1::builder::DagBuilder as DagRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct DagBuilder(DagRawBuilder);

impl ArtifactBuilder for DagBuilder {
    type Snapshot = DagSnapshot;
    type Mutation = DagMutation;
    type Diff = DagDiff;
    fn empty() -> Self { Self(DagRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(DagRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(DagRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(DagRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
