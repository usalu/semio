//! 🏗️ PdfHBuilder (1.7/✳️h) — a typed builder for PDF/H. Since `check_h_conformance` is
//! ALL-SOFT (PDF/H has no hard checks -- see `🧐️analyzer`'s module doc comment), `build()` here
//! can never fail: it re-runs `check_h_conformance` for symmetry with every other subset builder
//! in this repo, but the result is always `Ok` (there is no `Severity::Error`/`Fatal` variant this
//! check ever produces to filter into an `Err`). Ticket
//! 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.

use dsl::Diagnostic;
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::pdf::standards::v1_7::subsets::h::analyzer::check_h_conformance;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfInfo, PdfPage, PdfSnapshot};

//#region 🔖️Builder
#[derive(Clone, Debug)]
pub struct PdfHBuilder {
    snapshot: PdfSnapshot,
}

impl PdfHBuilder {
    pub fn new() -> Self {
        Self { snapshot: PdfSnapshot::default() }
    }

    pub fn add_page(mut self, page: PdfPage) -> Self {
        let index = self.snapshot.pages.len();
        apply_pdf_mutation(&mut self.snapshot, &PdfMutation::InsertPage { index, page });
        self
    }

    pub fn set_info(mut self, info: PdfInfo) -> Self {
        apply_pdf_mutation(&mut self.snapshot, &PdfMutation::SetInfo { info });
        self
    }
}

impl Default for PdfHBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtifactBuilder for PdfHBuilder {
    type Snapshot = PdfSnapshot;
    type Mutation = PdfMutation;
    type Diff = PdfDiff;

    fn empty() -> Self {
        Self::new()
    }

    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot }
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

    /// ✅ Always `Ok` -- `check_h_conformance` is ALL-SOFT, so the hard-filter below is never
    /// non-empty. Still runs the real check (not skipped) so a future hard check added here would
    /// correctly start gating without any other code change.
    fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
        let hard: Vec<Diagnostic> = check_h_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, dsl::Severity::Error | dsl::Severity::Fatal)).collect();
        if hard.is_empty() {
            Ok(self.snapshot)
        } else {
            Err(hard)
        }
    }
}
//#endregion 🔖️Builder

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_always_succeeds() {
        let snapshot = PdfHBuilder::new().add_page(PdfPage::new(200.0, 200.0)).build().expect("PDF/H build() never fails");
        assert_eq!(snapshot.pages.len(), 1);
    }

    #[test]
    fn set_info_clears_title_author_advisory() {
        let snapshot = PdfHBuilder::new().set_info(PdfInfo { title: Some("A Chart".into()), author: Some("Dr. X".into()), ..PdfInfo::default() }).build().unwrap();
        let diagnostics = check_h_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.code.0 != crate::artifacts::pdf::standards::v1_7::subsets::h::analyzer::CODE_INFO_TITLE_OR_AUTHOR), "got {diagnostics:?}");
    }
}
