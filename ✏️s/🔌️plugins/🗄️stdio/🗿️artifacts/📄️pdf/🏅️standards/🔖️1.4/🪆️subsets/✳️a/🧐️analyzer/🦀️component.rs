//! 🧐️ PdfAAnalyzer (1.4/✳️a) — honestly-scope-limited ISO 19005-1 (PDF/A-1) conformance checks
//! against `PdfSnapshot`'s bare `PageDoc{width,height,text}` (ticket
//! 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES, the schema-gapped reference case).
//!
//! Unlike 1.7's `✳️a` (which retains the full `objects: Vec<PdfIndirectObject>` graph and can
//! genuinely check `/Encrypt`, `/OutputIntents`, `/FontDescriptor`, …), PDF 1.4's engine retains
//! NONE of that -- `PageDoc` has exactly three fields. There is no license here to fabricate
//! checks against fields the engine doesn't parse (the repo's core honesty discipline for these
//! analyzers, established by the 1.7 `✳️a` pilot's own doc comment). So this analyzer implements:
//! - SOFT, real: `page.text` is non-empty -- a weak but genuine signal that this document has
//!   *some* extractable content (real ISO 19005-1 conformance needs far more: no encryption, an
//!   embedded ICC OutputIntent, embedded fonts, no external content references -- none of which
//!   `PageDoc` retains a basis to check).
//! - SOFT, always: `stdio.pdf.a.schema-gap-unverifiable` -- states plainly that full conformance
//!   cannot be checked from this schema and names the upgrade path.

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::pdf::standards::v1_4::subsets::any::analyzer::{PdfAnalyzer as PdfAnyAnalyzer, PdfParts};
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
pub struct PdfAAnalyzer;

impl ArtifactAnalyzer for PdfAAnalyzer {
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
