//! 🛡️ Logical Pptx Transitional conformance facts and diagnostics.
use crate::PptxSnapshot;
use semio_framework_diagnostic::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_s_artifact_stdio_xml::schema::snapshot::markup_facts::XmlMarkupFacts;
use semio_s_artifact_stdio_zip::opc::OpcPackage;

//#region 🔖️Namespaces
pub const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
/// 🏅️ Marker prefix common to EVERY ISO/IEC 29500-1 Strict namespace URI (markup namespaces
/// AND the officeDocument relationships base alike) -- see `🪆️subsets/🔒️strict`'s
/// `STRICT_MAIN_NS`/`STRICT_REL_BASE`, both of which start with this prefix.
pub const STRICT_NS_MARKER: &str = "purl.oclc.org/ooxml";
//#endregion 🔖️Namespaces

//#region 🔖️Conformance
pub const CODE_MAIN_NS: &str = "stdio.pptx.transitional.main-ns-not-transitional";
pub const CODE_STRICT_NS_PRESENT: &str = "stdio.pptx.transitional.strict-ns-present";
pub const CODE_CONFORMANCE_ATTR: &str = "stdio.pptx.transitional.conformance-attr-not-transitional";

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn main_part_path(opc: &OpcPackage) -> Option<String> {
    crate::standards::v_ecma_376::subsets::base::schema::vocabulary::resolve_office_document_relationship(opc)
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn hard(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn soft(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

/// 🛡️ Real ISO/IEC 29500-4:2016 Transitional conformance checks against one already-decoded
/// `PptxSnapshot`. Shared single source of truth: `PptxTransitionalComposer::compose` hard-gates
/// on this (pre-serialization, authoritative), `PptxTransitionalBuilder::build` hard-gates on
/// this too, and the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn check_transitional_conformance(snapshot: &PptxSnapshot) -> Vec<Diagnostic> {
    let opc = &snapshot.opc;
    let mut out = Vec::new();

    match main_part_path(opc) {
        Some(path) => match snapshot.xml_parts.iter().find(|part| part.path == path).map(|part| XmlMarkupFacts::from_document(&part.document)) {
            Some(text) => {
                if !text.declares_namespace(TRANSITIONAL_MAIN_NS) {
                    out.push(hard(CODE_MAIN_NS, format!("root officeDocument part {path} does not declare the Transitional PresentationML main namespace ({TRANSITIONAL_MAIN_NS})")));
                }
                if text.root_attribute_is("conformance", "strict") {
                    out.push(soft(CODE_CONFORMANCE_ATTR, format!("root officeDocument part {path}'s <p:presentation> declares conformance=\"strict\" -- Transitional expects it absent or \"transitional\"")));
                }
            }
            None => out.push(hard(CODE_MAIN_NS, format!("root officeDocument part {path} is missing or not valid utf-8 -- cannot verify the Transitional PresentationML main namespace"))),
        },
        None => out.push(hard(CODE_MAIN_NS, "package has no resolvable officeDocument relationship -- cannot verify the Transitional PresentationML main namespace".into())),
    }

    for part in &snapshot.xml_parts {
        let path = &part.path;
        let text = XmlMarkupFacts::from_document(&part.document);
        if text.namespaces.iter().any(|namespace| namespace.contains(STRICT_NS_MARKER)) {
            out.push(hard(CODE_STRICT_NS_PRESENT, format!("part {path} declares an ISO/IEC 29500-1 Strict namespace -- ISO/IEC 29500-4 Transitional forbids it")));
        }
    }

    for (owner, relationships) in opc.relationships.groups() {
        for rel in relationships {
            if rel.rel_type.contains(STRICT_NS_MARKER) {
                out.push(hard(CODE_STRICT_NS_PRESENT, format!("relationship {} owned by '{owner}' uses a Strict relationship base ({}) -- Transitional forbids it", rel.id, rel.rel_type)));
            }
        }
    }

    out
}
//#endregion 🔖️Conformance
