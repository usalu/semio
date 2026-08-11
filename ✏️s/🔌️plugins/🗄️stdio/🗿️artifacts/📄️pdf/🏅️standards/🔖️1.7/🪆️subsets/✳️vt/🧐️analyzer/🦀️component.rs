//! 🧐️ PdfVtAnalyzer (1.7/✳️vt) — real ISO 16612-2:2010 (PDF/VT-1/-2) conformance checks, layered
//! on top of PDF/X-4 (ISO 16612-2 is explicitly based on ISO 15930-7): this subset calls `✳️x`'s
//! own `check_x_conformance` directly (per the roster's explicit guidance -- "may call x's
//! check_x_conformance fn from vt's if that's cleaner than duplicating") and adds its own
//! VT-specific checks on top. Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
//!
//! Checks implemented as real, honest scans:
//! - HARD (VT-specific): `/Root` carries no `/DPartRoot` key -- ISO 16612-2's variable-data
//!   partitioning mechanism is entirely absent without it.
//! - HARD (inherited): every ISO 15930-7 (PDF/X-4) check `✳️x::check_x_conformance` runs --
//!   encryption, OutputIntent, TrimBox/ArtBox, since VT is PDF/X-4-based.
//! - SOFT: a DPart node reachable from `/DPartRoot/DParts` (recursively, since DPart nodes may
//!   themselves carry a nested `/DParts` array) with no `/DPM` metadata dict key.

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::pdf::standards::v1_7::subsets::any::analyzer::PdfAnalyzer as PdfAnyAnalyzer;
pub use crate::artifacts::pdf::standards::v1_7::subsets::any::analyzer::PdfParts;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfSnapshot};
use crate::artifacts::pdf::standards::v1_7::subsets::x::analyzer::check_x_conformance;

/// 🎯️ This subset's dialect coordinate.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("vt") };

//#region 🔖️Conformance
pub const CODE_DPART_ROOT: &str = "stdio.pdf.vt.missing-dpartroot";
pub const CODE_DPM: &str = "stdio.pdf.vt.dpart-missing-dpm";

fn dict_name<'a>(dict: &'a [PdfDictEntry], key: &str) -> Option<&'a str> {
    dict.iter().find(|e| e.key == key).and_then(|e| e.value.as_name())
}

fn resolve_ref<'a>(objects: &'a [PdfIndirectObject], r: ObjRef) -> Option<&'a PdfObject> {
    objects.iter().find(|o| o.id == r).map(|o| &o.value)
}

fn resolve_item<'a>(objects: &'a [PdfIndirectObject], item: &'a PdfObject) -> Option<&'a PdfObject> {
    match item {
        PdfObject::Ref(r) => resolve_ref(objects, *r),
        other => Some(other),
    }
}

fn find_catalog(objects: &[PdfIndirectObject]) -> Option<&PdfObject> {
    objects.iter().find(|o| o.value.as_dict().map(|d| dict_name(d, "Type") == Some("Catalog")).unwrap_or(false)).map(|o| &o.value)
}

/// 🌳️ Real, recursive walk of the `/DPartRoot`/`/DParts` tree (ISO 16612-2 §6.2): each DPart
/// node may itself carry a nested `/DParts` array. Returns the refs of every DPart node reachable
/// from the root that lacks a `/DPM` metadata dict key. `visited` guards against a malformed
/// document with a reference cycle.
fn dparts_missing_dpm(objects: &[PdfIndirectObject], node: &PdfObject, visited: &mut Vec<ObjRef>, out: &mut Vec<ObjRef>) {
    let Some(dparts) = node.dict_get("DParts").and_then(|v| v.as_array()) else { return };
    for item in dparts {
        let PdfObject::Ref(r) = item else { continue };
        if visited.contains(r) {
            continue;
        }
        visited.push(*r);
        let Some(resolved) = resolve_ref(objects, *r) else { continue };
        if resolved.dict_get("DPM").is_none() {
            out.push(*r);
        }
        dparts_missing_dpm(objects, resolved, visited, out);
    }
}

fn hard(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

fn soft(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

/// 🛡️ Real ISO 16612-2:2010 (PDF/VT-1/-2) conformance checks against one already-decoded
/// `PdfSnapshot`: the full ISO 15930-7 (PDF/X-4) check suite (`✳️x::check_x_conformance`) plus
/// VT's own `/DPartRoot`/`/DPM` checks. Shared single source of truth used by `PdfVtComposer`,
/// `PdfVtBuilder`, and `PdfVtValidator`.
pub fn check_vt_conformance(snapshot: &PdfSnapshot) -> Vec<Diagnostic> {
    let objects = &snapshot.objects;
    let mut out = check_x_conformance(snapshot);
    let Some(catalog) = find_catalog(objects) else {
        out.push(hard(CODE_DPART_ROOT, "no /Type /Catalog object found -- cannot verify /DPartRoot".into()));
        return out;
    };
    match catalog.dict_get("DPartRoot").and_then(|v| resolve_item(objects, v)) {
        None => out.push(hard(CODE_DPART_ROOT, "/Root carries no /DPartRoot key -- ISO 16612-2's variable-data partitioning mechanism is entirely absent".into())),
        Some(root) => {
            let mut visited = Vec::new();
            let mut missing = Vec::new();
            dparts_missing_dpm(objects, root, &mut visited, &mut missing);
            for r in missing {
                out.push(soft(CODE_DPM, format!("DPart node {} {} R has no /DPM metadata dict -- ISO 16612-2 expects per-part metadata", r.num, r.gen)));
            }
        }
    }
    out
}
//#endregion 🔖️Conformance

//#region 🔖️Analyzer
pub struct PdfVtAnalyzer;

impl ArtifactAnalyzer for PdfVtAnalyzer {
    type Parts = PdfParts;
    const DIALECT: Dialect = DIALECT;

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        PdfAnyAnalyzer::sniff(source)
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let inner = PdfAnyAnalyzer::analyze(sources);
        let mut diagnostics = inner.diagnostics.clone();
        let mut confidence = inner.confidence;
        if let Some(snapshot) = &inner.parts.snapshot {
            let checks = check_vt_conformance(snapshot);
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

    fn conforming_x_objects() -> Vec<PdfIndirectObject> {
        vec![
            PdfIndirectObject {
                id: ObjRef { num: 1, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) },
                    PdfDictEntry { key: "OutputIntents".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 2, gen: 0 })]) },
                    PdfDictEntry { key: "DPartRoot".into(), value: PdfObject::Ref(ObjRef { num: 10, gen: 0 }) },
                ]),
            },
            PdfIndirectObject {
                id: ObjRef { num: 2, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("OutputIntent".into()) },
                    PdfDictEntry { key: "S".into(), value: PdfObject::Name("GTS_PDFX".into()) },
                    PdfDictEntry { key: "DestOutputProfile".into(), value: PdfObject::Ref(ObjRef { num: 9, gen: 0 }) },
                ]),
            },
            PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Page".into()) },
                    PdfDictEntry { key: "TrimBox".into(), value: PdfObject::Array(vec![PdfObject::Int(0), PdfObject::Int(0), PdfObject::Int(100), PdfObject::Int(100)]) },
                ]),
            },
            PdfIndirectObject {
                id: ObjRef { num: 10, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("DPartRoot".into()) },
                    PdfDictEntry { key: "DParts".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 11, gen: 0 })]) },
                ]),
            },
            PdfIndirectObject {
                id: ObjRef { num: 11, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("DPart".into()) },
                    PdfDictEntry { key: "DPM".into(), value: PdfObject::Dict(vec![]) },
                ]),
            },
        ]
    }

    #[test]
    fn fully_conforming_vt_document_has_no_hard_diagnostics() {
        let snapshot = PdfSnapshot { objects: conforming_x_objects(), ..PdfSnapshot::default() };
        let diagnostics = check_vt_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn missing_dpartroot_is_hard() {
        let mut objects = conforming_x_objects();
        if let Some(catalog_obj) = objects.iter_mut().find(|o| o.id.num == 1) {
            if let PdfObject::Dict(d) = &mut catalog_obj.value {
                d.retain(|e| e.key != "DPartRoot");
            }
        }
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_vt_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DPART_ROOT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn x_violations_are_inherited_as_hard() {
        // No OutputIntent at all -- an X-4 violation must surface through vt too.
        let snapshot = PdfSnapshot::default();
        let diagnostics = check_vt_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == crate::artifacts::pdf::standards::v1_7::subsets::x::analyzer::CODE_OUTPUT_INTENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn dpart_missing_dpm_is_soft() {
        let mut objects = conforming_x_objects();
        if let Some(dpart) = objects.iter_mut().find(|o| o.id.num == 11) {
            if let PdfObject::Dict(d) = &mut dpart.value {
                d.retain(|e| e.key != "DPM");
            }
        }
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_vt_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DPM && d.severity == Severity::Warning), "got {diagnostics:?}");
    }
}
