//! 🏗️ StepBuilder (final, artifact-level) — delegates to the ap214 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::step::{StepDiff, StepMutation, StepSnapshot};
use crate::artifacts::step::standards::v_ap214::builder::StepBuilder as StepRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct StepBuilder(StepRawBuilder);

impl ArtifactBuilder for StepBuilder {
    type Snapshot = StepSnapshot;
    type Mutation = StepMutation;
    type Diff = StepDiff;
    fn empty() -> Self { Self(StepRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(StepRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(StepRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(StepRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
