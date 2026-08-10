//! 🏗️ RemodelBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::remodel::{RemodelDiff, RemodelMutation, RemodelSnapshot};
use crate::artifacts::remodel::standards::v1::builder::RemodelBuilder as RemodelRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct RemodelBuilder(RemodelRawBuilder);

impl ArtifactBuilder for RemodelBuilder {
    type Snapshot = RemodelSnapshot;
    type Mutation = RemodelMutation;
    type Diff = RemodelDiff;
    fn empty() -> Self { Self(RemodelRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(RemodelRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(RemodelRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(RemodelRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
