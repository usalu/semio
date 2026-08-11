//! 🧐️ DocxStrictAnalyzer (ecma-376/✳️strict) — real ISO/IEC 29500-1:2016 (OOXML Strict)
//! conformance checks against the retained `DocxSnapshot.opc` package (`OpcPackage`:
//! `parts[path,content_type,bytes]`, `content_types`, `relationships` — see
//! `🎒️zip/📦️opc/🦀️component.rs`). Checks are byte/string scans over `opc.parts[].bytes` and
//! `opc.relationships`, not a full XML parse -- Strict vs Transitional is fundamentally a
//! namespace-vocabulary distinction (the same markup shape under different XML namespace URIs plus
//! a package-relationship namespace swap), so a substring scan over the retained part bytes and
//! relationship types is a real, non-fabricated signal, not a fabricated check against an
//! unmodeled field.
//!
//! Checks implemented (per ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES's roster):
//! - HARD: the main document part (resolved via the root officeDocument relationship, matched by
//!   type SUFFIX so both conformance classes resolve -- see `main_document_part`) declares the
//!   strict WordprocessingML main namespace (`http://purl.oclc.org/ooxml/wordprocessingml/main`).
//! - HARD: no part's bytes anywhere in the package contain the transitional main namespace
//!   (`http://schemas.openxmlformats.org/wordprocessingml/2006/main`) -- strict conformance
//!   forbids mixing namespaces.
//! - HARD: no part's bytes anywhere contain the VML namespace (`urn:schemas-microsoft-com:vml`) --
//!   VML is transitional-only legacy drawing markup, forbidden under strict conformance.
//! - HARD: every relationship (every owner) uses the strict relationship base
//!   (`http://purl.oclc.org/ooxml/officeDocument/relationships`), never the transitional one
//!   (`http://schemas.openxmlformats.org/officeDocument/2006/relationships`).
//! - SOFT: the main document part's root element declares `conformance="strict"`.
//! - SOFT: no part's bytes contain `mc:AlternateContent` markup-compatibility markup.

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::docx::standards::v_ecma_376::subsets::any::analyzer::{DocxAnalyzer as DocxAnyAnalyzer, DocxParts};
use crate::artifacts::docx::DocxSnapshot;
use crate::artifacts::zip::opc::{resolve_relationship_target, OpcPackage, OpcPart};

/// 🎯️ This subset's dialect coordinate.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };

//#region 🔖️Namespaces
pub const STRICT_MAIN_NS: &str = "http://purl.oclc.org/ooxml/wordprocessingml/main";
pub const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
pub const STRICT_REL_BASE: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
pub const TRANSITIONAL_REL_BASE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
pub const VML_NS: &str = "urn:schemas-microsoft-com:vml";
//#endregion 🔖️Namespaces

//#region 🔖️Conformance
pub const CODE_MAIN_NS_MISSING: &str = "stdio.docx.strict.main-ns-missing";
pub const CODE_TRANSITIONAL_NS_PRESENT: &str = "stdio.docx.strict.transitional-ns-present";
pub const CODE_VML_PRESENT: &str = "stdio.docx.strict.vml-present";
pub const CODE_REL_BASE: &str = "stdio.docx.strict.non-strict-relationship-base";
pub const CODE_CONFORMANCE_ATTR: &str = "stdio.docx.strict.conformance-attr-missing";
pub const CODE_ALTERNATE_CONTENT: &str = "stdio.docx.strict.alternate-content-present";

/// 🔎️ Resolves the main document part via the root officeDocument relationship -- matched by
/// relationship-type SUFFIX (`/officeDocument`) rather than the transitional-shaped
/// `REL_TYPE_OFFICE_DOCUMENT` constant verbatim, since a genuinely strict package's root
/// relationship carries the SAME suffix under the strict base namespace (that swap is exactly what
/// `CODE_REL_BASE` below checks for) -- matching by suffix here keeps this lookup honest for both
/// conformance classes instead of silently failing to find the main part on any strict document.
fn main_document_part<'a>(opc: &'a OpcPackage) -> Option<(&'a OpcPart, String)> {
    let rel = opc.relationships_for("").iter().find(|r| r.rel_type.ends_with("/officeDocument"))?;
    let path = resolve_relationship_target("", &rel.target);
    opc.part(&path).map(|p| (p, path))
}

fn part_contains(bytes: &[u8], needle: &str) -> bool {
    !needle.is_empty() && bytes.windows(needle.len()).any(|w| w == needle.as_bytes())
}

fn hard(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

fn soft(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

/// 🛡️ Real ISO/IEC 29500-1:2016 Strict conformance checks against one already-decoded
/// `DocxSnapshot`. Shared single source of truth: `DocxStrictComposer::compose` hard-gates on
/// this (pre-serialization, authoritative), `DocxStrictBuilder::build` hard-gates on this too, and
/// the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
pub fn check_strict_conformance(snapshot: &DocxSnapshot) -> Vec<Diagnostic> {
    let opc = &snapshot.opc;
    let mut out = Vec::new();

    match main_document_part(opc) {
        Some((part, path)) => {
            if !part_contains(&part.bytes, STRICT_MAIN_NS) {
                out.push(hard(CODE_MAIN_NS_MISSING, format!("main document part {path} does not declare the strict WordprocessingML namespace {STRICT_MAIN_NS}")));
            }
            if !part_contains(&part.bytes, "conformance=\"strict\"") {
                out.push(soft(CODE_CONFORMANCE_ATTR, format!("main document part {path} root element does not declare conformance=\"strict\"")));
            }
        }
        None => out.push(hard(
            CODE_MAIN_NS_MISSING,
            "package has no root officeDocument relationship -- cannot locate the main document part to check the strict namespace on".into(),
        )),
    }

    for part in &opc.parts {
        if part_contains(&part.bytes, TRANSITIONAL_MAIN_NS) {
            out.push(hard(
                CODE_TRANSITIONAL_NS_PRESENT,
                format!("part {} contains the transitional WordprocessingML namespace {TRANSITIONAL_MAIN_NS} -- strict conformance forbids mixed namespaces", part.path),
            ));
        }
        if part_contains(&part.bytes, VML_NS) {
            out.push(hard(
                CODE_VML_PRESENT,
                format!("part {} contains the VML namespace {VML_NS} -- VML is transitional-only markup, forbidden under strict conformance", part.path),
            ));
        }
        if part_contains(&part.bytes, "mc:AlternateContent") {
            out.push(soft(CODE_ALTERNATE_CONTENT, format!("part {} contains mc:AlternateContent compatibility markup", part.path)));
        }
    }

    let mut owners: Vec<&String> = opc.relationships.keys().collect();
    owners.sort();
    for owner in owners {
        for rel in &opc.relationships[owner] {
            if rel.rel_type.starts_with(TRANSITIONAL_REL_BASE) {
                out.push(hard(
                    CODE_REL_BASE,
                    format!("relationship {} owned by {owner:?} uses the transitional relationship base {TRANSITIONAL_REL_BASE} -- strict conformance requires {STRICT_REL_BASE}", rel.id),
                ));
            }
        }
    }

    out
}
//#endregion 🔖️Conformance

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.docx` (ecma-376/✳️strict): delegates the real parse to the ✳️any subset's
/// analyzer (same `DocxSnapshot`), then folds real ISO/IEC 29500-1 Strict conformance diagnostics
/// on top.
pub struct DocxStrictAnalyzer;

impl ArtifactAnalyzer for DocxStrictAnalyzer {
    type Parts = DocxParts;
    const DIALECT: Dialect = DIALECT;

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        DocxAnyAnalyzer::sniff(source)
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let inner = DocxAnyAnalyzer::analyze(sources);
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
    use crate::artifacts::zip::opc::{OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

    fn strict_document_bytes() -> Vec<u8> {
        format!(r#"<w:document xmlns:w="{STRICT_MAIN_NS}" conformance="strict"><w:body/></w:document>"#).into_bytes()
    }

    fn snapshot_with_main_part(rel_type: &str, doc_bytes: Vec<u8>) -> DocxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", doc_bytes);
        opc.add_relationship("", "rId1", rel_type, "word/document.xml");
        DocxSnapshot::from_parts(opc, Default::default())
    }

    #[test]
    fn conforming_strict_document_has_no_hard_diagnostics() {
        let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
        let snapshot = snapshot_with_main_part(&rel_type, strict_document_bytes());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn missing_strict_namespace_is_hard() {
        let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
        let snapshot = snapshot_with_main_part(&rel_type, b"<w:document><w:body/></w:document>".to_vec());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS_MISSING && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn transitional_namespace_anywhere_is_hard() {
        let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
        let mut snapshot = snapshot_with_main_part(&rel_type, strict_document_bytes());
        snapshot.opc.set_part("word/styles.xml", "application/xml", format!(r#"<w:styles xmlns:w="{TRANSITIONAL_MAIN_NS}"/>"#).into_bytes());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TRANSITIONAL_NS_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn vml_namespace_anywhere_is_hard() {
        let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
        let mut snapshot = snapshot_with_main_part(&rel_type, strict_document_bytes());
        snapshot.opc.set_part("word/header1.xml", "application/xml", format!(r#"<w:hdr xmlns:v="{VML_NS}"/>"#).into_bytes());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VML_PRESENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn transitional_relationship_base_is_hard() {
        let snapshot = snapshot_with_main_part(REL_TYPE_OFFICE_DOCUMENT, strict_document_bytes());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_REL_BASE && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn missing_conformance_attr_is_soft() {
        let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
        let doc = format!(r#"<w:document xmlns:w="{STRICT_MAIN_NS}"><w:body/></w:document>"#).into_bytes();
        let snapshot = snapshot_with_main_part(&rel_type, doc);
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTR && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[test]
    fn alternate_content_anywhere_is_soft() {
        let rel_type = format!("{STRICT_REL_BASE}/officeDocument");
        let mut snapshot = snapshot_with_main_part(&rel_type, strict_document_bytes());
        snapshot.opc.set_part("word/document2.xml", "application/xml", b"<mc:AlternateContent/>".to_vec());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ALTERNATE_CONTENT && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[test]
    fn missing_officedocument_relationship_is_hard() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        let snapshot = DocxSnapshot::from_parts(opc, Default::default());
        let diagnostics = check_strict_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MAIN_NS_MISSING && d.severity == Severity::Error), "got {diagnostics:?}");
    }
}
