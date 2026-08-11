//! 🧐️ PptxTransitionalAnalyzer (ecma-376/✳️transitional) — real ISO/IEC 29500-4:2016 Transitional
//! conformance-class checks against the retained `PptxSnapshot.opc` (every part's bytes
//! verbatim + every real `*.rels` relationship `Type` URI -- D2 ground rule, `🎒️zip/📦️opc`'s
//! `OpcPackage` is a lossless decode, so scanning it is never fabricated against an unmodeled
//! field). Same shared ecma-376-conformance-class pattern as `📜️docx`/`📕️xlsx` ecma-376
//! ✳️transitional (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES roster).
//!
//! Real, honest scans:
//! - HARD: the root officeDocument part (`ppt/presentation.xml`) declares the Transitional
//!   PresentationML main namespace (`http://schemas.openxmlformats.org/presentationml/2006/main`).
//! - HARD: no Strict (`purl.oclc.org/ooxml`) namespace anywhere -- neither a Strict markup
//!   namespace on any part nor a Strict-base relationship type on any relationship (the same
//!   `purl.oclc.org/ooxml` prefix covers both, see `🪆️subsets/✳️strict`'s analyzer for the two
//!   axes taken separately).
//! - SOFT: the root `<p:presentation>` element's `conformance` attribute, if present, is
//!   `"transitional"` (absent is also compliant -- Transitional is the schema default).
//!
//! `📄️pdf` 1.7 `✳️a`'s `check_pdf_a_conformance` is the template this mirrors: one pure
//! `check_transitional_conformance(&PptxSnapshot) -> Vec<Diagnostic>` fn is the single source of
//! truth every other facet (this analyzer, the composer's hard gate, the builder's hard gate, the
//! registered `SubsetValidator`) calls -- never reimplemented twice.

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::analyzer::{PptxAnalyzer as PptxAnyAnalyzer, PptxParts};
use crate::artifacts::pptx::PptxSnapshot;
use crate::artifacts::zip::opc::OpcPackage;

/// 🎯️ This subset's dialect coordinate.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };

//#region 🔖️Namespaces
pub const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
/// 🏅️ Marker prefix common to EVERY ISO/IEC 29500-1 Strict namespace URI (markup namespaces
/// AND the officeDocument relationships base alike) -- see `🪆️subsets/✳️strict`'s
/// `STRICT_MAIN_NS`/`STRICT_REL_BASE`, both of which start with this prefix.
pub const STRICT_NS_MARKER: &str = "purl.oclc.org/ooxml";
//#endregion 🔖️Namespaces

//#region 🔖️Conformance
pub const CODE_MAIN_NS: &str = "stdio.pptx.transitional.main-ns-not-transitional";
pub const CODE_STRICT_NS_PRESENT: &str = "stdio.pptx.transitional.strict-ns-present";
pub const CODE_CONFORMANCE_ATTR: &str = "stdio.pptx.transitional.conformance-attr-not-transitional";

fn main_part_path(opc: &OpcPackage) -> Option<String> {
    crate::artifacts::pptx::standards::v_ecma_376::engine::resolve_office_document_relationship(opc)
}

fn part_text<'a>(opc: &'a OpcPackage, path: &str) -> Option<&'a str> {
    opc.part_bytes(path).and_then(|b| std::str::from_utf8(b).ok())
}

fn hard(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

fn soft(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

/// 🛡️ Real ISO/IEC 29500-4:2016 Transitional conformance checks against one already-decoded
/// `PptxSnapshot`. Shared single source of truth: `PptxTransitionalComposer::compose` hard-gates
/// on this (pre-serialization, authoritative), `PptxTransitionalBuilder::build` hard-gates on
/// this too, and the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
pub fn check_transitional_conformance(snapshot: &PptxSnapshot) -> Vec<Diagnostic> {
    let opc = &snapshot.opc;
    let mut out = Vec::new();

    match main_part_path(opc) {
        Some(path) => match part_text(opc, &path) {
            Some(text) => {
                if !text.contains(TRANSITIONAL_MAIN_NS) {
                    out.push(hard(CODE_MAIN_NS, format!("root officeDocument part {path} does not declare the Transitional PresentationML main namespace ({TRANSITIONAL_MAIN_NS})")));
                }
                if text.contains("conformance=\"strict\"") {
                    out.push(soft(CODE_CONFORMANCE_ATTR, format!("root officeDocument part {path}'s <p:presentation> declares conformance=\"strict\" -- Transitional expects it absent or \"transitional\"")));
                }
            }
            None => out.push(hard(CODE_MAIN_NS, format!("root officeDocument part {path} is missing or not valid utf-8 -- cannot verify the Transitional PresentationML main namespace"))),
        },
        None => out.push(hard(CODE_MAIN_NS, "package has no resolvable officeDocument relationship -- cannot verify the Transitional PresentationML main namespace".into())),
    }

    for part in &opc.parts {
        let Some(text) = std::str::from_utf8(&part.bytes).ok() else { continue };
        if text.contains(STRICT_NS_MARKER) {
            out.push(hard(CODE_STRICT_NS_PRESENT, format!("part {} declares an ISO/IEC 29500-1 Strict namespace -- ISO/IEC 29500-4 Transitional forbids it", part.path)));
        }
    }

    let mut owners: Vec<&String> = opc.relationships.keys().collect();
    owners.sort();
    for owner in owners {
        for rel in &opc.relationships[owner] {
            if rel.rel_type.contains(STRICT_NS_MARKER) {
                out.push(hard(CODE_STRICT_NS_PRESENT, format!("relationship {} owned by '{owner}' uses a Strict relationship base ({}) -- Transitional forbids it", rel.id, rel.rel_type)));
            }
        }
    }

    out
}
//#endregion 🔖️Conformance

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.pptx` (ecma-376/✳️transitional): delegates the real parse to the ✳️any
/// subset's analyzer (same `PptxSnapshot`), then folds real ISO/IEC 29500-4 Transitional
/// conformance diagnostics on top. `sniff` also delegates -- a subset-level sniff for
/// `transitional` is "is this recognizable as a pptx at all", the same OPC-shaped probe every
/// ecma-376 dialect shares; conformance is a separate, heavier question answered by
/// `analyze`/`check_transitional_conformance`, not by `sniff`.
pub struct PptxTransitionalAnalyzer;

impl ArtifactAnalyzer for PptxTransitionalAnalyzer {
    type Parts = PptxParts;
    const DIALECT: Dialect = DIALECT;

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        PptxAnyAnalyzer::sniff(source)
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let inner = PptxAnyAnalyzer::analyze(sources);
        let mut diagnostics = inner.diagnostics.clone();
        let mut confidence = inner.confidence;
        if let Some(snapshot) = &inner.parts.snapshot {
            let checks = check_transitional_conformance(snapshot);
            if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                confidence = IoConfidence::Low;
            }
            diagnostics.extend(checks);
        }
        Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
    }
}
//#endregion 🔖️Analyzer

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::zip::opc::{OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

    const TRANSITIONAL_PRESENTATION_XML: &str = concat!(
        r#"<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
        r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
        r#"<p:sldIdLst/>"#,
        "</p:presentation>",
    );

    fn transitional_snapshot() -> PptxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", TRANSITIONAL_PRESENTATION_XML.as_bytes().to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "ppt/presentation.xml");
        PptxSnapshot { opc, ..PptxSnapshot::default() }
    }

    #[test]
    fn conforming_transitional_snapshot_reports_nothing() {
        let diagnostics = check_transitional_conformance(&transitional_snapshot());
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    #[test]
    fn strict_main_ns_on_root_part_is_hard() {
        let mut snapshot = transitional_snapshot();
        let strict_xml = TRANSITIONAL_PRESENTATION_XML.replace(TRANSITIONAL_MAIN_NS, "http://purl.oclc.org/ooxml/presentationml/main");
        snapshot.opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", strict_xml.into_bytes());
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn strict_namespace_anywhere_in_package_is_hard() {
        let mut snapshot = transitional_snapshot();
        snapshot.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<p:sld xmlns:p=\"http://purl.oclc.org/ooxml/presentationml/main\"/>".to_vec());
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRICT_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn strict_relationship_base_is_hard() {
        let mut snapshot = transitional_snapshot();
        snapshot.opc.set_part("ppt/slides/slide1.xml", "application/vnd.openxmlformats-officedocument.presentationml.slide+xml", b"<p:sld/>".to_vec());
        snapshot.opc.add_relationship("ppt/presentation.xml", "rId2", "http://purl.oclc.org/ooxml/officeDocument/relationships/slide", "slides/slide1.xml");
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRICT_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn explicit_strict_conformance_attribute_is_soft() {
        let mut snapshot = transitional_snapshot();
        let with_conformance = TRANSITIONAL_PRESENTATION_XML.replace("<p:presentation ", "<p:presentation conformance=\"strict\" ");
        snapshot.opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", with_conformance.into_bytes());
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTR && d.severity == Severity::Warning), "got {diagnostics:?}");
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn missing_office_document_relationship_is_hard() {
        let snapshot = PptxSnapshot::default();
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS && d.severity == Severity::Error), "got {diagnostics:?}");
    }
}
