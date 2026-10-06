//! 🚪️ IO stdio.svg (1.1/🔰️basic) — reuses the ✳️any subset's `xml` import/export leaves rather
//! than duplicating them (same `SvgSnapshot` type, same catalog DAG edge). Registration flows
//! through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator, and the
//! `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io` already
//! established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_1::subsets::base::schema::snapshot::{set_element_attr, SvgSnapshot};
    use crate::standards::v1_1::subsets::base::io::SvgComposer as SvgAnyComposer;
    use crate::standards::v1_1::subsets::basic::schema::check_svg_basic_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_BASIC: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("basic") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    const DEP_XML: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct SvgBasicComposerComposition;

    impl ArtifactComposition for SvgBasicComposerComposition {
        type Snapshot = SvgSnapshot;
        const WRITES: Dialect = DIALECT_BASIC;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_BASIC, DEP_XML]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = SvgAnyComposer::compose(sources)?;
            let mut snapshot = inner.snapshot;
            if let Some(root) = snapshot.doc.root.as_mut() {
                set_element_attr(root, "baseProfile", Some("basic".into()));
                set_element_attr(root, "version", Some("1.1".into()));
            }
            let checks = check_svg_basic_conformance(&snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("SVG Basic 1.1 conformance violated: {} hard issue(s) -- not stamping the basic dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `1.1/basic` -- see the module doc comment for how this
    /// relates to (and honestly differs from) the composer's own pre-serialization hard gate above.
    pub struct SvgBasicValidator;

    impl SubsetValidator for SvgBasicValidator {
        const DIALECT: Dialect = DIALECT_BASIC;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SvgSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SvgSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_svg_basic_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.svg.basic.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "SVG Basic SubsetValidator: payload did not decode as an SvgSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SvgBasicValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the 1.1 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::subsets::base::io::io_registry::entries()`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

pub mod derived_construction {
    use crate::standards::v1_1::subsets::base::schema::snapshot::set_element_attr;
    use crate::standards::v1_1::subsets::basic::schema::check_svg_basic_conformance;
    use crate::standards::v1_1::subsets::basic::schema::mutations::{apply_svg_basic_mutation, SvgBasicMutation};
    use crate::{SvgDiff, SvgSnapshot};
    use semio_framework_diagnostic::Diagnostic;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct SvgBasicBuilderConstruction {
        snapshot: SvgSnapshot,
    }

    impl ArtifactBuilder for SvgBasicBuilderConstruction {
        type Snapshot = SvgSnapshot;
        type Mutation = SvgBasicMutation;
        type Diff = SvgDiff;

        fn empty() -> Self {
            Self::default()
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SvgSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SvgSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_svg_basic_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SvgDiff as protocol::MutationDiff<SvgSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ The real construction gate: injects the profile metadata, then a hard Basic 1.1
        /// violation (however `self.snapshot` got here) fails `build()` -- soft diagnostics (nested
        /// `<svg>`) pass through silently here, matching the `🔬️tiny`/PDF/A pilots' own `build()` shape.
        fn build(mut self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            if let Some(root) = self.snapshot.doc.root.as_mut() {
                set_element_attr(root, "baseProfile", Some("basic".into()));
                set_element_attr(root, "version", Some("1.1".into()));
            }
            let hard: Vec<Diagnostic> = check_svg_basic_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)).collect();
            if hard.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(hard)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v1_1::subsets::base::schema::snapshot::SvgSnapshot;
    use crate::standards::v1_1::subsets::base::io::SvgAnalyzer as SvgAnyAnalyzer;
    pub use crate::standards::v1_1::subsets::base::io::SvgParts;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};
    use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};
    use std::collections::HashMap;

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("basic") };

    //#region 🔖️Vocabulary
    /// 🚫 Expensive raster filter primitives SVG Basic 1.1 excludes (Full 1.1 has them; Basic's
    /// constrained-device target doesn't).
    const BLOCKED_FILTER_PRIMITIVES: &[&str] = &["feConvolveMatrix", "feDisplacementMap", "feTurbulence", "feMorphology", "feDiffuseLighting", "feSpecularLighting", "feDistantLight", "fePointLight", "feSpotLight"];

    const TEXT_ELEMENTS: &[&str] = &["text", "tspan", "tref", "textPath"];

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn local_name(name: &str) -> &str {
        name.rsplit(':').next().unwrap_or(name)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn attr_val<'a>(attrs: &'a [XmlAttr], name: &str) -> Option<&'a str> {
        attrs.iter().find(|a| local_name(&a.name) == name).map(|a| a.value.as_str())
    }

    /// 🔗 Extracts the fragment id from a `clip-path="url(#id)"`-shaped value (bare or quoted) --
    /// `None` for anything else (a non-`url()`/non-fragment value isn't resolvable against this
    /// document, so isn't scanned).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn clip_path_ref_id(value: &str) -> Option<&str> {
        let inner = value.trim().strip_prefix("url(")?.strip_suffix(')')?;
        inner.trim().trim_matches(|c| c == '\'' || c == '"').strip_prefix('#')
    }
    //#endregion 🔖️Vocabulary

    //#region 🔖️Conformance
    pub const CODE_FILTER_PRIMITIVE: &str = "stdio.svg.basic.blocklisted-filter-primitive";
    pub const CODE_CLIP_PATH_TEXT: &str = "stdio.svg.basic.clip-path-text";
    pub const CODE_BASE_PROFILE: &str = "stdio.svg.basic.base-profile";
    pub const CODE_NESTED_SVG: &str = "stdio.svg.basic.nested-svg";

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: semio_framework_diagnostic::FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: semio_framework_diagnostic::FaultScope::default() }
    }

    /// 🌳 Recursively collects every element node's `(name, attrs, children)` triple, depth-first.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn collect_elements<'a>(node: &'a XmlNode, out: &mut Vec<(&'a str, &'a [XmlAttr], &'a [XmlNode])>) {
        if let XmlNode::Element { name, attrs, children } = node {
            out.push((name.as_str(), attrs.as_slice(), children.as_slice()));
            for c in children {
                collect_elements(c, out);
            }
        }
    }

    /// 🔍️ `true` if any (possibly-nested) descendant is one of the SVG text element kinds.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn has_text_descendant(children: &[XmlNode]) -> bool {
        children.iter().any(|c| match c {
            XmlNode::Element { name, children, .. } => TEXT_ELEMENTS.contains(&local_name(name)) || has_text_descendant(children),
            _ => false,
        })
    }

    /// 🗺️ Builds an `id -> children` map for every retained `<clipPath id="...">` element, so a
    /// `clip-path="url(#id)"` reference can resolve to the REAL clipPath's descendants rather than a
    /// guess.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn clip_path_children_by_id<'a>(elements: &[(&'a str, &'a [XmlAttr], &'a [XmlNode])]) -> HashMap<&'a str, &'a [XmlNode]> {
        elements.iter().filter(|(name, ..)| local_name(name) == "clipPath").filter_map(|(_, attrs, children)| attr_val(attrs, "id").map(|id| (id, *children))).collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn find_nested_svg(children: &[XmlNode], out: &mut Vec<String>) {
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    /// 🛡️ Checks SVG Basic's actual referenced clip paths with bounded borrowed traversal.
    pub fn check_svg_basic_conformance_controlled(snapshot:&SvgSnapshot,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<Vec<Diagnostic>,semio_framework_os_kernel::sqlite_snapshot::ValueError>{
        use semio_framework_os_kernel::sqlite_snapshot::{SqliteSnapshotPhase, ValueError, ValueRefusalKind};
        fn tick(control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>,count:&mut usize)->Result<(),ValueError>{*count=count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"SVG Basic validation unit overflow"))?;if *count%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,*count,0)?;}Ok(())}
        let mut out=Vec::new();let mut count=0usize;control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;let Some(root)=&snapshot.doc.root else{return Ok(out)};let mut elements=Vec::new();let mut pending=vec![std::slice::from_ref(root).iter()];while let Some(nodes)=pending.last_mut(){let Some(node)=nodes.next()else{pending.pop();continue;};tick(control,&mut count)?;if let XmlNode::Element{name,attrs,children}=node{for _ in attrs{tick(control,&mut count)?;}elements.push((name.as_str(),attrs.as_slice(),children.as_slice()));pending.push(children.iter());}}
        let clip_paths=clip_path_children_by_id(&elements);
        for(name,attrs,_children)in &elements{tick(control,&mut count)?;if BLOCKED_FILTER_PRIMITIVES.contains(&local_name(name)){out.push(hard(CODE_FILTER_PRIMITIVE,format!("element <{name}> is an expensive raster filter primitive not supported by SVG Basic 1.1")));}if let Some(cp)=attr_val(attrs,"clip-path"){if let Some(id)=clip_path_ref_id(cp){if let Some(children)=clip_paths.get(id){let mut contains_text=false;let mut pending=vec![children.iter()];while let Some(nodes)=pending.last_mut(){let Some(node)=nodes.next()else{pending.pop();continue;};tick(control,&mut count)?;if let XmlNode::Element{name,children,..}=node{if TEXT_ELEMENTS.contains(&local_name(name)){contains_text=true;break;}pending.push(children.iter());}}if contains_text{out.push(hard(CODE_CLIP_PATH_TEXT,format!("<{name}> clip-path=\"{cp}\" references clipPath #{id}, which contains a text descendant -- SVG Basic 1.1 forbids clipping to text")));}}}}}
        for(name,..)in elements.iter().skip(1){tick(control,&mut count)?;if local_name(name)=="svg"{out.push(soft(CODE_NESTED_SVG,format!("nested <{name}> element found below the document root -- review its viewport/clipping behavior on constrained renderers")));}}
        if let XmlNode::Element{name,attrs,..}=root{let base_profile_ok=attrs.iter().any(|a|a.name=="baseProfile"&&a.value=="basic");let version_ok=attrs.iter().any(|a|a.name=="version"&&a.value=="1.1");if !base_profile_ok||!version_ok{out.push(soft(CODE_BASE_PROFILE,format!("root <{name}> is missing baseProfile=\"basic\"/version=\"1.1\" -- SVG Basic 1.1 documents should declare their profile")));}}
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,count)?;Ok(out)
    }

    pub fn check_svg_basic_conformance(snapshot: &SvgSnapshot) -> Vec<Diagnostic> {
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
    /// 🧐️ Analyzes `stdio.svg` (1.1/🔰️basic): delegates the real parse to the ✳️any subset's analyzer
    /// (same `SvgSnapshot`), then folds real SVG Basic 1.1 conformance diagnostics on top. `sniff`
    /// delegates too -- see `SvgTinyAnalyzer`'s doc comment, same rationale.
    pub struct SvgBasicAnalyzerAnalysis;

    impl ArtifactAnalysis for SvgBasicAnalyzerAnalysis {
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
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;

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
