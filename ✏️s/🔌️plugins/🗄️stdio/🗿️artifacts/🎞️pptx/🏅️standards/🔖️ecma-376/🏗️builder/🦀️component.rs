//! 🏗️ PptxBuilder (ecma-376 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::pptx::{PptxDiff, PptxMutation, PptxSnapshot};
use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::builder::PptxBuilder as PptxRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct PptxBuilder(PptxRawAnyBuilder);

impl ArtifactBuilder for PptxBuilder {
    type Snapshot = PptxSnapshot;
    type Mutation = PptxMutation;
    type Diff = PptxDiff;
    fn empty() -> Self { Self(PptxRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(PptxRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(PptxRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(PptxRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
