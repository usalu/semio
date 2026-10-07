//! 🛡️ Logical Docx Transitional conformance facts and diagnostics.
use crate::DocxSnapshot;
use semio_framework_diagnostic::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_s_artifact_stdio_xml::schema::snapshot::markup_facts::XmlMarkupFacts;
use crate::schema::snapshot::DocxXmlPart;
use semio_s_artifact_stdio_zip::opc::resolve_relationship_target;

//#region 🔖️Namespaces
pub const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
/// 🔎️ Every ISO/IEC 29500-1 Strict namespace URI (markup AND relationship) shares this prefix --
/// see `📏️strict`'s `STRICT_MAIN_NS`/`STRICT_REL_BASE`, both of which start with it.
pub const STRICT_NS_FAMILY_PREFIX: &str = "purl.oclc.org/ooxml";
//#endregion 🔖️Namespaces

//#region 🔖️Conformance
pub const CODE_MAIN_NS_MISSING: &str = "stdio.docx.transitional.main-ns-missing";
pub const CODE_STRICT_NS_PRESENT: &str = "stdio.docx.transitional.strict-ns-present";
pub const CODE_CONFORMANCE_ATTR: &str = "stdio.docx.transitional.conformance-attr-invalid";

/// 🔎️ Resolves the main document part via the root officeDocument relationship -- matched by
/// relationship-type SUFFIX (`/officeDocument`) so this resolves for either conformance class; see
/// `📏️strict::analyzer::main_document_part`'s doc comment for the full rationale.
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

/// 🛡️ Real ISO/IEC 29500-4:2016 Transitional conformance checks against one already-decoded
/// `DocxSnapshot`. Shared single source of truth: `DocxTransitionalComposer::compose` hard-gates
/// on this (pre-serialization, authoritative), `DocxTransitionalBuilder::build` hard-gates on this
/// too, and the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn check_transitional_conformance(snapshot: &DocxSnapshot) -> Vec<Diagnostic> {
    let opc = &snapshot.opc;
    let mut out = Vec::new();

    match main_document_part(snapshot) {
        Some((part, path)) => {
            if !part_facts(part).declares_namespace(TRANSITIONAL_MAIN_NS) {
                out.push(hard(CODE_MAIN_NS_MISSING, format!("main document part {path} does not declare the transitional WordprocessingML namespace {TRANSITIONAL_MAIN_NS}")));
            }
            if part_facts(part).root_attribute_is("conformance", "strict") {
                out.push(soft(CODE_CONFORMANCE_ATTR, format!("main document part {path} root element declares conformance=\"strict\" -- transitional documents must leave it absent or =\"transitional\"")));
            }
        }
        None => out.push(hard(CODE_MAIN_NS_MISSING, "package has no root officeDocument relationship -- cannot locate the main document part to check the transitional namespace on".into())),
    }

    for part in &snapshot.xml_parts {
        if part_facts(part).namespaces.iter().any(|namespace| namespace.contains(STRICT_NS_FAMILY_PREFIX)) {
            out.push(hard(CODE_STRICT_NS_PRESENT, format!("part {} contains a strict-family namespace ({STRICT_NS_FAMILY_PREFIX}) -- transitional conformance forbids mixed namespaces", part.path)));
        }
    }

    let mut owners: Vec<&String> = opc.relationships.keys().collect();
    owners.sort();
    for owner in owners {
        for rel in opc.relationships.get(owner).expect("enumerated retained relationship owner").iter() {
            if rel.rel_type.to_string_owner().contains(STRICT_NS_FAMILY_PREFIX) {
                out.push(hard(CODE_STRICT_NS_PRESENT, format!("relationship {} owned by {owner:?} uses a strict-family relationship base ({STRICT_NS_FAMILY_PREFIX}) -- transitional conformance forbids it", rel.id)));
            }
        }
    }

    out
}
//#endregion 🔖️Conformance
