//! 🧐️ SvgTinyAnalyzer (1.1/✳️tiny) — real W3C Mobile SVG Profiles (REC-SVGMobile-20030114) §SVG
//! Tiny 1.1 conformance checks against the retained `SvgSnapshot.doc`, a full, lossless
//! `XmlDocument` tree (element name + attrs + children, no typed-model schema gap — see the
//! roster's `sourceOfTruth`). Walks the raw XML tree recursively rather than the typed
//! `SvgElement` model, since "which elements/attributes are legal" is fundamentally a wire-
//! vocabulary question, independent of this artifact's typed shape model.
//!
//! Checks implemented as real, honest scans over the retained tree:
//! - HARD: any element from SVG Tiny 1.1's excluded vocabulary -- CSS styling (`style`/`script`),
//!   reusable-graphics constructs Tiny drops (`symbol`/`marker`), clipping/masking/patterning
//!   (`clipPath`/`mask`/`pattern`), gradients (`linearGradient`/`radialGradient`/`stop` -- Tiny
//!   1.1 has NO gradient support at all), filters (`filter` + every `fe*` primitive), `cursor`,
//!   and the text extensions `textPath`/`tspan`/`tref`/`view`.
//! - HARD: any element (root included) carrying one of the presentation attributes Tiny 1.1
//!   forbids outright (`style`/`opacity`/`fill-opacity`/`stroke-opacity`/`clip-path`/`mask`/
//!   `filter`) -- illegal regardless of which element bears it, unlike the per-name element
//!   blocklist above.
//! - SOFT: `<svg>` root missing `baseProfile="tiny"`/`version="1.1"`.
//! - SOFT: an `href`/`xlink:href` value that looks like a reference to an EXTERNAL document (a
//!   URI scheme, e.g. `http://...`, or a scheme-relative `//...`) rather than a same-document
//!   fragment (`#id`) or a bare relative path -- Tiny 1.1 restricts `<use>`/image references to a
//!   narrow scope; distinguishing genuinely-forbidden external refs from legitimately-external
//!   ones needs runtime context this static analyzer doesn't have, so this is advisory only.

use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::svg::standards::v1_1::subsets::any::analyzer::SvgAnalyzer as SvgAnyAnalyzer;
pub use crate::artifacts::svg::standards::v1_1::subsets::any::analyzer::SvgParts;
use crate::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::SvgSnapshot;
use crate::artifacts::xml::schema::snapshot::{XmlAttr, XmlNode};

/// 🎯️ This subset's dialect coordinate.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("tiny") };

//#region 🔖️Vocabulary
/// 🚫 Elements SVG Tiny 1.1 excludes outright (Full 1.1 features Tiny doesn't retain). `fe*`
/// filter primitives are matched separately by prefix (there are too many to enumerate, and
/// Tiny 1.1 forbids the whole `filter` mechanism, primitives included).
const BLOCKED_ELEMENTS: &[&str] = &[
    "style", "script", "symbol", "marker", "clipPath", "mask", "pattern", "linearGradient", "radialGradient", "stop", "filter", "cursor", "textPath",
    "tspan", "tref", "view",
];

/// 🚫 Presentation attributes SVG Tiny 1.1 forbids on ANY element.
const BLOCKED_ATTRS: &[&str] = &["style", "opacity", "fill-opacity", "stroke-opacity", "clip-path", "mask", "filter"];

/// ✂️ Strips an XML namespace prefix (`xlink:href` -> `href`) for vocabulary-matching purposes
/// only -- diagnostics still report the original, fully-qualified name.
fn local_name(name: &str) -> &str {
    name.rsplit(':').next().unwrap_or(name)
}

fn is_blocked_element(name: &str) -> bool {
    let ln = local_name(name);
    BLOCKED_ELEMENTS.contains(&ln) || ln.starts_with("fe")
}

/// 🌐️ `true` for a value that looks like a reference to an external document (a URI scheme or a
/// scheme-relative `//host/...`), `false` for a same-document fragment (`#id`) or a bare relative
/// path.
fn is_external_href(value: &str) -> bool {
    let v = value.trim();
    !v.starts_with('#') && (v.contains("://") || v.starts_with("//"))
}
//#endregion 🔖️Vocabulary

//#region 🔖️Conformance
pub const CODE_ELEMENT: &str = "stdio.svg.tiny.blocklisted-element";
pub const CODE_ATTRIBUTE: &str = "stdio.svg.tiny.blocklisted-attribute";
pub const CODE_BASE_PROFILE: &str = "stdio.svg.tiny.base-profile";
pub const CODE_EXTERNAL_HREF: &str = "stdio.svg.tiny.external-href";

fn hard(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: dsl::FaultScope::default() }
}

fn soft(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: dsl::FaultScope::default() }
}

/// 🌳 Recursively walks one element (and its descendants), reporting blocklisted elements,
/// blocklisted attributes, and external `href`/`xlink:href` values.
fn walk(node: &XmlNode, out: &mut Vec<Diagnostic>) {
    if let XmlNode::Element { name, attrs, children } = node {
        if is_blocked_element(name) {
            out.push(hard(CODE_ELEMENT, format!("element <{name}> is outside SVG Tiny 1.1's vocabulary -- REC-SVGMobile-20030114 excludes it")));
        }
        for a in attrs {
            let ln = local_name(&a.name);
            if BLOCKED_ATTRS.contains(&ln) {
                out.push(hard(CODE_ATTRIBUTE, format!("attribute '{}' on <{name}> is forbidden anywhere in SVG Tiny 1.1", a.name)));
            }
            if ln == "href" && is_external_href(&a.value) {
                out.push(soft(CODE_EXTERNAL_HREF, format!("<{name}> {}=\"{}\" looks like an external document reference -- SVG Tiny 1.1 restricts references to the same document", a.name, a.value)));
            }
        }
        for c in children {
            walk(c, out);
        }
    }
}

fn root_attrs(root: &XmlNode) -> &[XmlAttr] {
    match root {
        XmlNode::Element { attrs, .. } => attrs.as_slice(),
        _ => &[],
    }
}

/// 🛡️ Real SVG Tiny 1.1 conformance checks against one already-decoded `SvgSnapshot`. Shared
/// single source of truth: `SvgTinyComposer::compose` hard-gates on this (pre-serialization,
/// authoritative), `SvgTinyBuilder::build` hard-gates on this too, and the registered
/// `SubsetValidator` (`🎹️composer::register`) re-runs it post-hoc against the wire payload for
/// the D5 validate-on-build hook.
pub fn check_svg_tiny_conformance(snapshot: &SvgSnapshot) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    let Some(root) = &snapshot.doc.root else { return out };
    walk(root, &mut out);
    if let XmlNode::Element { name, .. } = root {
        let attrs = root_attrs(root);
        let base_profile_ok = attrs.iter().any(|a| a.name == "baseProfile" && a.value == "tiny");
        let version_ok = attrs.iter().any(|a| a.name == "version" && a.value == "1.1");
        if !base_profile_ok || !version_ok {
            out.push(soft(CODE_BASE_PROFILE, format!("root <{name}> is missing baseProfile=\"tiny\"/version=\"1.1\" -- SVG Tiny 1.1 documents should declare their profile")));
        }
    }
    out
}
//#endregion 🔖️Conformance

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.svg` (1.1/✳️tiny): delegates the real parse to the ✳️any subset's analyzer
/// (same `SvgSnapshot`), then folds real SVG Tiny 1.1 conformance diagnostics on top. `sniff`
/// delegates too -- a subset-level sniff for `tiny` is "is this recognizable as an SVG document at
/// all", the same root-element probe every 1.1 dialect shares; conformance is a separate, heavier
/// question answered by `analyze`/`check_svg_tiny_conformance`, not by `sniff`.
pub struct SvgTinyAnalyzer;

impl ArtifactAnalyzer for SvgTinyAnalyzer {
    type Parts = SvgParts;
    const DIALECT: Dialect = DIALECT;

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        SvgAnyAnalyzer::sniff(source)
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let inner = SvgAnyAnalyzer::analyze(sources);
        let mut diagnostics = inner.diagnostics.clone();
        let mut confidence = inner.confidence;
        if let Some(snapshot) = &inner.parts.snapshot {
            let checks = check_svg_tiny_conformance(snapshot);
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

    fn svg_root(attrs: Vec<XmlAttr>, children: Vec<XmlNode>) -> SvgSnapshot {
        let mut snapshot = SvgSnapshot::default();
        snapshot.doc.root = Some(XmlNode::Element { name: "svg".into(), attrs, children });
        snapshot
    }

    fn attr(name: &str, value: &str) -> XmlAttr {
        XmlAttr { name: name.into(), value: value.into() }
    }

    fn elem(name: &str, attrs: Vec<XmlAttr>) -> XmlNode {
        XmlNode::Element { name: name.into(), attrs, children: vec![] }
    }

    #[test]
    fn fully_conforming_document_reports_no_diagnostics() {
        let snapshot = svg_root(
            vec![attr("baseProfile", "tiny"), attr("version", "1.1")],
            vec![elem("rect", vec![attr("x", "0"), attr("y", "0"), attr("width", "10"), attr("height", "10")])],
        );
        let diagnostics = check_svg_tiny_conformance(&snapshot);
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    #[test]
    fn blocklisted_element_is_hard() {
        let snapshot = svg_root(vec![attr("baseProfile", "tiny"), attr("version", "1.1")], vec![elem("linearGradient", vec![])]);
        let diagnostics = check_svg_tiny_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ELEMENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn filter_primitive_by_prefix_is_hard() {
        let snapshot = svg_root(vec![attr("baseProfile", "tiny"), attr("version", "1.1")], vec![elem("feGaussianBlur", vec![])]);
        let diagnostics = check_svg_tiny_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ELEMENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn blocklisted_attribute_is_hard() {
        let snapshot = svg_root(vec![attr("baseProfile", "tiny"), attr("version", "1.1")], vec![elem("rect", vec![attr("opacity", "0.5")])]);
        let diagnostics = check_svg_tiny_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ATTRIBUTE && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn missing_base_profile_is_soft() {
        let snapshot = svg_root(vec![], vec![]);
        let diagnostics = check_svg_tiny_conformance(&snapshot);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code.0, CODE_BASE_PROFILE);
        assert_eq!(diagnostics[0].severity, Severity::Warning);
    }

    #[test]
    fn external_href_is_soft() {
        let snapshot = svg_root(vec![attr("baseProfile", "tiny"), attr("version", "1.1")], vec![elem("use", vec![attr("xlink:href", "http://example.com/sprite.svg#icon")])]);
        let diagnostics = check_svg_tiny_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_EXTERNAL_HREF && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[test]
    fn same_document_fragment_href_is_clean() {
        let snapshot = svg_root(vec![attr("baseProfile", "tiny"), attr("version", "1.1")], vec![elem("use", vec![attr("href", "#icon")])]);
        let diagnostics = check_svg_tiny_conformance(&snapshot);
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }
}
