//! 🧬️ PdfSnapshot schema (1.4/✳️a) — reuses the ✳️any subset's `PdfSnapshot` verbatim (the SAME
//! Rust type, same `s.stdio.pdf` schema id). A subset is a validation-gated dialect STAMP on top
//! of that existing schema, not a new one -- see D4's Tier-1 "same snapshot type, subset moves"
//! semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so `🪆️subsets/✳️a/🧬️schema/`
//! is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without duplicating the schema definition.
//!
//! Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES: PDF 1.4's `PdfSnapshot` is a bare
//! `PageDoc{width,height,text}` -- no retained object graph -- so `🧐️analyzer` here implements
//! only what's honestly checkable from those fields, plus a SOFT schema-gap diagnostic. See
//! `🧐️analyzer` for the full honesty accounting.

pub use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::pdf::standards::v1_4::subsets::a::schema::check_pdf_a_conformance;
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::{diff::PdfDiff, mutations::PdfMutation, snapshot::PdfSnapshot};

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct PdfABuilderConstruction {
        snapshot: PdfSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for PdfABuilderConstruction {
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

        /// 🛡️ Re-runs the honestly-scope-limited PDF/A-1 check -- always SOFT at this schema, so
        /// `build()` never fails; the diagnostics still surface via the analyzer/composer/validator
        /// paths for anyone inspecting them.
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            let _ = check_pdf_a_conformance(&self.snapshot);
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
            let snapshot = PdfABuilderConstruction::empty().build().expect("no hard check exists at this schema; build must succeed");
            assert_eq!(snapshot.page.width, 612.0);
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::{PdfAnalyzer as PdfAnyAnalyzer, PdfParts};
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("a") };

    //#region 🔖️Conformance
    pub const CODE_TEXT_EMPTY: &str = "stdio.pdf.a.text-empty";
    pub const CODE_SCHEMA_GAP: &str = "stdio.pdf.a.schema-gap-unverifiable";

    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Honestly-scope-limited PDF/A-1 conformance check against one already-decoded `PdfSnapshot`.
    /// Shared single source of truth: `PdfAComposer::compose` (pass-through, can't hard-gate without
    /// an object graph) and the registered `SubsetValidator` both call this.
    pub fn check_pdf_a_conformance(snapshot: &PdfSnapshot) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        if snapshot.page.text.trim().is_empty() {
            out.push(soft(CODE_TEXT_EMPTY, "page.text is empty -- no extractable text content found; a very weak signal, but a real one given PageDoc has no other checkable field".into()));
        }
        out.push(soft(
            CODE_SCHEMA_GAP,
            "PDF 1.4's retained snapshot has no object graph; full ISO 19005-1 / ISO 15930 conformance cannot be checked from this schema; upgrade 1.4's engine to retain the object graph (see 1.7's PdfSnapshot.objects: Vec<PdfIndirectObject>) to implement real checks here.".into(),
        ));
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.pdf` (1.4/✳️a): delegates the real parse to the ✳️any subset's analyzer
    /// (same `PdfSnapshot`), then folds the honestly-scope-limited PDF/A-1 diagnostics on top.
    pub struct PdfAAnalyzerAnalysis;

    impl ArtifactAnalysis for PdfAAnalyzerAnalysis {
        type Parts = PdfParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            PdfAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = PdfAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            if let Some(snapshot) = &inner.parts.snapshot {
                diagnostics.extend(check_pdf_a_conformance(snapshot));
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence: inner.confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PageDoc;

        #[test]
        fn schema_gap_diagnostic_always_fires() {
            let snapshot = PdfSnapshot { page: PageDoc { width: 612.0, height: 792.0, text: "hello".into() }, ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_SCHEMA_GAP && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[test]
        fn empty_text_is_flagged_soft() {
            let snapshot = PdfSnapshot { page: PageDoc { width: 612.0, height: 792.0, text: String::new() }, ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TEXT_EMPTY && d.severity == Severity::Warning), "got {diagnostics:?}");
            assert_eq!(diagnostics.len(), 2, "expected text-empty + schema-gap, got {diagnostics:?}");
        }

        #[test]
        fn non_empty_text_skips_the_text_check() {
            let snapshot = PdfSnapshot { page: PageDoc { width: 612.0, height: 792.0, text: "content".into() }, ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_TEXT_EMPTY), "got {diagnostics:?}");
            assert_eq!(diagnostics.len(), 1, "expected only schema-gap, got {diagnostics:?}");
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec PdfABuilderFacets {
        construction: derived_construction::PdfABuilderConstruction,
        analysis: derived_analysis::PdfAAnalyzerAnalysis,
        composition: super::io::derived_composition::PdfAComposerComposition,
    }
    builder: PdfABuilder,
    analyzer: PdfAAnalyzer,
    composer: PdfAComposer,
);
//#endregion 🧬️DerivedArtifactFacets
