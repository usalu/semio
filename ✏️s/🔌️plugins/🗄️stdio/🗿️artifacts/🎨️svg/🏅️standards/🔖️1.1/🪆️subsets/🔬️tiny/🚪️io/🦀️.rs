//! 🚪️ IO stdio.svg (1.1/🔬️tiny) — reuses the ✳️any subset's `xml` import/export leaves rather
//! than duplicating them (same `SvgSnapshot` type, same catalog DAG edge). Registration flows
//! through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator, and the
//! `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io` already
//! established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_1::subsets::base::schema::snapshot::{set_element_attr, SvgSnapshot};
    use crate::standards::v1_1::subsets::base::io::SvgComposer as SvgAnyComposer;
    use crate::standards::v1_1::subsets::tiny::schema::check_svg_tiny_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_TINY: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("tiny") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    const DEP_XML: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct SvgTinyComposerComposition;

    impl ArtifactComposition for SvgTinyComposerComposition {
        type Snapshot = SvgSnapshot;
        const WRITES: Dialect = DIALECT_TINY;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_TINY, DEP_XML]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = SvgAnyComposer::compose(sources)?;
            let mut snapshot = inner.snapshot;
            if let Some(root) = snapshot.doc.root.as_mut() {
                set_element_attr(root, "baseProfile", Some("tiny".into()));
                set_element_attr(root, "version", Some("1.1".into()));
            }
            let checks = check_svg_tiny_conformance(&snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("SVG Tiny 1.1 conformance violated: {} hard issue(s) -- not stamping the tiny dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `1.1/tiny` -- see the module doc comment for how this
    /// relates to (and honestly differs from) the composer's own pre-serialization hard gate above.
    pub struct SvgTinyValidator;

    impl SubsetValidator for SvgTinyValidator {
        const DIALECT: Dialect = DIALECT_TINY;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SvgSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SvgSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_svg_tiny_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.svg.tiny.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "SVG Tiny SubsetValidator: payload did not decode as an SvgSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SvgTinyValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the 1.1 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::subsets::base::io::io_registry::entries()`), matching how `✳️any`'s own
    /// entry is registered.
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
    use crate::standards::v1_1::subsets::tiny::schema::check_svg_tiny_conformance;
    use crate::standards::v1_1::subsets::tiny::schema::mutations::{apply_svg_tiny_mutation, SvgTinyMutation};
    use crate::{SvgDiff, SvgSnapshot};
    use semio_framework_diagnostic::Diagnostic;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct SvgTinyBuilderConstruction {
        snapshot: SvgSnapshot,
    }

    impl ArtifactBuilder for SvgTinyBuilderConstruction {
        type Snapshot = SvgSnapshot;
        type Mutation = SvgTinyMutation;
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
            let diff = apply_svg_tiny_mutation(&mut self.snapshot, &mutation);
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
            let hard: Vec<Diagnostic> = check_svg_tiny_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)).collect();
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn local_name(name: &str) -> &str {
        name.rsplit(':').next().unwrap_or(name)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_blocked_element(name: &str) -> bool {
        let ln = local_name(name);
        BLOCKED_ELEMENTS.contains(&ln) || ln.starts_with("fe")
    }

    /// 🌐️ `true` for a value that looks like a reference to an external document (a URI scheme or a
    /// scheme-relative `//host/...`), `false` for a same-document fragment (`#id`) or a bare relative
    /// path.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: semio_framework_diagnostic::FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: semio_framework_diagnostic::FaultScope::default() }
    }

    /// 🌳 Recursively walks one element (and its descendants), reporting blocklisted elements,
    /// blocklisted attributes, and external `href`/`xlink:href` values.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    /// 🛡️ Applies SVG Tiny's existing borrowed rules with iterative cancellation checkpoints.
    pub fn check_svg_tiny_conformance_controlled(snapshot:&SvgSnapshot,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<Vec<Diagnostic>,semio_framework_os_kernel::sqlite_snapshot::ValueError>{
        use semio_framework_os_kernel::sqlite_snapshot::{SqliteSnapshotPhase, ValueError, ValueRefusalKind};
        let mut out=Vec::new();let mut count=0usize;control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;let Some(root)=&snapshot.doc.root else{return Ok(out)};
        let mut pending=vec![std::slice::from_ref(root).iter()];while let Some(nodes)=pending.last_mut(){let Some(node)=nodes.next()else{pending.pop();continue;};count=count.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "SVG validation count overflow"))?;if count%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,0)?;}
            if let XmlNode::Element{name,attrs,children}=node{if is_blocked_element(name){out.push(hard(CODE_ELEMENT,format!("element <{name}> is outside SVG Tiny 1.1's vocabulary -- REC-SVGMobile-20030114 excludes it")));}for a in attrs{count=count.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "SVG validation count overflow"))?;if count%256==0||a.value.len()>65536||a.name.len()>65536{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,0)?;}let ln=local_name(&a.name);if BLOCKED_ATTRS.contains(&ln){out.push(hard(CODE_ATTRIBUTE,format!("attribute '{}' on <{name}> is forbidden anywhere in SVG Tiny 1.1",a.name)));}if ln=="href"&&is_external_href(&a.value){out.push(soft(CODE_EXTERNAL_HREF,format!("<{name}> {}=\"{}\" looks like an external document reference -- SVG Tiny 1.1 restricts references to the same document",a.name,a.value)));}}pending.push(children.iter());}
        }
        if let XmlNode::Element{name,..}=root{let attrs=root_attrs(root);let base_profile_ok=attrs.iter().any(|a|a.name=="baseProfile"&&a.value=="tiny");let version_ok=attrs.iter().any(|a|a.name=="version"&&a.value=="1.1");if !base_profile_ok||!version_ok{out.push(soft(CODE_BASE_PROFILE,format!("root <{name}> is missing baseProfile=\"tiny\"/version=\"1.1\" -- SVG Tiny 1.1 documents should declare their profile")));}}
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,count)?;Ok(out)
    }

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
    /// 🧐️ Analyzes `stdio.svg` (1.1/🔬️tiny): delegates the real parse to the ✳️any subset's analyzer
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
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;

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
