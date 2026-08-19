//! 🧬️ SvgSnapshot schema (1.1/✳️basic) — reuses the ✳️any subset's `SvgSnapshot` verbatim (the
//! SAME Rust type, same `s.stdio.svg` schema id). SVG Basic 1.1 (W3C Mobile SVG Profiles,
//! REC-SVGMobile-20030114 §SVG Basic 1.1) is a validation-gated dialect STAMP on top of that
//! existing schema, not a new one -- D4's Tier-1 "same snapshot type, subset moves" semantics
//! (`ArtifactCommand::MigrateDialect`). This leaf exists so `🪆️subsets/✳️basic/🧬️schema/` is
//! present per `🔣️taxonomy.json`'s `subsetChildDirs`, without duplicating the schema definition.

pub use crate::artifacts::svg::standards::v1_1::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::set_element_attr;
    use crate::artifacts::svg::standards::v1_1::subsets::basic::schema::check_svg_basic_conformance;
    use crate::artifacts::svg::{SvgDiff, SvgMutation, SvgSnapshot};
    use dsl::Diagnostic;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct SvgBasicBuilderConstruction {
        snapshot: SvgSnapshot,
    }

    impl ArtifactBuilder for SvgBasicBuilderConstruction {
        type Snapshot = SvgSnapshot;
        type Mutation = SvgMutation;
        type Diff = SvgDiff;

        async fn empty() -> Self {
            Self::default()
        }

        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SvgSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SvgSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::svg::schema::mutations::apply_svg_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SvgDiff as protocol::MutationDiff<SvgSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ The real construction gate: injects the profile metadata, then a hard Basic 1.1
        /// violation (however `self.snapshot` got here) fails `build()` -- soft diagnostics (nested
        /// `<svg>`) pass through silently here, matching the `✳️tiny`/PDF/A pilots' own `build()` shape.
        async fn build(mut self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            if let Some(root) = self.snapshot.doc.root.as_mut() {
                set_element_attr(root, "baseProfile", Some("basic".into()));
                set_element_attr(root, "version", Some("1.1".into()));
            }
            let hard: Vec<Diagnostic> = check_svg_basic_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, dsl::Severity::Error | dsl::Severity::Fatal)).collect();
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
        use crate::artifacts::svg::standards::v1_1::subsets::basic::schema::CODE_FILTER_PRIMITIVE;
        use crate::artifacts::xml::schema::snapshot::{XmlAttr, XmlNode};

        #[test]
        async fn empty_builder_injects_profile_and_builds_clean() {
            let snapshot = SvgBasicBuilderConstruction::empty().build().expect("empty document builds clean");
            match &snapshot.doc.root {
                Some(XmlNode::Element { attrs, .. }) => {
                    assert!(attrs.iter().any(|a| a.name == "baseProfile" && a.value == "basic"));
                    assert!(attrs.iter().any(|a| a.name == "version" && a.value == "1.1"));
                }
                other => panic!("expected element root, got {other:?}"),
            }
        }

        #[test]
        async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
            let mut snapshot = SvgBasicBuilderConstruction::empty().build().unwrap();
            if let Some(XmlNode::Element { children, .. }) = snapshot.doc.root.as_mut() {
                children.push(XmlNode::Element { name: "filter".into(), attrs: vec![XmlAttr { name: "id".into(), value: "f1".into() }], children: vec![XmlNode::Element { name: "feMorphology".into(), attrs: vec![], children: vec![] }] });
            }
            let (mutated, _diff) = SvgBasicBuilderConstruction::from_snapshot(SvgSnapshot::default()).mutate(SvgMutation::SetSnapshot { snapshot });
            let err = mutated.build().expect_err("a feMorphology primitive must fail build()");
            assert!(err.iter().any(|d| d.code.0 == CODE_FILTER_PRIMITIVE));
        }

        #[test]
        async fn from_text_round_trips_through_basic_build() {
            let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><circle cx="5" cy="5" r="5"/></svg>"#;
            let built = SvgBasicBuilderConstruction::from_text(text).expect("parses").build().expect("conforming document builds");
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
    use std::collections::HashMap;

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("basic") };

    //#region 🔖️Vocabulary
    /// 🚫 Expensive raster filter primitives SVG Basic 1.1 excludes (Full 1.1 has them; Basic's
    /// constrained-device target doesn't).
    const BLOCKED_FILTER_PRIMITIVES: &[&str] = &["feConvolveMatrix", "feDisplacementMap", "feTurbulence", "feMorphology", "feDiffuseLighting", "feSpecularLighting", "feDistantLight", "fePointLight", "feSpotLight"];

    const TEXT_ELEMENTS: &[&str] = &["text", "tspan", "tref", "textPath"];

    async fn local_name(name: &str) -> &str {
        name.rsplit(':').next().unwrap_or(name)
    }

    async fn attr_val<'a>(attrs: &'a [XmlAttr], name: &str) -> Option<&'a str> {
        attrs.iter().find(|a| local_name(&a.name) == name).map(|a| a.value.as_str())
    }

    /// 🔗 Extracts the fragment id from a `clip-path="url(#id)"`-shaped value (bare or quoted) --
    /// `None` for anything else (a non-`url()`/non-fragment value isn't resolvable against this
    /// document, so isn't scanned).
    async fn clip_path_ref_id(value: &str) -> Option<&str> {
        let inner = value.trim().strip_prefix("url(")?.strip_suffix(')')?;
        inner.trim().trim_matches(|c| c == '\'' || c == '"').strip_prefix('#')
    }
    //#endregion 🔖️Vocabulary

    //#region 🔖️Conformance
    pub const CODE_FILTER_PRIMITIVE: &str = "stdio.svg.basic.blocklisted-filter-primitive";
    pub const CODE_CLIP_PATH_TEXT: &str = "stdio.svg.basic.clip-path-text";
    pub const CODE_BASE_PROFILE: &str = "stdio.svg.basic.base-profile";
    pub const CODE_NESTED_SVG: &str = "stdio.svg.basic.nested-svg";

    async fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: dsl::FaultScope::default() }
    }

    async fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: dsl::FaultScope::default() }
    }

    /// 🌳 Recursively collects every element node's `(name, attrs, children)` triple, depth-first.
    async fn collect_elements<'a>(node: &'a XmlNode, out: &mut Vec<(&'a str, &'a [XmlAttr], &'a [XmlNode])>) {
        if let XmlNode::Element { name, attrs, children } = node {
            out.push((name.as_str(), attrs.as_slice(), children.as_slice()));
            for c in children {
                collect_elements(c, out);
            }
        }
    }

    /// 🔍️ `true` if any (possibly-nested) descendant is one of the SVG text element kinds.
    async fn has_text_descendant(children: &[XmlNode]) -> bool {
        children.iter().any(|c| match c {
            XmlNode::Element { name, children, .. } => TEXT_ELEMENTS.contains(&local_name(name)) || has_text_descendant(children),
            _ => false,
        })
    }

    /// 🗺️ Builds an `id -> children` map for every retained `<clipPath id="...">` element, so a
    /// `clip-path="url(#id)"` reference can resolve to the REAL clipPath's descendants rather than a
    /// guess.
    async fn clip_path_children_by_id<'a>(elements: &[(&'a str, &'a [XmlAttr], &'a [XmlNode])]) -> HashMap<&'a str, &'a [XmlNode]> {
        elements.iter().filter(|(name, ..)| local_name(name) == "clipPath").filter_map(|(_, attrs, children)| attr_val(attrs, "id").map(|id| (id, *children))).collect()
    }

    async fn find_nested_svg(children: &[XmlNode], out: &mut Vec<String>) {
        for n in children {
            if let XmlNode::Element { name, children, .. } = n {
                if local_name(name) == "svg" {
                    out.push(name.clone());
                }
                find_nested_svg(children, out);
            }
        }
    }

    /// 🛡️ Real SVG Basic 1.1 conformance checks against one already-decoded `SvgSnapshot`. Shared
    /// single source of truth: `SvgBasicComposer::compose` hard-gates on this (pre-serialization,
    /// authoritative), `SvgBasicBuilder::build` hard-gates on this too, and the registered
    /// `SubsetValidator` (`🎹️composer::register`) re-runs it post-hoc against the wire payload for
    /// the D5 validate-on-build hook.
    pub async fn check_svg_basic_conformance(snapshot: &SvgSnapshot) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        let Some(root) = &snapshot.doc.root else { return out };

        let mut elements = Vec::new();
        collect_elements(root, &mut elements);
        let clip_paths = clip_path_children_by_id(&elements);

        for (name, attrs, _children) in &elements {
            if BLOCKED_FILTER_PRIMITIVES.contains(&local_name(name)) {
                out.push(hard(CODE_FILTER_PRIMITIVE, format!("element <{name}> is an expensive raster filter primitive not supported by SVG Basic 1.1")));
            }
            if let Some(cp) = attr_val(attrs, "clip-path") {
                if let Some(id) = clip_path_ref_id(cp) {
                    if let Some(cp_children) = clip_paths.get(id) {
                        if has_text_descendant(cp_children) {
                            out.push(hard(CODE_CLIP_PATH_TEXT, format!("<{name}> clip-path=\"{cp}\" references clipPath #{id}, which contains a text descendant -- SVG Basic 1.1 forbids clipping to text")));
                        }
                    }
                }
            }
        }

        if let XmlNode::Element { children, .. } = root {
            let mut nested = Vec::new();
            find_nested_svg(children, &mut nested);
            for name in nested {
                out.push(soft(CODE_NESTED_SVG, format!("nested <{name}> element found below the document root -- review its viewport/clipping behavior on constrained renderers")));
            }
        }

        if let XmlNode::Element { name, attrs, .. } = root {
            let base_profile_ok = attrs.iter().any(|a| a.name == "baseProfile" && a.value == "basic");
            let version_ok = attrs.iter().any(|a| a.name == "version" && a.value == "1.1");
            if !base_profile_ok || !version_ok {
                out.push(soft(CODE_BASE_PROFILE, format!("root <{name}> is missing baseProfile=\"basic\"/version=\"1.1\" -- SVG Basic 1.1 documents should declare their profile")));
            }
        }
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.svg` (1.1/✳️basic): delegates the real parse to the ✳️any subset's analyzer
    /// (same `SvgSnapshot`), then folds real SVG Basic 1.1 conformance diagnostics on top. `sniff`
    /// delegates too -- see `SvgTinyAnalyzer`'s doc comment, same rationale.
    pub struct SvgBasicAnalyzerAnalysis;

    impl ArtifactAnalysis for SvgBasicAnalyzerAnalysis {
        type Parts = SvgParts;
        const DIALECT: Dialect = DIALECT;

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            SvgAnyAnalyzer::sniff(source)
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = SvgAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_svg_basic_conformance(snapshot);
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

        async fn svg_root(attrs: Vec<XmlAttr>, children: Vec<XmlNode>) -> SvgSnapshot {
            let mut snapshot = SvgSnapshot::default();
            snapshot.doc.root = Some(XmlNode::Element { name: "svg".into(), attrs, children });
            snapshot
        }

        async fn attr(name: &str, value: &str) -> XmlAttr {
            XmlAttr { name: name.into(), value: value.into() }
        }

        async fn elem(name: &str, attrs: Vec<XmlAttr>, children: Vec<XmlNode>) -> XmlNode {
            XmlNode::Element { name: name.into(), attrs, children }
        }

        async fn base_attrs() -> Vec<XmlAttr> {
            vec![attr("baseProfile", "basic"), attr("version", "1.1")]
        }

        #[test]
        async fn fully_conforming_document_reports_no_diagnostics() {
            let snapshot = svg_root(base_attrs(), vec![elem("rect", vec![attr("x", "0"), attr("y", "0"), attr("width", "10"), attr("height", "10")], vec![])]);
            let diagnostics = check_svg_basic_conformance(&snapshot);
            assert!(diagnostics.is_empty(), "got {diagnostics:?}");
        }

        #[test]
        async fn blocklisted_filter_primitive_is_hard() {
            let snapshot = svg_root(base_attrs(), vec![elem("filter", vec![attr("id", "f1")], vec![elem("feTurbulence", vec![], vec![])])]);
            let diagnostics = check_svg_basic_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_FILTER_PRIMITIVE && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        async fn lighting_child_primitive_is_hard() {
            let snapshot = svg_root(base_attrs(), vec![elem("filter", vec![attr("id", "f1")], vec![elem("feDiffuseLighting", vec![], vec![elem("feDistantLight", vec![], vec![])])])]);
            let diagnostics = check_svg_basic_conformance(&snapshot);
            assert_eq!(diagnostics.iter().filter(|d| d.code.0 == CODE_FILTER_PRIMITIVE).count(), 2, "expected both feDiffuseLighting and feDistantLight flagged: {diagnostics:?}");
        }

        #[test]
        async fn clip_path_referencing_text_is_hard() {
            let snapshot = svg_root(base_attrs(), vec![elem("clipPath", vec![attr("id", "c1")], vec![elem("text", vec![], vec![XmlNode::Text { text: "hi".into() }])]), elem("rect", vec![attr("clip-path", "url(#c1)")], vec![])]);
            let diagnostics = check_svg_basic_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CLIP_PATH_TEXT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        async fn clip_path_referencing_shapes_only_is_clean() {
            let snapshot = svg_root(base_attrs(), vec![elem("clipPath", vec![attr("id", "c1")], vec![elem("circle", vec![attr("cx", "5"), attr("cy", "5"), attr("r", "5")], vec![])]), elem("rect", vec![attr("clip-path", "url(#c1)")], vec![])]);
            let diagnostics = check_svg_basic_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_CLIP_PATH_TEXT), "got {diagnostics:?}");
        }

        #[test]
        async fn nested_svg_is_soft() {
            let snapshot = svg_root(base_attrs(), vec![elem("svg", vec![attr("width", "5"), attr("height", "5")], vec![])]);
            let diagnostics = check_svg_basic_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_NESTED_SVG && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[test]
        async fn missing_base_profile_is_soft() {
            let snapshot = svg_root(vec![], vec![]);
            let diagnostics = check_svg_basic_conformance(&snapshot);
            assert_eq!(diagnostics.len(), 1);
            assert_eq!(diagnostics[0].code.0, CODE_BASE_PROFILE);
            assert_eq!(diagnostics[0].severity, Severity::Warning);
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec SvgBasicBuilderFacets {
        construction: SvgBasicBuilderConstruction,
        analysis: SvgBasicAnalyzerAnalysis,
        composition: super::io::derived_composition::SvgBasicComposerComposition,
    }
    builder: SvgBasicBuilder,
    analyzer: SvgBasicAnalyzer,
    composer: SvgBasicComposer,
);
//#endregion 🧬️DerivedArtifactFacets
