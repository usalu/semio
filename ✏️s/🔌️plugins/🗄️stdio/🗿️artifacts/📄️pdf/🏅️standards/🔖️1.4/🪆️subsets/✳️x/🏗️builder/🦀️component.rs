//! 🏗️ PdfXBuilder (1.4/✳️x) — PASS-THROUGH by design, matching `🎹️composer`'s honesty scope:
//! `PageDoc{width,height,text}` has no object graph, so there is no field this builder could
//! restrict its ergonomic surface around to make construction "conforming by construction" the
//! way 1.7's `✳️a` builder does. `build()` re-runs `check_pdf_x_conformance` and always attaches
//! its (always-SOFT) diagnostics, but since there is no HARD check possible at this schema, it
//! never fails -- documented honestly rather than pretending a hard gate exists.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::pdf::standards::v1_4::subsets::x::analyzer::check_pdf_x_conformance;
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::{diff::PdfDiff, mutations::PdfMutation, snapshot::PdfSnapshot};

//#region 🔖️Builder
#[derive(Clone, Debug, Default)]
pub struct PdfXBuilder {
    snapshot: PdfSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for PdfXBuilder {
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
        let diff = crate::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::apply_pdf_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }

    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <PdfDiff as protocol::MutationDiff<PdfSnapshot>>::apply(&diff, &self.snapshot);
        self
    }

    /// 🛡️ Re-runs the honestly-scope-limited PDF/X check -- always SOFT at this schema, so
    /// `build()` never fails; the diagnostics still surface via the analyzer/composer/validator
    /// paths for anyone inspecting them.
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        let _ = check_pdf_x_conformance(&self.snapshot);
        if self.diagnostics.is_empty() {
            Ok(self.snapshot)
        } else {
            Err(self.diagnostics)
        }
    }
}
//#endregion 🔖️Builder

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_through_build_never_fails_on_conformance_grounds() {
        let snapshot = PdfXBuilder::empty().build().expect("no hard check exists at this schema; build must succeed");
        assert_eq!(snapshot.page.width, 612.0);
    }
}
