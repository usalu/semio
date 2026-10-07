//! 🛡️ Logical Pptx Strict conformance facts and diagnostics.
use crate::PptxSnapshot;
use semio_framework_diagnostic::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_s_artifact_stdio_xml::schema::snapshot::markup_facts::XmlMarkupFacts;
use semio_s_artifact_stdio_zip::opc::OpcPackage;

//#region 🔖️Namespaces
pub const STRICT_MAIN_NS: &str = "http://purl.oclc.org/ooxml/presentationml/main";
pub const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
pub const TRANSITIONAL_DRAWING_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
pub const VML_NS: &str = "urn:schemas-microsoft-com:vml";
pub const STRICT_REL_BASE: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
pub const TRANSITIONAL_REL_BASE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
//#endregion 🔖️Namespaces

//#region 🔖️Conformance
pub const CODE_MAIN_NS: &str = "stdio.pptx.strict.main-ns-not-strict";
pub const CODE_TRANSITIONAL_NS_PRESENT: &str = "stdio.pptx.strict.transitional-ns-present";
pub const CODE_VML_PRESENT: &str = "stdio.pptx.strict.vml-present";
pub const CODE_REL_BASE: &str = "stdio.pptx.strict.relationship-base-not-strict";
pub const CODE_CONFORMANCE_ATTR: &str = "stdio.pptx.strict.conformance-attr-missing";
pub const CODE_ALTERNATE_CONTENT: &str = "stdio.pptx.strict.alternate-content-present";

/// 🧭️ Locates the root officeDocument part regardless of whether the package declares the
/// Transitional or Strict officeDocument relationship type.
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

/// 🛡️ Real ISO/IEC 29500-1:2016 Strict conformance checks against one already-decoded
/// `PptxSnapshot`. Shared single source of truth: `PptxStrictComposer::compose` hard-gates on
/// this (pre-serialization, authoritative), `PptxStrictBuilder::build` hard-gates on this too,
/// and the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn check_strict_conformance(snapshot: &PptxSnapshot) -> Vec<Diagnostic> {
    let opc = &snapshot.opc;
    let mut out = Vec::new();

    match main_part_path(opc) {
        Some(path) => match snapshot.xml_parts.iter().find(|part| part.path == path).map(|part| XmlMarkupFacts::from_document(&part.document)) {
            Some(text) => {
                if !text.declares_namespace(STRICT_MAIN_NS) {
                    out.push(hard(CODE_MAIN_NS, format!("root officeDocument part {path} does not declare the Strict PresentationML main namespace ({STRICT_MAIN_NS})")));
                }
                if !text.root_attribute_is("conformance", "strict") {
                    out.push(soft(CODE_CONFORMANCE_ATTR, format!("root officeDocument part {path}'s <p:presentation> does not declare conformance=\"strict\"")));
                }
            }
            None => out.push(hard(CODE_MAIN_NS, format!("root officeDocument part {path} is missing or not valid utf-8 -- cannot verify the Strict PresentationML main namespace"))),
        },
        None => out.push(hard(CODE_MAIN_NS, "package has no resolvable officeDocument relationship -- cannot verify the Strict PresentationML main namespace".into())),
    }

    for part in &snapshot.xml_parts {
        let path = &part.path;
        let text = XmlMarkupFacts::from_document(&part.document);
        if text.declares_namespace(TRANSITIONAL_MAIN_NS) || text.declares_namespace(TRANSITIONAL_DRAWING_NS) {
            out.push(hard(CODE_TRANSITIONAL_NS_PRESENT, format!("part {path} declares a Transitional OOXML main namespace -- ISO/IEC 29500-1 Strict forbids it")));
        }
        if text.declares_namespace(VML_NS) {
            out.push(hard(CODE_VML_PRESENT, format!("part {path} contains VML markup ({VML_NS}) -- ISO/IEC 29500-1 Strict forbids VML")));
        }
        if text.alternate_content {
            out.push(soft(CODE_ALTERNATE_CONTENT, format!("part {path} contains mc:AlternateContent markup-compatibility escape hatch")));
        }
    }

    for (owner, relationships) in opc.relationships.groups() {
        for rel in relationships {
            if rel.rel_type.starts_with(TRANSITIONAL_REL_BASE) {
                out.push(hard(CODE_REL_BASE, format!("relationship {} owned by '{owner}' uses the Transitional officeDocument relationships base ({}) -- Strict requires {STRICT_REL_BASE}", rel.id, rel.rel_type)));
            }
        }
    }

    out
}
//#endregion 🔖️Conformance
