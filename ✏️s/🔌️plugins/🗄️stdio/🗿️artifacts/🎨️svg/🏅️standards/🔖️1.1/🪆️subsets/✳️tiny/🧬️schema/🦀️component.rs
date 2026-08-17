//! 🧬️ SvgSnapshot schema (1.1/✳️tiny) — reuses the ✳️any subset's `SvgSnapshot` verbatim (the
//! SAME Rust type, same `s.stdio.svg` schema id). SVG Tiny 1.1 (W3C Mobile SVG Profiles,
//! REC-SVGMobile-20030114 §SVG Tiny 1.1) is a validation-gated dialect STAMP on top of that
//! existing schema, not a new one -- D4's Tier-1 "same snapshot type, subset moves" semantics
//! (`ArtifactCommand::MigrateDialect`). This leaf exists so `🪆️subsets/✳️tiny/🧬️schema/` is
//! present per `🔣️taxonomy.json`'s `subsetChildDirs`, without duplicating the schema definition.

pub use crate::artifacts::svg::standards::v1_1::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::set_element_attr;
    use crate::artifacts::svg::standards::v1_1::subsets::tiny::schema::check_svg_tiny_conformance;
    use crate::artifacts::svg::{SvgDiff, SvgMutation, SvgSnapshot};
    use dsl::Diagnostic;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct SvgTinyBuilderConstruction {
        snapshot: SvgSnapshot,
    }

    impl ArtifactBuilder for SvgTinyBuilderConstruction {
        type Snapshot = SvgSnapshot;
        type Mutation = SvgMutation;
        type Diff = SvgDiff;

        fn empty() -> Self {
            Self::default()
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SvgSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SvgSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::svg::schema::mutations::apply_svg_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SvgDiff as protocol::MutationDiff<SvgSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ The real construction gate: injects the profile metadata, then a hard Tiny 1.1
        /// violation (however `self.snapshot` got here) fails `build()` -- soft diagnostics (external
        /// `href`) pass through silently here since `ArtifactBuilder::build`'s `Err` path only ever
        /// carries the hard set, matching the PDF/A pilot's own `build()` shape.
        fn build(mut self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            if let Some(root) = self.snapshot.doc.root.as_mut() {
                set_element_attr(root, "baseProfile", Some("tiny".into()));
                set_element_attr(root, "version", Some("1.1".into()));
            }
            let hard: Vec<Diagnostic> = check_svg_tiny_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, dsl::Severity::Error | dsl::Severity::Fatal)).collect();
            if hard.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(hard)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::svg::standards::v1_1::subsets::tiny::schema::CODE_ELEMENT;
        use crate::artifacts::xml::schema::snapshot::XmlNode;

        #[test]
        fn empty_builder_injects_profile_and_builds_clean() {
            let snapshot = SvgTinyBuilderConstruction::empty().build().expect("empty document builds clean");
            match &snapshot.doc.root {
                Some(XmlNode::Element { attrs, .. }) => {
                    assert!(attrs.iter().any(|a| a.name == "baseProfile" && a.value == "tiny"));
                    assert!(attrs.iter().any(|a| a.name == "version" && a.value == "1.1"));
                }
                other => panic!("expected element root, got {other:?}"),
            }
        }

        #[test]
        fn hard_violation_injected_via_raw_mutate_still_fails_build() {
            let mut snapshot = SvgTinyBuilderConstruction::empty().build().unwrap();
            if let Some(XmlNode::Element { children, .. }) = snapshot.doc.root.as_mut() {
                children.push(XmlNode::Element { name: "script".into(), attrs: vec![], children: vec![XmlNode::Text { text: "alert(1)".into() }] });
            }
            let (mutated, _diff) = SvgTinyBuilderConstruction::from_snapshot(SvgSnapshot::default()).mutate(SvgMutation::SetSnapshot { snapshot });
            let err = mutated.build().expect_err("a <script> element must fail build()");
            assert!(err.iter().any(|d| d.code.0 == CODE_ELEMENT));
        }

        #[test]
        fn from_text_round_trips_through_tiny_build() {
            let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><circle cx="5" cy="5" r="5"/></svg>"#;
            let built = SvgTinyBuilderConstruction::from_text(text).expect("parses").build().expect("conforming document builds");
            assert!(matches!(built.doc.root, Some(XmlNode::Element { .. })));
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::SvgSnapshot;
    use crate::artifacts::svg::standards::v1_1::subsets::any::schema::SvgAnalyzer as SvgAnyAnalyzer;
    pub use crate::artifacts::svg::standards::v1_1::subsets::any::schema::SvgParts;
    use crate::artifacts::xml::schema::snapshot::{XmlAttr, XmlNode};
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("tiny") };

    //#region 🔖️Vocabulary
    /// 🚫 Elements SVG Tiny 1.1 excludes outright (Full 1.1 features Tiny doesn't retain). `fe*`
    /// filter primitives are matched separately by prefix (there are too many to enumerate, and
    /// Tiny 1.1 forbids the whole `filter` mechanism, primitives included).
    const BLOCKED_ELEMENTS: &[&str] = &["style", "script", "symbol", "marker", "clipPath", "mask", "pattern", "linearGradient", "radialGradient", "stop", "filter", "cursor", "textPath", "tspan", "tref", "view"];

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
    pub struct SvgTinyAnalyzerAnalysis;

    impl ArtifactAnalysis for SvgTinyAnalyzerAnalysis {
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
            let snapshot = svg_root(vec![attr("baseProfile", "tiny"), attr("version", "1.1")], vec![elem("rect", vec![attr("x", "0"), attr("y", "0"), attr("width", "10"), attr("height", "10")])]);
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
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec SvgTinyBuilderFacets {
        construction: SvgTinyBuilderConstruction,
        analysis: SvgTinyAnalyzerAnalysis,
        composition: super::io::derived_composition::SvgTinyComposerComposition,
    }
    builder: SvgTinyBuilder,
    analyzer: SvgTinyAnalyzer,
    composer: SvgTinyComposer,
);
//#endregion 🧬️DerivedArtifactFacets
