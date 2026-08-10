//! 🏗️ DxfBuilder (r12 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::dxf::{DxfDiff, DxfMutation, DxfSnapshot};
use crate::artifacts::dxf::standards::v_r12::subsets::any::builder::DxfBuilder as DxfRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct DxfBuilder(DxfRawAnyBuilder);

impl ArtifactBuilder for DxfBuilder {
    type Snapshot = DxfSnapshot;
    type Mutation = DxfMutation;
    type Diff = DxfDiff;
    fn empty() -> Self { Self(DxfRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(DxfRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(DxfRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(DxfRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
