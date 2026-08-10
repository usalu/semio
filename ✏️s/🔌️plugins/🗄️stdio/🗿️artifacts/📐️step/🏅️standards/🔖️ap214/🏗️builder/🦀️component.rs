//! 🏗️ StepBuilder (ap214 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::step::{StepDiff, StepMutation, StepSnapshot};
use crate::artifacts::step::standards::v_ap214::subsets::any::builder::StepBuilder as StepRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct StepBuilder(StepRawAnyBuilder);

impl ArtifactBuilder for StepBuilder {
    type Snapshot = StepSnapshot;
    type Mutation = StepMutation;
    type Diff = StepDiff;
    fn empty() -> Self { Self(StepRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(StepRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(StepRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(StepRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
