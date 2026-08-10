//! 🏗️ DocxBuilder (ecma-376 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::docx::schema::snapshot::{DocxParagraph, DocxRun};
use crate::artifacts::docx::{DocxDiff, DocxMutation, DocxSnapshot};
use crate::artifacts::docx::standards::v_ecma_376::subsets::any::builder::DocxBuilder as DocxRawAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct DocxBuilder(DocxRawAnyBuilder);

impl ArtifactBuilder for DocxBuilder {
    type Snapshot = DocxSnapshot;
    type Mutation = DocxMutation;
    type Diff = DocxDiff;
    fn empty() -> Self { Self(DocxRawAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(DocxRawAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(DocxRawAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(DocxRawAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}

impl DocxBuilder {
    pub fn add_paragraph(self, paragraph: DocxParagraph) -> Self { Self(self.0.add_paragraph(paragraph)) }
    pub fn add_text_paragraph(self, text: impl Into<String>) -> Self { Self(self.0.add_text_paragraph(text)) }
    pub fn add_runs(self, runs: Vec<DocxRun>) -> Self { Self(self.0.add_runs(runs)) }
}
