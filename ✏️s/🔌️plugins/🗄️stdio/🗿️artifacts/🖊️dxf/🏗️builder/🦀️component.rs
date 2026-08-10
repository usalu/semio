//! 🏗️ DxfBuilder (final, artifact-level) — delegates to the r12 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::dxf::{DxfDiff, DxfMutation, DxfSnapshot};
use crate::artifacts::dxf::standards::v_r12::builder::DxfBuilder as DxfRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct DxfBuilder(DxfRawBuilder);

impl ArtifactBuilder for DxfBuilder {
    type Snapshot = DxfSnapshot;
    type Mutation = DxfMutation;
    type Diff = DxfDiff;
    fn empty() -> Self { Self(DxfRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(DxfRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(DxfRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(DxfRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
