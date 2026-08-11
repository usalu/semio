//! 🏗️ EpwBuilder (final, artifact-level) — delegates to the only standard, energyplus.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::epw::{EpwDiff, EpwMutation, EpwSnapshot};
use crate::artifacts::epw::standards::energyplus::builder::EpwBuilder as EpwRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct EpwBuilder(EpwRawBuilder);

impl ArtifactBuilder for EpwBuilder {
    type Snapshot = EpwSnapshot;
    type Mutation = EpwMutation;
    type Diff = EpwDiff;
    fn empty() -> Self { Self(EpwRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(EpwRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(EpwRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(EpwRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
