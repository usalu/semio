//! 🏗️ PdfBuilder (1.7/✳️any) — typed builder over `PdfMutation` ops (requirement #8).

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{PdfMutation, apply_pdf_mutation};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfInfo, PdfPage, PdfSnapshot};

//#region 🔖️Builder
/// 🏗️ Builds a `stdio.pdf.1.7` snapshot.
#[derive(Clone, Debug, Default)]
pub struct PdfBuilder {
    snapshot: PdfSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl PdfBuilder {
    /// ➕ Typed construction: appends a page (the analyzer→builder round-trip acceptance test's
    /// primary entry point -- requirement #8's `InsertPage`, exposed ergonomically).
    pub fn add_page(self, page: PdfPage) -> Self {
        let index = self.snapshot.pages.len();
        let (next, _diff) = self.mutate(PdfMutation::InsertPage { index, page });
        next
    }
    pub fn set_info(self, info: PdfInfo) -> Self {
        let (next, _diff) = self.mutate(PdfMutation::SetInfo { info });
        next
    }
}

impl ArtifactBuilder for PdfBuilder {
    type Snapshot = PdfSnapshot;
    type Mutation = PdfMutation;
    type Diff = PdfDiff;
    fn empty() -> Self {
        Self { snapshot: PdfSnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = apply_pdf_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <PdfDiff as protocol::MutationDiff<PdfSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder
