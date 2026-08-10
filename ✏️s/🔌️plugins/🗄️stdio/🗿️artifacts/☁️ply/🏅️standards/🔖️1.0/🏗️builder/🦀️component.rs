//! 🏗️ PlyBuilder (1.0 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::ply::{PlyDiff, PlyMutation, PlySnapshot};
use crate::artifacts::ply::standards::v1_0::subsets::any::builder::PlyBuilder as PlyRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct PlyBuilder(PlyRawAnyBuilder);

impl ArtifactBuilder for PlyBuilder {
    type Snapshot = PlySnapshot;
    type Mutation = PlyMutation;
    type Diff = PlyDiff;
    fn empty() -> Self { Self(PlyRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(PlyRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(PlyRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(PlyRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
