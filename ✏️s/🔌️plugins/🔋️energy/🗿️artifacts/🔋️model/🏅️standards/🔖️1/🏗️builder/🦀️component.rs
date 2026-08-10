//! 🏗️ EnergyModelBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::model::{EnergyModelDiff, EnergyModelMutation, EnergyModelSnapshot};
use crate::artifacts::model::standards::v1::subsets::any::builder::ModelBuilder as EnergyModelAnyBuilder;

#[derive(Clone, Debug)]
pub struct EnergyModelBuilder(EnergyModelAnyBuilder);

impl ArtifactBuilder for EnergyModelBuilder {
    type Snapshot = EnergyModelSnapshot;
    type Mutation = EnergyModelMutation;
    type Diff = EnergyModelDiff;
    fn empty() -> Self { Self(EnergyModelAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(EnergyModelAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(EnergyModelAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(EnergyModelAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
