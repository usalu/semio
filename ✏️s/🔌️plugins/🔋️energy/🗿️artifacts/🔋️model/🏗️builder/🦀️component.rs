//! 🏗️ EnergyModelBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::model::{EnergyModelDiff, EnergyModelMutation, EnergyModelSnapshot};
use crate::artifacts::model::standards::v1::builder::EnergyModelBuilder as EnergyModelRawBuilder;

#[derive(Clone, Debug)]
pub struct EnergyModelBuilder(EnergyModelRawBuilder);

impl ArtifactBuilder for EnergyModelBuilder {
    type Snapshot = EnergyModelSnapshot;
    type Mutation = EnergyModelMutation;
    type Diff = EnergyModelDiff;
    fn empty() -> Self { Self(EnergyModelRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(EnergyModelRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(EnergyModelRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(EnergyModelRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
