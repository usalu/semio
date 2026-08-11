//! 🧐️ XmlValidAnalyzer (1.0/✳️valid) — real, honestly-scoped W3C XML 1.0 Fifth Edition §5.1
//! ("Validity Constraints") checks against the retained `XmlSnapshot.doc` (`doctype` is kept as a
//! raw, unparsed `String` -- see the schema's own doc comment -- so this subset can only check
//! what that raw string genuinely lets it tell apart, never fabricated DTD content-model
//! validation).
//!
//! Checks implemented as real, honest scans:
//! - HARD (blocks the `valid` dialect stamp): `doc.doctype` must be present at all -- a document
//!   with no DOCTYPE declaration cannot be "valid" per §5.1 (only well-formed), regardless of how
//!   clean its content otherwise is.
//! - HARD: when a doctype IS present, its declared root name (the first token after `<!DOCTYPE`)
//!   must equal the actual root element's tag name -- §2.8's `doctypedecl` production ties the
//!   DOCTYPE's `Name` to the document's root element by construction; a mismatch is never valid
//!   XML regardless of DTD content.
//! - SOFT: `standalone="yes"` in the XML declaration while the doctype references an external
//!   subset (`SYSTEM`/`PUBLIC`) -- suspicious per §2.9 (a standalone document declaring an external
//!   subset it then ignores is a common authoring mistake, not a hard validity violation on its
//!   own since the external subset's actual markup declarations aren't retained to check against).
//! - SOFT, ALWAYS-FIRES: an advisory naming the exact DTD-content-model gap this schema has --
//!   `doctype` is retained as a raw unparsed `String` (no internal/external subset markup
//!   declarations are parsed), so element/attribute-list validation against the DTD is out of
//!   scope; this is the "honest schema gap" protocol's mandatory diagnostic.

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::xml::standards::v1_0::subsets::any::analyzer::XmlAnalyzer as XmlAnyAnalyzer;
pub use crate::artifacts::xml::standards::v1_0::subsets::any::analyzer::XmlParts;
use crate::artifacts::xml::standards::v1_0::subsets::any::schema::snapshot::{XmlNode, XmlSnapshot};

/// 🎯️ This subset's dialect coordinate.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("valid") };

//#region 🔖️Conformance
pub const CODE_DOCTYPE_MISSING: &str = "stdio.xml.valid.doctype-missing";
pub const CODE_ROOT_NAME_MISMATCH: &str = "stdio.xml.valid.root-name-mismatch";
pub const CODE_STANDALONE_EXTERNAL_SUBSET: &str = "stdio.xml.valid.standalone-external-subset";
pub const CODE_VALIDITY_NOT_VERIFIED: &str = "stdio.xml.valid.validity-not-fully-verified";

/// 🔍️ Parses the declared root `Name` out of a raw `<!DOCTYPE ...>` string -- the first name
/// token after the (case-insensitively matched) `<!DOCTYPE` keyword, per §2.8's `doctypedecl`
/// production (`'<!DOCTYPE' S Name (S ExternalID)? S? ('[' intSubset ']' S?)? '>'`).
fn parse_doctype_root_name(doctype: &str) -> Option<&str> {
    let lower = doctype.to_ascii_lowercase();
    if !lower.starts_with("<!doctype") {
        return None;
    }
    let rest = doctype["<!doctype".len()..].trim_start();
    let end = rest.find(|c: char| c.is_whitespace() || c == '[' || c == '>').unwrap_or(rest.len());
    let name = &rest[..end];
    if name.is_empty() { None } else { Some(name) }
}

/// 🌳️ The actual root element's tag name, if a root element is present at all.
fn root_element_name(snapshot: &XmlSnapshot) -> Option<&str> {
    match &snapshot.doc.root {
        Some(XmlNode::Element { name, .. }) => Some(name.as_str()),
        _ => None,
    }
}

/// 🔗️ Real, honest scan: does the raw doctype string reference an external subset -- a
/// `SYSTEM`/`PUBLIC` external ID per §4.2.2 -- without parsing its content?
fn doctype_references_external_subset(doctype: &str) -> bool {
    doctype.contains("SYSTEM") || doctype.contains("PUBLIC")
}

fn hard(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

fn soft(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

/// 🛡️ Real, scope-limited W3C XML 1.0 Fifth Edition §5.1 validity checks against one
/// already-decoded `XmlSnapshot`. Shared single source of truth: `XmlValidComposer::compose`
/// hard-gates on this (pre-serialization, authoritative), `XmlValidBuilder::build` hard-gates on
/// this too, and the registered `SubsetValidator` re-runs it post-hoc against the wire payload for
/// the D5 validate-on-build hook.
pub fn check_valid_conformance(snapshot: &XmlSnapshot) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    match &snapshot.doc.doctype {
        None => {
            out.push(hard(CODE_DOCTYPE_MISSING, "no <!DOCTYPE ...> declaration present -- XML 1.0 §5.1 validity requires one (a document without one can be well-formed at best)".into()));
        }
        Some(doctype) => {
            if let Some(declared_root) = parse_doctype_root_name(doctype) {
                if let Some(actual_root) = root_element_name(snapshot) {
                    if declared_root != actual_root {
                        out.push(hard(
                            CODE_ROOT_NAME_MISMATCH,
                            format!("doctype declares root name '{declared_root}' but the actual root element is '<{actual_root}>' -- §2.8 requires the DOCTYPE Name to match the document element"),
                        ));
                    }
                }
            }
            if doctype_references_external_subset(doctype) {
                if snapshot.doc.declaration.as_ref().and_then(|d| d.standalone) == Some(true) {
                    out.push(soft(
                        CODE_STANDALONE_EXTERNAL_SUBSET,
                        "XML declaration says standalone=\"yes\" but the doctype references an external subset (SYSTEM/PUBLIC) -- suspicious per §2.9".into(),
                    ));
                }
            }
        }
    }
    out.push(soft(
        CODE_VALIDITY_NOT_VERIFIED,
        "validity not fully verified: this schema retains <!DOCTYPE ...> as a raw unparsed String with no internal/external subset markup declarations parsed, so full DTD element/attribute-list content-model validation (§3.2/§3.3) is out of scope from this data -- only the presence of a doctype and its declared-root-name/actual-root-name agreement are checked".into(),
    ));
    out
}
//#endregion 🔖️Conformance

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.xml` (1.0/✳️valid): delegates the real parse to the ✳️any subset's
/// analyzer (same `XmlSnapshot`), then folds real XML validity conformance diagnostics on top.
pub struct XmlValidAnalyzer;

impl ArtifactAnalyzer for XmlValidAnalyzer {
    type Parts = XmlParts;
    const DIALECT: Dialect = DIALECT;

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        XmlAnyAnalyzer::sniff(source)
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let inner = XmlAnyAnalyzer::analyze(sources);
        let mut diagnostics = inner.diagnostics.clone();
        let mut confidence = inner.confidence;
        if let Some(snapshot) = &inner.parts.snapshot {
            let checks = check_valid_conformance(snapshot);
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
    use crate::artifacts::xml::standards::v1_0::subsets::any::schema::snapshot::{XmlDeclaration, XmlDocument};

    fn snapshot_with(doctype: Option<&str>, standalone: Option<bool>, root_name: &str) -> XmlSnapshot {
        XmlSnapshot {
            doc: XmlDocument {
                declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: None, standalone }),
                doctype: doctype.map(|s| s.to_string()),
                root: Some(XmlNode::Element { name: root_name.into(), attrs: Vec::new(), children: Vec::new() }),
            },
            ..XmlSnapshot::default()
        }
    }

    #[test]
    fn conforming_doctype_reports_only_the_always_on_advisory() {
        let snapshot = snapshot_with(Some("<!DOCTYPE html>"), None, "html");
        let diagnostics = check_valid_conformance(&snapshot);
        assert_eq!(diagnostics.len(), 1, "got {diagnostics:?}");
        assert_eq!(diagnostics[0].code.0, CODE_VALIDITY_NOT_VERIFIED);
        assert_eq!(diagnostics[0].severity, Severity::Warning);
    }

    #[test]
    fn missing_doctype_is_hard() {
        let snapshot = snapshot_with(None, None, "html");
        let diagnostics = check_valid_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DOCTYPE_MISSING && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn root_name_mismatch_is_hard() {
        let snapshot = snapshot_with(Some("<!DOCTYPE book>"), None, "html");
        let diagnostics = check_valid_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ROOT_NAME_MISMATCH && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn standalone_yes_with_external_subset_is_soft() {
        let snapshot = snapshot_with(Some("<!DOCTYPE html SYSTEM \"http://example.com/html.dtd\">"), Some(true), "html");
        let diagnostics = check_valid_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STANDALONE_EXTERNAL_SUBSET && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[test]
    fn standalone_yes_without_external_subset_is_clean() {
        let snapshot = snapshot_with(Some("<!DOCTYPE html>"), Some(true), "html");
        let diagnostics = check_valid_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.code.0 != CODE_STANDALONE_EXTERNAL_SUBSET), "got {diagnostics:?}");
    }

    #[test]
    fn public_external_subset_reference_is_detected() {
        let snapshot = snapshot_with(Some("<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd\">"), Some(true), "html");
        let diagnostics = check_valid_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STANDALONE_EXTERNAL_SUBSET), "got {diagnostics:?}");
    }
}
