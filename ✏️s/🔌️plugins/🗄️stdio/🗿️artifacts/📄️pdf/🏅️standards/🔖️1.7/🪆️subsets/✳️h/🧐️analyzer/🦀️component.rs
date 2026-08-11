//! 🧐️ PdfHAnalyzer (1.7/✳️h) — real AIIM/ASTM PDF Healthcare Best Practices Guide (2008)
//! conformance checks against the retained `PdfSnapshot.objects` graph and `snapshot.info`.
//! PDF/H is an industry best-practice guide, never an ISO standard -- ALL-SOFT, no hard checks at
//! all (see the roster's explicit "ALL-SOFT profile, no hard checks"). Ticket
//! 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
//!
//! Checks implemented as real, honest scans, ALL `Severity::Warning`:
//! - `snapshot.info.title` and `snapshot.info.author` are both present and non-empty.
//! - no `/S /JavaScript` action or bare `/JS` key anywhere (advisory here, unlike PDF/A/E's hard
//!   ban -- PDF/H is a best-practice guide with no enforcement mechanism).
//! - no `/S /Launch` action anywhere.
//! - at least one `/AcroForm/Fields` entry resolves to a dict with `/FT /Sig` -- the Best
//!   Practices Guide's signature-workflow recommendation.
//! - a `/Type /Font` object with no reachable embedded font program.

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::pdf::standards::v1_7::subsets::any::analyzer::PdfAnalyzer as PdfAnyAnalyzer;
pub use crate::artifacts::pdf::standards::v1_7::subsets::any::analyzer::PdfParts;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfSnapshot};

/// 🎯️ This subset's dialect coordinate.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("h") };

//#region 🔖️Conformance
pub const CODE_INFO_TITLE_OR_AUTHOR: &str = "stdio.pdf.h.missing-info-title-or-author";
pub const CODE_JAVASCRIPT: &str = "stdio.pdf.h.javascript-action";
pub const CODE_LAUNCH: &str = "stdio.pdf.h.launch-action";
pub const CODE_SIGNATURE_FIELD: &str = "stdio.pdf.h.missing-signature-field";
pub const CODE_FONT_NOT_EMBEDDED: &str = "stdio.pdf.h.font-not-embedded";

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

fn scan_action_subtype(objects: &[PdfIndirectObject], subtype: &str) -> Vec<ObjRef> {
    objects.iter().filter(|o| o.value.as_dict().map(|d| dict_name(d, "S") == Some(subtype)).unwrap_or(false)).map(|o| o.id).collect()
}

fn scan_js_key_only(objects: &[PdfIndirectObject], already: &[ObjRef]) -> Vec<ObjRef> {
    objects
        .iter()
        .filter(|o| !already.contains(&o.id) && o.value.as_dict().map(|d| d.iter().any(|e| e.key == "JS")).unwrap_or(false))
        .map(|o| o.id)
        .collect()
}

/// ✍️ Real check: `/Root/AcroForm/Fields` contains a resolved entry with `/FT /Sig`.
fn has_signature_field(objects: &[PdfIndirectObject]) -> bool {
    let Some(catalog) = find_catalog(objects) else { return false };
    let Some(acroform) = catalog.dict_get("AcroForm").and_then(|v| resolve_item(objects, v)) else { return false };
    let Some(fields) = acroform.dict_get("Fields").and_then(|v| v.as_array()) else { return false };
    fields.iter().any(|item| resolve_item(objects, item).and_then(|f| f.as_dict()).map(|d| dict_name(d, "FT") == Some("Sig")).unwrap_or(false))
}

fn descriptor_has_embedded_file(objects: &[PdfIndirectObject], desc_ref: ObjRef) -> bool {
    resolve_ref(objects, desc_ref)
        .and_then(|o| o.as_dict())
        .map(|d| d.iter().any(|e| e.key == "FontFile" || e.key == "FontFile2" || e.key == "FontFile3"))
        .unwrap_or(false)
}

fn non_embedded_fonts(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
    let mut out = Vec::new();
    for o in objects {
        let Some(d) = o.value.as_dict() else { continue };
        if dict_name(d, "Type") != Some("Font") {
            continue;
        }
        let direct = d.iter().find(|e| e.key == "FontDescriptor").and_then(|e| e.value.as_ref()).map(|r| descriptor_has_embedded_file(objects, r)).unwrap_or(false);
        let via_descendants = d
            .iter()
            .find(|e| e.key == "DescendantFonts")
            .and_then(|e| e.value.as_array())
            .map(|arr| {
                arr.iter().any(|item| {
                    resolve_item(objects, item)
                        .and_then(|desc| desc.as_dict())
                        .and_then(|dd| dd.iter().find(|e| e.key == "FontDescriptor").and_then(|e| e.value.as_ref()))
                        .map(|r| descriptor_has_embedded_file(objects, r))
                        .unwrap_or(false)
                })
            })
            .unwrap_or(false);
        if !direct && !via_descendants {
            out.push(o.id);
        }
    }
    out
}

fn soft(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

/// 🛡️ Real AIIM/ASTM PDF Healthcare Best Practices Guide (2008) checks against one
/// already-decoded `PdfSnapshot`. ALL-SOFT by design (industry best-practice guide, never an ISO
/// standard, no enforcement mechanism) -- never returns a `Severity::Error`/`Fatal` diagnostic.
/// Shared single source of truth used by `PdfHComposer` and `PdfHValidator` (both pass-through,
/// per the roster's "never hard-gates").
pub fn check_h_conformance(snapshot: &PdfSnapshot) -> Vec<Diagnostic> {
    let objects = &snapshot.objects;
    let mut out = Vec::new();
    let title_ok = snapshot.info.title.as_deref().map(|s| !s.is_empty()).unwrap_or(false);
    let author_ok = snapshot.info.author.as_deref().map(|s| !s.is_empty()).unwrap_or(false);
    if !title_ok || !author_ok {
        out.push(soft(CODE_INFO_TITLE_OR_AUTHOR, "document Info.title and/or Info.author is absent or empty -- the PDF Healthcare Best Practices Guide recommends both be populated".into()));
    }
    let js_actions = scan_action_subtype(objects, "JavaScript");
    for r in &js_actions {
        out.push(soft(CODE_JAVASCRIPT, format!("object {} {} R is an /S /JavaScript action -- the PDF Healthcare Best Practices Guide discourages embedded JavaScript", r.num, r.gen)));
    }
    for r in scan_js_key_only(objects, &js_actions) {
        out.push(soft(CODE_JAVASCRIPT, format!("object {} {} R carries a /JS key -- the PDF Healthcare Best Practices Guide discourages embedded JavaScript", r.num, r.gen)));
    }
    for r in scan_action_subtype(objects, "Launch") {
        out.push(soft(CODE_LAUNCH, format!("object {} {} R is an /S /Launch action -- the PDF Healthcare Best Practices Guide discourages launch actions", r.num, r.gen)));
    }
    if !has_signature_field(objects) {
        out.push(soft(CODE_SIGNATURE_FIELD, "no /AcroForm field with /FT /Sig found -- the PDF Healthcare Best Practices Guide recommends a signature workflow".into()));
    }
    for r in non_embedded_fonts(objects) {
        out.push(soft(CODE_FONT_NOT_EMBEDDED, format!("font object {} {} R has no FontFile/FontFile2/FontFile3 reachable from its FontDescriptor -- the Guide recommends embedded fonts", r.num, r.gen)));
    }
    out
}
//#endregion 🔖️Conformance

//#region 🔖️Analyzer
pub struct PdfHAnalyzer;

impl ArtifactAnalyzer for PdfHAnalyzer {
    type Parts = PdfParts;
    const DIALECT: Dialect = DIALECT;

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        PdfAnyAnalyzer::sniff(source)
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let inner = PdfAnyAnalyzer::analyze(sources);
        let mut diagnostics = inner.diagnostics.clone();
        // ℹ️ ALL-SOFT profile -- confidence is never downgraded by check_h_conformance's output
        // since it never returns Error/Fatal.
        let confidence = inner.confidence;
        if let Some(snapshot) = &inner.parts.snapshot {
            diagnostics.extend(check_h_conformance(snapshot));
        }
        Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
    }
}
//#endregion 🔖️Analyzer

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfInfo;

    #[test]
    fn empty_snapshot_reports_only_soft_findings() {
        let snapshot = PdfSnapshot::default();
        let diagnostics = check_h_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error && d.severity != Severity::Fatal), "PDF/H must never emit a hard diagnostic: got {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_INFO_TITLE_OR_AUTHOR));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_SIGNATURE_FIELD));
    }

    #[test]
    fn title_and_author_present_clears_that_finding() {
        let snapshot = PdfSnapshot { info: PdfInfo { title: Some("A Chart".into()), author: Some("Dr. X".into()), ..PdfInfo::default() }, ..PdfSnapshot::default() };
        let diagnostics = check_h_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_INFO_TITLE_OR_AUTHOR), "got {diagnostics:?}");
    }

    #[test]
    fn javascript_action_is_soft_never_hard() {
        let objects = vec![PdfIndirectObject {
            id: ObjRef { num: 1, gen: 0 },
            value: PdfObject::Dict(vec![PdfDictEntry { key: "S".into(), value: PdfObject::Name("JavaScript".into()) }]),
        }];
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_h_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_JAVASCRIPT && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[test]
    fn signature_field_present_clears_that_finding() {
        let objects = vec![
            PdfIndirectObject {
                id: ObjRef { num: 1, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) },
                    PdfDictEntry { key: "AcroForm".into(), value: PdfObject::Ref(ObjRef { num: 2, gen: 0 }) },
                ]),
            },
            PdfIndirectObject {
                id: ObjRef { num: 2, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "Fields".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 3, gen: 0 })]) }]),
            },
            PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "FT".into(), value: PdfObject::Name("Sig".into()) }]),
            },
        ];
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_h_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_SIGNATURE_FIELD), "got {diagnostics:?}");
    }
}
