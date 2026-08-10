//! 🏗️ PptxBuilder — local ArtifactBuilder until SDK Wave 3.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::pptx::schema::snapshot::{PptxParagraph, PptxRun, PptxSlide};
use crate::artifacts::pptx::{PptxDiff, PptxMutation, PptxSnapshot};

//#region 🔖️Builder
/// 🏗️ Builds a `stdio.pptx` snapshot.
#[derive(Clone, Debug, Default)]
pub struct PptxBuilder {
    snapshot: PptxSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for PptxBuilder {
    type Snapshot = PptxSnapshot;
    type Mutation = PptxMutation;
    type Diff = PptxDiff;
    fn empty() -> Self {
        Self { snapshot: PptxSnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<PptxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<PptxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::pptx::schema::mutations::apply_pptx_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <PptxDiff as protocol::MutationDiff<PptxSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder

//#region 🔖️TypedConstructors
/// 🧱️ Typed content constructors — build a presentation from slides of paragraphs/runs with
/// basic formatting (bold/italic), the same shape as `docx::DocxBuilder`'s constructors.
impl PptxBuilder {
    /// ➕️ Appends a new (initially empty) slide and makes it the active slide for `add_paragraph`.
    pub fn add_slide(mut self) -> Self {
        self.snapshot.presentation.slides.push(PptxSlide::default());
        self.rebuild()
    }

    /// ➕️ Appends a paragraph to the active slide (the most recently added one).
    pub fn add_paragraph(mut self, paragraph: PptxParagraph) -> Self {
        if let Some(slide) = self.snapshot.presentation.slides.last_mut() {
            slide.paragraphs.push(paragraph);
        }
        self.rebuild()
    }

    /// ➕️ Appends a single-run plain-text paragraph to the active slide.
    pub fn add_text_paragraph(self, text: impl Into<String>) -> Self {
        self.add_paragraph(PptxParagraph::text(text.into()))
    }

    /// ➕️ Appends a paragraph made of the given runs (basic bold/italic formatting).
    pub fn add_runs(self, runs: Vec<PptxRun>) -> Self {
        self.add_paragraph(PptxParagraph { runs })
    }

    fn rebuild(mut self) -> Self {
        self.snapshot = crate::artifacts::pptx::engine::build_minimal_pptx(self.snapshot.presentation);
        self
    }
}
//#endregion 🔖️TypedConstructors
