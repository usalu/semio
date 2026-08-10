//! 🏗️ CurateBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::curate::{CurateDiff, SourcingMutation, CurateSnapshot};
use crate::artifacts::curate::standards::v1::builder::CurateBuilder as CurateRawBuilder;

#[derive(Clone, Debug)]
pub struct CurateBuilder(CurateRawBuilder);

impl ArtifactBuilder for CurateBuilder {
    type Snapshot = CurateSnapshot;
    type Mutation = SourcingMutation;
    type Diff = CurateDiff;
    fn empty() -> Self { Self(CurateRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(CurateRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(CurateRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(CurateRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
