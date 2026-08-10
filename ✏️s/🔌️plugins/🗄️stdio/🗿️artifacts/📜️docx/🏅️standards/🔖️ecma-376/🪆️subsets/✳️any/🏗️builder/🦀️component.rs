//! 🏗️ DocxBuilder — local ArtifactBuilder until SDK Wave 3.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::docx::schema::snapshot::{DocxParagraph, DocxRun};
use crate::artifacts::docx::{DocxDiff, DocxMutation, DocxSnapshot};

//#region 🔖️Builder
/// 🏗️ Builds a `stdio.docx` snapshot.
#[derive(Clone, Debug, Default)]
pub struct DocxBuilder {
    snapshot: DocxSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for DocxBuilder {
    type Snapshot = DocxSnapshot;
    type Mutation = DocxMutation;
    type Diff = DocxDiff;
    fn empty() -> Self {
        Self { snapshot: DocxSnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<DocxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<DocxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = crate::artifacts::docx::schema::mutations::apply_docx_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <DocxDiff as protocol::MutationDiff<DocxSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder

//#region 🔖️TypedConstructors
/// 🧱️ Typed content constructors — build a `word/document.xml` document from paragraphs/runs
/// with basic formatting (bold/italic), mirroring the svg artifact's "builder builds a full
/// standard document" reference shape. Chainable; `build()` (from `ArtifactBuilder`) produces
/// the final `DocxSnapshot`, whose OPC container is assembled fresh (see `engine::build_minimal_docx`)
/// the first time a paragraph is added to an otherwise-empty builder.
impl DocxBuilder {
    /// ➕️ Appends a paragraph.
    pub fn add_paragraph(mut self, paragraph: DocxParagraph) -> Self {
        self.snapshot.document.paragraphs.push(paragraph);
        self.snapshot = crate::artifacts::docx::engine::build_minimal_docx(self.snapshot.document);
        self
    }

    /// ➕️ Appends a single-run plain-text paragraph.
    pub fn add_text_paragraph(self, text: impl Into<String>) -> Self {
        self.add_paragraph(DocxParagraph::text(text.into()))
    }

    /// ➕️ Appends a paragraph made of the given runs (basic bold/italic formatting).
    pub fn add_runs(self, runs: Vec<DocxRun>) -> Self {
        self.add_paragraph(DocxParagraph { runs })
    }
}
//#endregion 🔖️TypedConstructors
