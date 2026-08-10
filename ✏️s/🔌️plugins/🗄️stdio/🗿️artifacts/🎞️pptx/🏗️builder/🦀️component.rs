//! 🏗️ PptxBuilder (final, artifact-level) — delegates to the ecma-376 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::pptx::schema::snapshot::{PptxParagraph, PptxRun};
use crate::artifacts::pptx::{PptxDiff, PptxMutation, PptxSnapshot};
use crate::artifacts::pptx::standards::v_ecma_376::builder::PptxBuilder as PptxRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct PptxBuilder(PptxRawBuilder);

impl ArtifactBuilder for PptxBuilder {
    type Snapshot = PptxSnapshot;
    type Mutation = PptxMutation;
    type Diff = PptxDiff;
    fn empty() -> Self { Self(PptxRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(PptxRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(PptxRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(PptxRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}

/// 🧱️ Typed content constructors, forwarded to the ecma-376 standard builder.
impl PptxBuilder {
    pub fn add_slide(self) -> Self { Self(self.0.add_slide()) }
    pub fn add_paragraph(self, paragraph: PptxParagraph) -> Self { Self(self.0.add_paragraph(paragraph)) }
    pub fn add_text_paragraph(self, text: impl Into<String>) -> Self { Self(self.0.add_text_paragraph(text)) }
    pub fn add_runs(self, runs: Vec<PptxRun>) -> Self { Self(self.0.add_runs(runs)) }
}
