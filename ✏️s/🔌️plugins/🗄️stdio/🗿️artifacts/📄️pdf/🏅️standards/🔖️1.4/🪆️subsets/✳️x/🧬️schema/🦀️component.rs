//! 🧬️ PdfSnapshot schema (1.4/✳️x) — reuses the ✳️any subset's `PdfSnapshot` verbatim (the SAME
//! Rust type, same `s.stdio.pdf` schema id). A subset is a validation-gated dialect STAMP on top
//! of that existing schema, not a new one -- see D4's Tier-1 "same snapshot type, subset moves"
//! semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so `🪆️subsets/✳️x/🧬️schema/`
//! is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without duplicating the schema definition.
//!
//! Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES: PDF 1.4's `PdfSnapshot` is a bare
//! `PageDoc{width,height,text}` -- no retained object graph -- so `🧐️analyzer` here implements
//! only what's honestly checkable from those fields, plus a SOFT schema-gap diagnostic. See
//! `🧐️analyzer` for the full honesty accounting.

pub use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::*;
//#region 🧬️Mutations
// 🧬️ This subset's OWN conformance vocabulary, mounted here rather than in the crate's shared
// `📦️glue.rs`: that file is one wiring file for every stdio artifact at once, and the rationale the
// ✳️any subset already records for its own test mount — leave the shared file alone, let an artifact
// own the subtree it owns — applies to a production leaf of this subset just as well. `#[path]` on a
// non-inline module resolves against this file's own directory. The explicit declaration shadows the
// glob re-export of ✳️any's `mutations` above, which is what puts this subset's own vocabulary at
// `subsets::<name>::schema::mutations` while ✳️any's document vocabulary stays reachable at its own
// address.
#[path = "🧬️mutations/🦀️component.rs"]
pub mod mutations;
//#endregion 🧬️Mutations

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::{diff::PdfDiff, mutations::PdfMutation, snapshot::PdfSnapshot};
    use crate::artifacts::pdf::standards::v1_4::subsets::x::schema::check_pdf_x_conformance;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct PdfXBuilderConstruction {
        snapshot: PdfSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for PdfXBuilderConstruction {
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

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::apply_pdf_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <PdfDiff as protocol::MutationDiff<PdfSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
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

        #[semio_framework_async_macros::async_test]
        async fn pass_through_build_never_fails_on_conformance_grounds() {
            let snapshot = PdfXBuilderConstruction::empty().build().expect("no hard check exists at this schema; build must succeed");
            assert_eq!(snapshot.pages.len(), 1, "an empty PDF 1.4 document is one blank page, never a document with no page tree");
            assert_eq!(snapshot.first_page().expect("page 1").width, 612.0);
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::{PdfAnalyzer as PdfAnyAnalyzer, PdfParts};
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("x") };

    //#region 🔖️Conformance
    pub const CODE_DEGENERATE_PAGE_SIZE: &str = "stdio.pdf.x.degenerate-page-size";
    pub const CODE_SCHEMA_GAP: &str = "stdio.pdf.x.schema-gap-unverifiable";

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Honestly-scope-limited PDF/X conformance check against one already-decoded `PdfSnapshot`.
    /// Shared single source of truth: `PdfXComposer::compose` (pass-through, can't hard-gate without
    /// an object graph) and the registered `SubsetValidator` both call this.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_pdf_x_conformance(snapshot: &PdfSnapshot) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        let page = snapshot.first_page().cloned().unwrap_or_default();
        if !(page.width > 0.0 && page.height > 0.0) {
            out.push(soft(
                CODE_DEGENERATE_PAGE_SIZE,
                format!("page 1's dimensions are degenerate ({}x{}) -- a print-ready PDF/X page needs a positive MediaBox; a weak signal, but a real one given PageDoc has no other checkable field", page.width, page.height),
            ));
        }
        out.push(soft(
            CODE_SCHEMA_GAP,
            "PDF 1.4's retained snapshot has no object graph; full ISO 19005-1 / ISO 15930 conformance cannot be checked from this schema; upgrade 1.4's engine to retain the object graph (see 1.7's PdfSnapshot.objects: Vec<PdfIndirectObject>) to implement real checks here.".into(),
        ));
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.pdf` (1.4/✳️x): delegates the real parse to the ✳️any subset's analyzer
    /// (same `PdfSnapshot`), then folds the honestly-scope-limited PDF/X diagnostics on top.
    pub struct PdfXAnalyzerAnalysis;

    impl ArtifactAnalysis for PdfXAnalyzerAnalysis {
        type Parts = PdfParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            PdfAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = PdfAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            if let Some(snapshot) = &inner.parts.snapshot {
                diagnostics.extend(check_pdf_x_conformance(snapshot));
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence: inner.confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PageDoc;

        #[semio_framework_async_macros::async_test]
        async fn schema_gap_diagnostic_always_fires() {
            let snapshot = PdfSnapshot { pages: vec![PageDoc { width: 612.0, height: 792.0, text: "hello".into() }, PageDoc { width: 612.0, height: 792.0, text: "a later page this check never reads".into() }], ..PdfSnapshot::default() };
            let diagnostics = check_pdf_x_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_SCHEMA_GAP && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn degenerate_page_size_is_flagged_soft() {
            let snapshot = PdfSnapshot { pages: vec![PageDoc { width: 0.0, height: 792.0, text: "x".into() }, PageDoc { width: 612.0, height: 792.0, text: "a later page this check never reads".into() }], ..PdfSnapshot::default() };
            let diagnostics = check_pdf_x_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DEGENERATE_PAGE_SIZE && d.severity == Severity::Warning), "got {diagnostics:?}");
            assert_eq!(diagnostics.len(), 2, "expected degenerate-page-size + schema-gap, got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn positive_dimensions_skip_the_page_size_check() {
            let snapshot = PdfSnapshot { pages: vec![PageDoc { width: 612.0, height: 792.0, text: "x".into() }, PageDoc { width: 612.0, height: 792.0, text: "a later page this check never reads".into() }], ..PdfSnapshot::default() };
            let diagnostics = check_pdf_x_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_DEGENERATE_PAGE_SIZE), "got {diagnostics:?}");
            assert_eq!(diagnostics.len(), 1, "expected only schema-gap, got {diagnostics:?}");
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec PdfXBuilderFacets {
        construction: PdfXBuilderConstruction,
        analysis: PdfXAnalyzerAnalysis,
        composition: super::io::derived_composition::PdfXComposerComposition,
    }
    builder: PdfXBuilder,
    analyzer: PdfXAnalyzer,
    composer: PdfXComposer,
);
//#endregion 🧬️DerivedArtifactFacets
