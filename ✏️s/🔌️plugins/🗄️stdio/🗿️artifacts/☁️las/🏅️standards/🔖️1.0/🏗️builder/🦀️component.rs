//! 🏗️ LasBuilder (1.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::las::{LasDiff, LasMutation, LasSnapshot};
use crate::artifacts::las::standards::v1_0::subsets::any::builder::LasBuilder as LasRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct LasBuilder(LasRawAnyBuilder);

impl ArtifactBuilder for LasBuilder {
    type Snapshot = LasSnapshot;
    type Mutation = LasMutation;
    type Diff = LasDiff;
    fn empty() -> Self { Self(LasRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(LasRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(LasRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(LasRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
