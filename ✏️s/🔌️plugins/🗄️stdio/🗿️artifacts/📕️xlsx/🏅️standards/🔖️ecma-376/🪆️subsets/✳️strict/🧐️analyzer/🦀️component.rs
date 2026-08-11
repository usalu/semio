//! 🧐️ XlsxStrictAnalyzer (ecma-376/✳️strict) — real ISO/IEC 29500-1 Strict conformance checks
//! against the retained `XlsxSnapshot.opc` OPC package (every part verbatim, D2 ground rule), not
//! fabricated against an unmodeled field: the Strict/Transitional distinction lives entirely in
//! `xl/workbook.xml`'s own namespace declarations and `workbook@conformance` attribute, plus the
//! package's declared content types — all bytes this engine already retains losslessly.
//!
//! Checks implemented as real, honest scans:
//! - HARD: `xl/workbook.xml` root `xmlns` is not the Strict SpreadsheetML main namespace
//!   (`http://purl.oclc.org/ooxml/spreadsheetml/main`, ISO/IEC 29500-1).
//! - HARD: `xl/workbook.xml` root `xmlns:r` is not the Strict officeDocument relationships
//!   (markup) namespace (`http://purl.oclc.org/ooxml/officeDocument/relationships`) — the
//!   "relationship-base" distinction the ticket brief calls out, same axis as docx strict.
//! - HARD: `workbook@conformance` is missing or not `"strict"` — ISO/IEC 29500-1 §12.3.24 requires
//!   an explicit declaration; a document that never says "strict" cannot be honestly stamped so.
//! - HARD: any package part declares the legacy VML drawing content type
//!   (`application/vnd.openxmlformats-officedocument.vmlDrawing`) — ISO/IEC 29500-1 Strict removes
//!   VML support entirely (the "VML rules" the ticket brief calls out, same as docx strict).
//! - SOFT: a `xl/worksheets/*.xml` part whose resolved content type isn't the worksheet content
//!   type — ECMA-376 Part 1 §12.3.24 requires an explicit worksheet content-type declaration.

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, XmlNode};
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::analyzer::XlsxAnalyzer as XlsxAnyAnalyzer;
pub use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::analyzer::XlsxParts;
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::XlsxSnapshot;

/// 🎯️ This subset's dialect coordinate.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };

//#region 🔖️Conformance
pub const CODE_NAMESPACE_MISMATCH: &str = "stdio.xlsx.strict.namespace-mismatch";
pub const CODE_RELATIONSHIPS_NAMESPACE_MISMATCH: &str = "stdio.xlsx.strict.relationships-namespace-mismatch";
pub const CODE_CONFORMANCE_ATTRIBUTE: &str = "stdio.xlsx.strict.conformance-attribute";
pub const CODE_VML_FORBIDDEN: &str = "stdio.xlsx.strict.vml-forbidden";
pub const CODE_WORKSHEET_CONTENT_TYPE: &str = "stdio.xlsx.strict.worksheet-content-type-missing";

/// 🏷️ ISO/IEC 29500-1 Strict SpreadsheetML main namespace.
pub const STRICT_SML_NS: &str = "http://purl.oclc.org/ooxml/spreadsheetml/main";
/// 🔗️ ISO/IEC 29500-1 Strict officeDocument relationships (markup) namespace, used for `r:id`
/// attributes inside content parts — see `⚙️engine`'s `REL_TYPE_OFFICE_DOCUMENT_STRICT` doc
/// comment for the sibling relationship-TYPE-URI distinction this is NOT the same axis as.
pub const STRICT_R_NS: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
const VML_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.vmlDrawing";
const WORKSHEET_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
const WORKBOOK_PART: &str = "xl/workbook.xml";

/// 🔎️ Real scan of `xl/workbook.xml`'s root element attrs -- `(xmlns, xmlns:r, conformance)`,
/// each `None` when absent. `None` overall only when the part is missing or unparsable as XML
/// (should never happen for anything that survived `✳️any` decode, but never assumed).
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

/// 🩺️ Real worksheet content-type scan. Small enough (and CODE_* consts stay subset-namespaced
/// per the pattern doc) that duplicating beats a cross-subset dependency on ✳️transitional's own
/// copy for one five-line check.
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

/// 🛡️ Real ISO/IEC 29500-1 (Strict) conformance checks against one already-decoded
/// `XlsxSnapshot`. Shared single source of truth: `XlsxStrictComposer::compose` hard-gates on
/// this (pre-serialization, authoritative), `XlsxStrictBuilder::build` hard-gates on this too, and
/// the registered `SubsetValidator` (see `🎹️composer::register`) re-runs it post-hoc against the
/// wire payload for the D5 validate-on-build hook.
pub fn check_strict_conformance(snapshot: &XlsxSnapshot) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    let Some((xmlns, xmlns_r, conformance)) = workbook_root_attrs(snapshot) else {
        out.push(hard(CODE_NAMESPACE_MISMATCH, format!("{WORKBOOK_PART} is missing or unparsable as XML -- cannot verify ISO/IEC 29500-1 Strict conformance")));
        return out;
    };
    if xmlns.as_deref() != Some(STRICT_SML_NS) {
        out.push(hard(CODE_NAMESPACE_MISMATCH, format!("{WORKBOOK_PART} root xmlns is {xmlns:?}, expected the Strict SpreadsheetML namespace {STRICT_SML_NS:?} (ISO/IEC 29500-1)")));
    }
    if xmlns_r.as_deref() != Some(STRICT_R_NS) {
        out.push(hard(
            CODE_RELATIONSHIPS_NAMESPACE_MISMATCH,
            format!("{WORKBOOK_PART} root xmlns:r is {xmlns_r:?}, expected the Strict officeDocument relationships namespace {STRICT_R_NS:?}"),
        ));
    }
    if conformance.as_deref() != Some("strict") {
        out.push(hard(CODE_CONFORMANCE_ATTRIBUTE, format!("{WORKBOOK_PART} workbook@conformance is {conformance:?}, expected \"strict\" (ISO/IEC 29500-1 §12.3.24)")));
    }
    for part in &snapshot.opc.parts {
        if part.content_type == VML_CONTENT_TYPE {
            out.push(hard(
                CODE_VML_FORBIDDEN,
                format!("part {} declares legacy VML drawing content type {VML_CONTENT_TYPE:?} -- ISO/IEC 29500-1 Strict removes VML support entirely", part.path),
            ));
        }
    }
    out.extend(worksheet_content_type_gaps(snapshot));
    out
}
//#endregion 🔖️Conformance

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.xlsx` (ecma-376/✳️strict): delegates the real parse to the ✳️any subset's
/// analyzer (same `XlsxSnapshot`), then folds real Strict conformance diagnostics on top. `sniff`
/// also delegates -- a subset-level sniff for `strict` is "is this recognizable as an xlsx at
/// all", the same probe every ecma-376 dialect shares; conformance is a separate, heavier
/// question answered by `analyze`/`check_strict_conformance`, not by `sniff`.
pub struct XlsxStrictAnalyzer;

impl ArtifactAnalyzer for XlsxStrictAnalyzer {
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
            let checks = check_strict_conformance(snapshot);
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
    fn conforming_strict_workbook_has_no_hard_diagnostics() {
        let snapshot = snapshot_with_workbook(STRICT_SML_NS, STRICT_R_NS, Some("strict"));
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn transitional_namespace_is_hard() {
        let snapshot = snapshot_with_workbook(
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
            Some("strict"),
        );
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_NAMESPACE_MISMATCH && d.severity == Severity::Error), "got {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_RELATIONSHIPS_NAMESPACE_MISMATCH && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn missing_conformance_attribute_is_hard() {
        let snapshot = snapshot_with_workbook(STRICT_SML_NS, STRICT_R_NS, None);
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTRIBUTE && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn vml_part_is_hard() {
        let mut snapshot = snapshot_with_workbook(STRICT_SML_NS, STRICT_R_NS, Some("strict"));
        snapshot.opc.set_part("xl/drawings/vmlDrawing1.vml", VML_CONTENT_TYPE, b"<xml/>".to_vec());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VML_FORBIDDEN && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn worksheet_wrong_content_type_is_soft() {
        let mut snapshot = snapshot_with_workbook(STRICT_SML_NS, STRICT_R_NS, Some("strict"));
        snapshot.opc.set_part("xl/worksheets/sheet1.xml", "application/xml", b"<worksheet/>".to_vec());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_WORKSHEET_CONTENT_TYPE && d.severity == Severity::Warning), "got {diagnostics:?}");
    }
}
