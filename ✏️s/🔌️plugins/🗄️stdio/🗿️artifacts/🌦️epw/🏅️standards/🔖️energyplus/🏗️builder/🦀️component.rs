//! 🏗️ EpwBuilder (energyplus standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::diff::EpwDiff;
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::EpwMutation;
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
use crate::artifacts::epw::standards::energyplus::subsets::any::builder::EpwBuilder as EpwRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct EpwBuilder(EpwRawAnyBuilder);

impl ArtifactBuilder for EpwBuilder {
    type Snapshot = EpwSnapshot;
    type Mutation = EpwMutation;
    type Diff = EpwDiff;
    fn empty() -> Self { Self(EpwRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(EpwRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(EpwRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(EpwRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
