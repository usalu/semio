//! 🛡️ Logical Docx Strict conformance facts and diagnostics.
use crate::DocxSnapshot;
use semio_framework_diagnostic::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_s_artifact_stdio_xml::schema::snapshot::markup_facts::XmlMarkupFacts;
use crate::schema::snapshot::DocxXmlPart;
use semio_s_artifact_stdio_zip::opc::resolve_relationship_target;

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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn main_document_part(snapshot: &DocxSnapshot) -> Option<(&DocxXmlPart, String)> {
    let rel = snapshot.opc.relationships_for("")?.iter().find(|relationship| relationship.rel_type.to_string_owner().ends_with("/officeDocument"))?;
    let path = resolve_relationship_target("", &rel.target.to_string_owner());
    snapshot.xml_part(&path).map(|part| (part, path))
}

fn part_facts(part: &DocxXmlPart) -> XmlMarkupFacts {
    XmlMarkupFacts::from_retained_document(&part.document)
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
/// `DocxSnapshot`. Shared single source of truth: `DocxStrictComposer::compose` hard-gates on
/// this (pre-serialization, authoritative), `DocxStrictBuilder::build` hard-gates on this too, and
/// the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn check_strict_conformance(snapshot: &DocxSnapshot) -> Vec<Diagnostic> {
    let opc = &snapshot.opc;
    let mut out = Vec::new();

    match main_document_part(snapshot) {
        Some((part, path)) => {
            if !part_facts(part).declares_namespace(STRICT_MAIN_NS) {
                out.push(hard(CODE_MAIN_NS_MISSING, format!("main document part {path} does not declare the strict WordprocessingML namespace {STRICT_MAIN_NS}")));
            }
            if !part_facts(part).root_attribute_is("conformance", "strict") {
                out.push(soft(CODE_CONFORMANCE_ATTR, format!("main document part {path} root element does not declare conformance=\"strict\"")));
            }
        }
        None => out.push(hard(CODE_MAIN_NS_MISSING, "package has no root officeDocument relationship -- cannot locate the main document part to check the strict namespace on".into())),
    }

    for part in &snapshot.xml_parts {
        if part_facts(part).declares_namespace(TRANSITIONAL_MAIN_NS) {
            out.push(hard(CODE_TRANSITIONAL_NS_PRESENT, format!("part {} contains the transitional WordprocessingML namespace {TRANSITIONAL_MAIN_NS} -- strict conformance forbids mixed namespaces", part.path)));
        }
        if part_facts(part).declares_namespace(VML_NS) {
            out.push(hard(CODE_VML_PRESENT, format!("part {} contains the VML namespace {VML_NS} -- VML is transitional-only markup, forbidden under strict conformance", part.path)));
        }
        if part_facts(part).alternate_content {
            out.push(soft(CODE_ALTERNATE_CONTENT, format!("part {} contains mc:AlternateContent compatibility markup", part.path)));
        }
    }

    let mut owners: Vec<_> = opc.relationships.keys().collect();
    owners.sort();
    for owner in owners {
        for rel in opc.relationships.get(owner).expect("enumerated retained relationship owner").iter() {
            if rel.rel_type.to_string_owner().starts_with(TRANSITIONAL_REL_BASE) {
                out.push(hard(CODE_REL_BASE, format!("relationship {} owned by {owner:?} uses the transitional relationship base {TRANSITIONAL_REL_BASE} -- strict conformance requires {STRICT_REL_BASE}", rel.id)));
            }
        }
    }

    out
}
//#endregion 🔖️Conformance
