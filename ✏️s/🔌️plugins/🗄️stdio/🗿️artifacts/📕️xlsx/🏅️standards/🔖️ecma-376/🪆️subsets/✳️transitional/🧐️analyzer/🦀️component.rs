//! 🧐️ XlsxTransitionalAnalyzer (ecma-376/✳️transitional) — real ISO/IEC 29500-4 Transitional
//! conformance checks against the retained `XlsxSnapshot.opc` OPC package, mirroring
//! ✳️strict's shape (same namespace/relationship-base signals, opposite polarity -- see that
//! module's doc comment for the shared rationale). Transitional is ECMA-376/ISO 29500-4's
//! permissive baseline profile, so its checks are less about "what's forbidden" (Transitional
//! legitimately allows VML, legacy elements, macros -- none of that is flagged here) and more
//! about namespace/attribute consistency: catching a document that is actually Strict-shaped (or
//! unparsable) but got routed through this dialect by mistake.
//!
//! Checks implemented as real, honest scans:
//! - HARD: `xl/workbook.xml` root `xmlns` is not the Transitional SpreadsheetML main namespace.
//! - HARD: `xl/workbook.xml` root `xmlns:r` is not the Transitional officeDocument relationships
//!   namespace.
//! - HARD: `workbook@conformance` is explicitly `"strict"` -- a document that declares Strict
//!   conformance cannot be honestly stamped Transitional (ISO/IEC 29500-1 §12.3.24's default is
//!   Transitional when the attribute is absent, so an ABSENT attribute is fine here).
//! - SOFT: a `xl/worksheets/*.xml` part whose resolved content type isn't the worksheet content
//!   type (same real ECMA-376 Part 1 §12.3.24 check as ✳️strict).

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, XmlNode};
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::analyzer::XlsxAnalyzer as XlsxAnyAnalyzer;
pub use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::analyzer::XlsxParts;
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::XlsxSnapshot;

/// 🎯️ This subset's dialect coordinate.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };

//#region 🔖️Conformance
pub const CODE_NAMESPACE_MISMATCH: &str = "stdio.xlsx.transitional.namespace-mismatch";
pub const CODE_RELATIONSHIPS_NAMESPACE_MISMATCH: &str = "stdio.xlsx.transitional.relationships-namespace-mismatch";
pub const CODE_CONFORMANCE_ATTRIBUTE: &str = "stdio.xlsx.transitional.conformance-attribute";
pub const CODE_WORKSHEET_CONTENT_TYPE: &str = "stdio.xlsx.transitional.worksheet-content-type-missing";

/// 🏷️ ISO/IEC 29500-4 Transitional SpreadsheetML main namespace (same value the shared
/// `⚙️engine`'s private `SML_NS` uses -- duplicated here as a `pub` constant since the engine's
/// copy isn't exported).
pub const TRANSITIONAL_SML_NS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
/// 🔗️ ISO/IEC 29500-4 Transitional officeDocument relationships (markup) namespace.
pub const TRANSITIONAL_R_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const WORKSHEET_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
const WORKBOOK_PART: &str = "xl/workbook.xml";

/// 🔎️ Real scan of `xl/workbook.xml`'s root element attrs -- `(xmlns, xmlns:r, conformance)`,
/// each `None` when absent. `None` overall only when the part is missing or unparsable as XML.
fn workbook_root_attrs(snapshot: &XlsxSnapshot) -> Option<(Option<String>, Option<String>, Option<String>)> {
    let bytes = snapshot.opc.part_bytes(WORKBOOK_PART)?;
    let text = std::str::from_utf8(bytes).ok()?;
    let doc = xml_document_from_text(text).ok()?;
    let XmlNode::Element { name, attrs, .. } = doc.root? else { return None };
    if name != "workbook" {
        return None;
    }
    let get = |n: &str| attrs.iter().find(|a| a.name == n).map(|a| a.value.clone());
    Some((get("xmlns"), get("xmlns:r"), get("conformance")))
}

fn hard(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

fn soft(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

/// 🩺️ Real worksheet content-type scan -- same check as ✳️strict's own copy, duplicated (small
/// enough, CODE_* consts stay subset-namespaced) rather than a cross-subset dependency.
fn worksheet_content_type_gaps(snapshot: &XlsxSnapshot) -> Vec<Diagnostic> {
    snapshot
        .opc
        .parts
        .iter()
        .filter(|p| p.path.starts_with("xl/worksheets/") && p.path.ends_with(".xml") && p.content_type != WORKSHEET_CONTENT_TYPE)
        .map(|p| {
            soft(
                CODE_WORKSHEET_CONTENT_TYPE,
                format!("worksheet part {} resolves content type {:?}, expected {WORKSHEET_CONTENT_TYPE:?} (ECMA-376 Part 1 §12.3.24)", p.path, p.content_type),
            )
        })
        .collect()
}

/// 🛡️ Real ISO/IEC 29500-4 (Transitional) conformance checks against one already-decoded
/// `XlsxSnapshot`. Same single-source-of-truth role as ✳️strict's `check_strict_conformance`:
/// `XlsxTransitionalComposer::compose` and `XlsxTransitionalBuilder::build` hard-gate on this, and
/// the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
pub fn check_transitional_conformance(snapshot: &XlsxSnapshot) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    let Some((xmlns, xmlns_r, conformance)) = workbook_root_attrs(snapshot) else {
        out.push(hard(CODE_NAMESPACE_MISMATCH, format!("{WORKBOOK_PART} is missing or unparsable as XML -- cannot verify ISO/IEC 29500-4 Transitional conformance")));
        return out;
    };
    if xmlns.as_deref() != Some(TRANSITIONAL_SML_NS) {
        out.push(hard(
            CODE_NAMESPACE_MISMATCH,
            format!("{WORKBOOK_PART} root xmlns is {xmlns:?}, expected the Transitional SpreadsheetML namespace {TRANSITIONAL_SML_NS:?} (ISO/IEC 29500-4)"),
        ));
    }
    if xmlns_r.as_deref() != Some(TRANSITIONAL_R_NS) {
        out.push(hard(
            CODE_RELATIONSHIPS_NAMESPACE_MISMATCH,
            format!("{WORKBOOK_PART} root xmlns:r is {xmlns_r:?}, expected the Transitional officeDocument relationships namespace {TRANSITIONAL_R_NS:?}"),
        ));
    }
    if conformance.as_deref() == Some("strict") {
        out.push(hard(
            CODE_CONFORMANCE_ATTRIBUTE,
            format!("{WORKBOOK_PART} workbook@conformance is \"strict\" -- a document that declares Strict conformance cannot be honestly stamped Transitional"),
        ));
    }
    out.extend(worksheet_content_type_gaps(snapshot));
    out
}
//#endregion 🔖️Conformance

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.xlsx` (ecma-376/✳️transitional): delegates the real parse to the ✳️any
/// subset's analyzer (same `XlsxSnapshot`), then folds real Transitional conformance diagnostics
/// on top. `sniff` also delegates -- same rationale as ✳️strict's analyzer.
pub struct XlsxTransitionalAnalyzer;

impl ArtifactAnalyzer for XlsxTransitionalAnalyzer {
    type Parts = XlsxParts;
    const DIALECT: Dialect = DIALECT;

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        XlsxAnyAnalyzer::sniff(source)
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let inner = XlsxAnyAnalyzer::analyze(sources);
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
    use crate::artifacts::xml::schema::snapshot::{xml_document_to_text, XmlAttr, XmlDocument};
    use crate::artifacts::zip::opc::OpcPackage;

    fn attr(name: &str, value: &str) -> XmlAttr {
        XmlAttr { name: name.into(), value: value.into() }
    }

    fn workbook_xml(xmlns: &str, xmlns_r: &str, conformance: Option<&str>) -> Vec<u8> {
        let mut attrs = vec![attr("xmlns", xmlns), attr("xmlns:r", xmlns_r)];
        if let Some(c) = conformance {
            attrs.push(attr("conformance", c));
        }
        let doc = XmlDocument {
            root: Some(XmlNode::Element {
                name: "workbook".into(),
                attrs,
                children: vec![XmlNode::Element { name: "sheets".into(), attrs: vec![], children: vec![] }],
            }),
            doctype: None,
            declaration: None,
        };
        xml_document_to_text(&doc).into_bytes()
    }

    fn snapshot_with_workbook(xmlns: &str, xmlns_r: &str, conformance: Option<&str>) -> XlsxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.set_part(WORKBOOK_PART, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml", workbook_xml(xmlns, xmlns_r, conformance));
        XlsxSnapshot::from_parts(opc, Default::default())
    }

    #[test]
    fn conforming_transitional_workbook_has_no_hard_diagnostics() {
        let snapshot = snapshot_with_workbook(TRANSITIONAL_SML_NS, TRANSITIONAL_R_NS, None);
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn strict_namespace_is_hard() {
        let snapshot = snapshot_with_workbook("http://purl.oclc.org/ooxml/spreadsheetml/main", "http://purl.oclc.org/ooxml/officeDocument/relationships", None);
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_NAMESPACE_MISMATCH && d.severity == Severity::Error), "got {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_RELATIONSHIPS_NAMESPACE_MISMATCH && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn explicit_strict_conformance_attribute_is_hard() {
        let snapshot = snapshot_with_workbook(TRANSITIONAL_SML_NS, TRANSITIONAL_R_NS, Some("strict"));
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTRIBUTE && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn explicit_transitional_conformance_attribute_is_fine() {
        let snapshot = snapshot_with_workbook(TRANSITIONAL_SML_NS, TRANSITIONAL_R_NS, Some("transitional"));
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn worksheet_wrong_content_type_is_soft() {
        let mut snapshot = snapshot_with_workbook(TRANSITIONAL_SML_NS, TRANSITIONAL_R_NS, None);
        snapshot.opc.set_part("xl/worksheets/sheet1.xml", "application/xml", b"<worksheet/>".to_vec());
        let diagnostics = check_transitional_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_WORKSHEET_CONTENT_TYPE && d.severity == Severity::Warning), "got {diagnostics:?}");
    }
}
