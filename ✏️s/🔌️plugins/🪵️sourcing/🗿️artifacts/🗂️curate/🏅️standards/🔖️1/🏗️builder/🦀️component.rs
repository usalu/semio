//! 🏗️ CurateBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::curate::{CurateDiff, SourcingMutation, CurateSnapshot};
use crate::artifacts::curate::standards::v1::subsets::any::builder::CurateBuilder as CurateAnyBuilder;

#[derive(Clone, Debug)]
pub struct CurateBuilder(CurateAnyBuilder);

impl ArtifactBuilder for CurateBuilder {
    type Snapshot = CurateSnapshot;
    type Mutation = SourcingMutation;
    type Diff = CurateDiff;
    fn empty() -> Self { Self(CurateAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(CurateAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(CurateAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(CurateAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
