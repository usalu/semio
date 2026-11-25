//! 🛡️ Logical SpreadsheetML subset conformance diagnostics.

use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::vocabulary::{workbook_namespaces as base_workbook_namespaces, WorkbookNamespaces};
use semio_framework_diagnostic::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;

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

/// 🔎️ Real scan of `xl/workbook.xml`: the namespace of its root element, the namespace of every relationship-id attribute resolved through the bindings in scope where it is used (a prefix may be declared on any element, not only the root) and its `conformance` attribute. `None` only when the part is missing or no workbook element; `Some(Err)` when a prefix is undeclared.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn workbook_namespaces(snapshot: &XlsxSnapshot) -> Option<Result<WorkbookNamespaces, String>> {
    let path = snapshot.workbook_part_path()?;
    let root = snapshot.xml_part(&path)?.document.root.as_ref()?;
    let XmlNode::Element { name, .. } = root else { return None };
    if name.rsplit_once(':').map_or(name.as_str(), |(_, local)| local) != "workbook" {
        return None;
    }
    Some(base_workbook_namespaces(root))
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn hard(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn soft(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

/// 🩺️ Real worksheet content-type scan over every part the workbook's worksheet relationships target (its role),
/// reading the package-declared type. Small enough (and CODE_* consts stay subset-namespaced) that duplicating beats a
/// cross-subset dependency on 🌉️transitional's own copy.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn worksheet_content_type_gaps(snapshot: &XlsxSnapshot) -> Vec<Diagnostic> {
    snapshot
        .worksheet_part_paths()
        .into_iter()
        .filter_map(|path| {
            let content_type = snapshot.opc.content_types.resolve(&path).map(str::to_string);
            (content_type.as_deref() != Some(WORKSHEET_CONTENT_TYPE))
                .then(|| soft(CODE_WORKSHEET_CONTENT_TYPE, format!("worksheet part {path} resolves content type {content_type:?}, expected {WORKSHEET_CONTENT_TYPE:?} (ECMA-376 Part 1 §12.3.24)")))
        })
        .collect()
}

/// 🛡️ Real ISO/IEC 29500-1 (Strict) conformance checks against one already-decoded
/// `XlsxSnapshot`. Shared single source of truth: `XlsxStrictComposer::compose` hard-gates on
/// this (pre-serialization, authoritative), `XlsxStrictBuilder::build` hard-gates on this too, and
/// the registered `SubsetValidator` (see `🎹️composer::register`) re-runs it post-hoc against the
/// wire payload for the D5 validate-on-build hook.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn check_strict_conformance(snapshot: &XlsxSnapshot) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    let Some(namespaces) = workbook_namespaces(snapshot) else {
        out.push(hard(CODE_NAMESPACE_MISMATCH, format!("{WORKBOOK_PART} is missing or unparsable as XML -- cannot verify ISO/IEC 29500-1 Strict conformance")));
        return out;
    };
    let namespaces = match namespaces {
        Ok(namespaces) => namespaces,
        Err(detail) => {
            out.push(hard(CODE_RELATIONSHIPS_NAMESPACE_MISMATCH, format!("{WORKBOOK_PART} uses an undeclared namespace prefix: {detail}")));
            return out;
        }
    };
    let conformance = namespaces.conformance;
    if namespaces.main != STRICT_SML_NS {
        out.push(hard(CODE_NAMESPACE_MISMATCH, format!("{WORKBOOK_PART} root namespace is {:?}, expected the Strict SpreadsheetML namespace {STRICT_SML_NS:?} (ISO/IEC 29500-1)", namespaces.main)));
    }
    for namespace in namespaces.relationship_ids.iter().filter(|namespace| namespace.as_str() != STRICT_R_NS) {
        out.push(hard(CODE_RELATIONSHIPS_NAMESPACE_MISMATCH, format!("{WORKBOOK_PART} relationship id attributes use the namespace {namespace:?}, expected the Strict officeDocument relationships namespace {STRICT_R_NS:?}")));
    }
    if conformance.as_deref() != Some("strict") {
        out.push(hard(CODE_CONFORMANCE_ATTRIBUTE, format!("{WORKBOOK_PART} workbook@conformance is {conformance:?}, expected \"strict\" (ISO/IEC 29500-1 §12.3.24)")));
    }
    let lanes = snapshot.opc.parts.iter().map(|part| (&part.path, &part.content_type)).chain(snapshot.xml_parts.iter().map(|part| (&part.path, &part.content_type)));
    for (path, _) in lanes.filter(|(_, content_type)| content_type.as_str() == VML_CONTENT_TYPE) {
        out.push(hard(CODE_VML_FORBIDDEN, format!("part {path} declares legacy VML drawing content type {VML_CONTENT_TYPE:?} -- ISO/IEC 29500-1 Strict removes VML support entirely")));
    }
    out.extend(worksheet_content_type_gaps(snapshot));
    out
}
//#endregion 🔖️Conformance


#[cfg(test)]
include!("../../🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
