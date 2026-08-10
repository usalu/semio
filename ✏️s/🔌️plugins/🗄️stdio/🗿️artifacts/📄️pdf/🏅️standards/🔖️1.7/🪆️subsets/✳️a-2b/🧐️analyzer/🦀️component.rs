//! 🧐️ PdfA2bAnalyzer (1.7/✳️a-2b) — real-only-so-far ISO 19005-2 (PDF/A-2b) conformance checks
//! against the retained `PdfSnapshot.objects` graph (D5, ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION §D5). This is the FIRST
//! real, non-`✳️any` subset in the whole repo: every one of the other 82 artifacts' subsets today
//! is the degenerate `✳️any` (no real differentiation).
//!
//! Checks implemented as real, honest scans (never fabricated against fields the engine doesn't
//! parse):
//! - HARD (blocks the `a-2b` dialect stamp): `/Encrypt` — since decode already rejects any file
//!   whose *trailer* declares `/Encrypt` (`PdfEngineError::Unsupported`, see `⚙️engine`), and the
//!   `PdfSnapshot` model doesn't retain the trailer at all, this check is deliberately independent
//!   of that decode-time rejection: it scans the retained object graph for a real Standard
//!   Security Handler dictionary shape (`/Filter /Standard` + `/V`/`/R`/`/O`/`/U`), so a document
//!   that reached this validator via any future decode-tolerant path is still caught.
//! - HARD: `/S /JavaScript` action or a bare `/JS` key anywhere in the retained object graph.
//! - HARD: `/S /Launch` action anywhere in the retained object graph.
//! - SOFT: no `OutputIntent` with `/S /GTS_PDFA1` reachable from `/Root`'s `/OutputIntents`.
//! - SOFT: a `/Type /Font` object (or a `/DescendantFonts` composite) with no reachable
//!   `/FontFile`/`/FontFile2`/`/FontFile3` on its `/FontDescriptor` -- real, since `objects`
//!   retains the FULL raw indirect-object graph (fonts included) losslessly; this is NOT
//!   fabricated against an unmodeled field.

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::pdf::standards::v1_7::subsets::any::analyzer::PdfAnalyzer as PdfAnyAnalyzer;
pub use crate::artifacts::pdf::standards::v1_7::subsets::any::analyzer::PdfParts;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfIndirectObject, PdfObject, PdfSnapshot};

/// 🎯️ This subset's dialect coordinate.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("a-2b") };

//#region 🔖️Conformance
pub const CODE_ENCRYPT: &str = "stdio.pdf.a2b.encrypt-present";
pub const CODE_JAVASCRIPT: &str = "stdio.pdf.a2b.javascript-action";
pub const CODE_LAUNCH: &str = "stdio.pdf.a2b.launch-action";
pub const CODE_OUTPUT_INTENT: &str = "stdio.pdf.a2b.missing-output-intent";
pub const CODE_FONT_NOT_EMBEDDED: &str = "stdio.pdf.a2b.font-not-embedded";

fn resolve_ref<'a>(objects: &'a [PdfIndirectObject], r: ObjRef) -> Option<&'a PdfObject> {
    objects.iter().find(|o| o.id == r).map(|o| &o.value)
}

fn resolve_item<'a>(objects: &'a [PdfIndirectObject], item: &'a PdfObject) -> Option<&'a PdfObject> {
    match item {
        PdfObject::Ref(r) => resolve_ref(objects, *r),
        other => Some(other),
    }
}

fn dict_name<'a>(dict: &'a [crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfDictEntry], key: &str) -> Option<&'a str> {
    dict.iter().find(|e| e.key == key).and_then(|e| e.value.as_name())
}

/// 🔒️ Real, independent-of-decode scan: does any retained object look like a Standard Security
/// Handler encryption dictionary (`/Filter /Standard` + `/V`/`/R`/`/O`/`/U`)?
fn scan_encryption(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
    objects
        .iter()
        .filter(|o| {
            let Some(d) = o.value.as_dict() else { return false };
            dict_name(d, "Filter") == Some("Standard")
                && d.iter().any(|e| e.key == "V")
                && d.iter().any(|e| e.key == "R")
                && d.iter().any(|e| e.key == "O")
                && d.iter().any(|e| e.key == "U")
        })
        .map(|o| o.id)
        .collect()
}

/// 📜️ Real scan for `/S /<subtype>` action dictionaries anywhere in the retained object graph.
fn scan_action_subtype(objects: &[PdfIndirectObject], subtype: &str) -> Vec<ObjRef> {
    objects.iter().filter(|o| o.value.as_dict().map(|d| dict_name(d, "S") == Some(subtype)).unwrap_or(false)).map(|o| o.id).collect()
}

/// 📜️ Real scan for a bare `/JS` key not already caught by `/S /JavaScript` (some JS action
/// dicts carry `/JS` without a matching `/S` when malformed/hand-authored -- PDF/A-2b forbids the
/// key itself, not just the well-formed `/S /JavaScript` shape).
fn scan_js_key_only(objects: &[PdfIndirectObject], already: &[ObjRef]) -> Vec<ObjRef> {
    objects
        .iter()
        .filter(|o| !already.contains(&o.id) && o.value.as_dict().map(|d| d.iter().any(|e| e.key == "JS")).unwrap_or(false))
        .map(|o| o.id)
        .collect()
}

fn find_catalog(objects: &[PdfIndirectObject]) -> Option<&PdfObject> {
    objects.iter().find(|o| o.value.as_dict().map(|d| dict_name(d, "Type") == Some("Catalog")).unwrap_or(false)).map(|o| &o.value)
}

/// 🏳️ Real check: `/Root`'s `/OutputIntents` array contains an intent with `/S /GTS_PDFA1`.
fn has_pdfa_output_intent(objects: &[PdfIndirectObject]) -> bool {
    let Some(catalog) = find_catalog(objects) else { return false };
    let Some(intents) = catalog.dict_get("OutputIntents").and_then(|v| v.as_array()) else { return false };
    intents.iter().any(|item| {
        resolve_item(objects, item).and_then(|o| o.as_dict()).map(|d| dict_name(d, "S") == Some("GTS_PDFA1")).unwrap_or(false)
    })
}

fn descriptor_has_embedded_file(objects: &[PdfIndirectObject], desc_ref: ObjRef) -> bool {
    resolve_ref(objects, desc_ref)
        .and_then(|o| o.as_dict())
        .map(|d| d.iter().any(|e| e.key == "FontFile" || e.key == "FontFile2" || e.key == "FontFile3"))
        .unwrap_or(false)
}

/// 🔤️ Real check: every `/Type /Font` object (simple or `/DescendantFonts` composite) resolves
/// to a `/FontDescriptor` carrying an embedded font program. Real because `objects` retains the
/// FULL raw indirect-object graph losslessly (D2 ground rule) -- font dicts are genuinely present
/// here, this is not fabricated against a field the engine doesn't parse.
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

fn hard(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

fn soft(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

/// 🛡️ Real ISO 19005-2 (PDF/A-2b) conformance checks against one already-decoded `PdfSnapshot`.
/// Shared single source of truth: `PdfA2bComposer::compose` hard-gates on this (pre-serialization,
/// authoritative), `PdfA2bBuilder::build` hard-gates on this too, and the generic
/// `SubsetValidator` (registered from `🎹️composer::register`) re-runs it post-hoc against the
/// wire payload for the D5 validate-on-build hook.
pub fn check_pdf_a2b_conformance(snapshot: &PdfSnapshot) -> Vec<Diagnostic> {
    let objects = &snapshot.objects;
    let mut out = Vec::new();
    for r in scan_encryption(objects) {
        out.push(hard(CODE_ENCRYPT, format!("object {} {} R looks like a Standard Security Handler encryption dictionary -- PDF/A-2b forbids /Encrypt", r.num, r.gen)));
    }
    let js_actions = scan_action_subtype(objects, "JavaScript");
    for r in &js_actions {
        out.push(hard(CODE_JAVASCRIPT, format!("object {} {} R is an /S /JavaScript action -- PDF/A-2b forbids embedded JavaScript", r.num, r.gen)));
    }
    for r in scan_js_key_only(objects, &js_actions) {
        out.push(hard(CODE_JAVASCRIPT, format!("object {} {} R carries a /JS key -- PDF/A-2b forbids embedded JavaScript", r.num, r.gen)));
    }
    for r in scan_action_subtype(objects, "Launch") {
        out.push(hard(CODE_LAUNCH, format!("object {} {} R is an /S /Launch action -- PDF/A-2b forbids launch actions", r.num, r.gen)));
    }
    if !has_pdfa_output_intent(objects) {
        out.push(soft(CODE_OUTPUT_INTENT, "no OutputIntent with /S /GTS_PDFA1 reachable from /Root/OutputIntents -- real PDF/A files declare one".into()));
    }
    for r in non_embedded_fonts(objects) {
        out.push(soft(CODE_FONT_NOT_EMBEDDED, format!("font object {} {} R has no FontFile/FontFile2/FontFile3 reachable from its FontDescriptor -- PDF/A requires embedded fonts", r.num, r.gen)));
    }
    out
}
//#endregion 🔖️Conformance

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.pdf` (1.7/✳️a-2b): delegates the real parse to the ✳️any subset's analyzer
/// (same `PdfSnapshot`), then folds real PDF/A-2b conformance diagnostics on top. `sniff` also
/// delegates -- a subset-level sniff for a-2b is "is this recognizable as a PDF at all", the same
/// magic-byte probe every 1.7 dialect shares; conformance is a separate, heavier question answered
/// by `analyze`/`check_pdf_a2b_conformance`, not by `sniff`.
pub struct PdfA2bAnalyzer;

impl ArtifactAnalyzer for PdfA2bAnalyzer {
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
            let checks = check_pdf_a2b_conformance(snapshot);
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
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfDictEntry;

    fn output_intent_objects(condition: &str) -> Vec<PdfIndirectObject> {
        vec![
            PdfIndirectObject {
                id: ObjRef { num: 1, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) },
                    PdfDictEntry { key: "OutputIntents".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 2, gen: 0 })]) },
                ]),
            },
            PdfIndirectObject {
                id: ObjRef { num: 2, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("OutputIntent".into()) },
                    PdfDictEntry { key: "S".into(), value: PdfObject::Name("GTS_PDFA1".into()) },
                    PdfDictEntry { key: "OutputConditionIdentifier".into(), value: PdfObject::Str(condition.as_bytes().to_vec()) },
                ]),
            },
        ]
    }

    #[test]
    fn conforming_snapshot_with_output_intent_has_no_diagnostics() {
        let snapshot = PdfSnapshot { objects: output_intent_objects("sRGB IEC61966-2.1"), ..PdfSnapshot::default() };
        let diagnostics = check_pdf_a2b_conformance(&snapshot);
        assert!(diagnostics.is_empty(), "expected zero diagnostics, got {diagnostics:?}");
    }

    #[test]
    fn missing_output_intent_is_soft() {
        let snapshot = PdfSnapshot::default();
        let diagnostics = check_pdf_a2b_conformance(&snapshot);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code.0, CODE_OUTPUT_INTENT);
        assert_eq!(diagnostics[0].severity, Severity::Warning);
    }

    #[test]
    fn encryption_dict_shape_is_hard() {
        let mut objects = output_intent_objects("sRGB IEC61966-2.1");
        objects.push(PdfIndirectObject {
            id: ObjRef { num: 3, gen: 0 },
            value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Filter".into(), value: PdfObject::Name("Standard".into()) },
                PdfDictEntry { key: "V".into(), value: PdfObject::Int(2) },
                PdfDictEntry { key: "R".into(), value: PdfObject::Int(3) },
                PdfDictEntry { key: "O".into(), value: PdfObject::Str(vec![0u8; 32]) },
                PdfDictEntry { key: "U".into(), value: PdfObject::Str(vec![0u8; 32]) },
            ]),
        });
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_pdf_a2b_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ENCRYPT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn javascript_action_is_hard() {
        let mut objects = output_intent_objects("sRGB IEC61966-2.1");
        objects.push(PdfIndirectObject {
            id: ObjRef { num: 3, gen: 0 },
            value: PdfObject::Dict(vec![
                PdfDictEntry { key: "S".into(), value: PdfObject::Name("JavaScript".into()) },
                PdfDictEntry { key: "JS".into(), value: PdfObject::Str(b"app.alert(1)".to_vec()) },
            ]),
        });
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_pdf_a2b_conformance(&snapshot);
        assert_eq!(diagnostics.iter().filter(|d| d.code.0 == CODE_JAVASCRIPT).count(), 1, "must not double-report S=JavaScript + JS key on the same object: got {diagnostics:?}");
        assert_eq!(diagnostics[0].severity, Severity::Error);
    }

    #[test]
    fn launch_action_is_hard() {
        let mut objects = output_intent_objects("sRGB IEC61966-2.1");
        objects.push(PdfIndirectObject {
            id: ObjRef { num: 3, gen: 0 },
            value: PdfObject::Dict(vec![
                PdfDictEntry { key: "S".into(), value: PdfObject::Name("Launch".into()) },
                PdfDictEntry { key: "F".into(), value: PdfObject::Str(b"calc.exe".to_vec()) },
            ]),
        });
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_pdf_a2b_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_LAUNCH && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn non_embedded_font_is_soft() {
        let mut objects = output_intent_objects("sRGB IEC61966-2.1");
        objects.push(PdfIndirectObject {
            id: ObjRef { num: 3, gen: 0 },
            value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Font".into()) },
                PdfDictEntry { key: "Subtype".into(), value: PdfObject::Name("Type1".into()) },
                PdfDictEntry { key: "BaseFont".into(), value: PdfObject::Name("Helvetica".into()) },
            ]),
        });
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_pdf_a2b_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_FONT_NOT_EMBEDDED && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[test]
    fn embedded_font_via_descriptor_has_no_diagnostic() {
        let mut objects = output_intent_objects("sRGB IEC61966-2.1");
        objects.push(PdfIndirectObject {
            id: ObjRef { num: 3, gen: 0 },
            value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Font".into()) },
                PdfDictEntry { key: "Subtype".into(), value: PdfObject::Name("TrueType".into()) },
                PdfDictEntry { key: "FontDescriptor".into(), value: PdfObject::Ref(ObjRef { num: 4, gen: 0 }) },
            ]),
        });
        objects.push(PdfIndirectObject {
            id: ObjRef { num: 4, gen: 0 },
            value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Type".into(), value: PdfObject::Name("FontDescriptor".into()) },
                PdfDictEntry { key: "FontFile2".into(), value: PdfObject::Ref(ObjRef { num: 5, gen: 0 }) },
            ]),
        });
        let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
        let diagnostics = check_pdf_a2b_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_FONT_NOT_EMBEDDED), "got {diagnostics:?}");
    }
}
