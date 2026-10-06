//! 🚪️ IO stdio.xlsx (ecma-376/🌉️transitional) — reuses the 🧱️base subset's `zip`/`xml` raw-codec
//! DAG leaves rather than duplicating them (same `XlsxSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! `🧱️base/🚪️io` and `🔒️strict/🚪️io` already established for this artifact family.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
    use crate::standards::v_ecma_376::subsets::base::io::XlsxComposer as XlsxAnyComposer;
    use crate::standards::v_ecma_376::subsets::transitional::schema::check_transitional_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_TRANSITIONAL: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct XlsxTransitionalComposerComposition;

    impl ArtifactComposition for XlsxTransitionalComposerComposition {
        type Snapshot = XlsxSnapshot;
        const WRITES: Dialect = DIALECT_TRANSITIONAL;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_TRANSITIONAL, DEP_ZIP]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = XlsxAnyComposer::compose(sources)?;
            let checks = check_transitional_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("ISO/IEC 29500-4 Transitional conformance violated: {} hard issue(s) -- not stamping the transitional dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct XlsxTransitionalValidator;

    impl SubsetValidator for XlsxTransitionalValidator {
        const DIALECT: Dialect = DIALECT_TRANSITIONAL;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <XlsxSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_transitional_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.xlsx.transitional.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "Xlsx Transitional SubsetValidator: payload did not decode as an XlsxSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<XlsxTransitionalValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry. Called from the
    /// ecma-376 standard's own `⚙️engine::register()`. The `ComposerEntry` itself is registered
    /// separately by the standard-level composer aggregator
    /// (`crate::standards::v_ecma_376::engine::io_registry::entries()`).
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
    #[cfg(test)]
    use crate::standards::v_ecma_376::subsets::base::schema::mutations::set_snapshot;
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxSnapshot, XlsxWorkbook};
    use crate::standards::v_ecma_376::subsets::transitional::schema::{check_transitional_conformance, TRANSITIONAL_R_NS, TRANSITIONAL_SML_NS};
    use crate::{XlsxDiff, XlsxMutation};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::Severity;
    use semio_framework_plugin::ArtifactBuilder;
    use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};

    //#region 🔖️Stamp
    /// 🖋️ Real-rewrites the main workbook XML part's root attrs to explicit Transitional shape.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn stamp_transitional_namespace(mut snapshot: XlsxSnapshot) -> XlsxSnapshot {
        let main_path = snapshot.workbook_part_path();
        if let Some(part) = main_path.as_deref().and_then(|path| snapshot.xml_part_mut(path)) {
            if let Some(XmlNode::Element { attrs, .. }) = &mut part.document.root {
                set_attr(attrs, "xmlns", TRANSITIONAL_SML_NS);
                set_attr(attrs, "xmlns:r", TRANSITIONAL_R_NS);
                set_attr(attrs, "conformance", "transitional");
            }
        }
        snapshot
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn set_attr(attrs: &mut Vec<XmlAttr>, name: &str, value: &str) {
        if let Some(existing) = attrs.iter_mut().find(|a| a.name == name) {
            existing.value = value.into();
        } else {
            attrs.push(XmlAttr { name: name.into(), value: value.into() });
        }
    }
    //#endregion 🔖️Stamp

    //#region 🔖️Builder
    #[derive(Clone, Debug)]
    pub struct XlsxTransitionalBuilderConstruction {
        snapshot: XlsxSnapshot,
    }

    impl XlsxTransitionalBuilderConstruction {
        /// ➕️ The recommended entry point: builds a minimal package from `workbook` via the shared
        /// ecma-376 engine, then stamps it explicitly Transitional.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new(workbook: XlsxWorkbook) -> Self {
            Self { snapshot: stamp_transitional_namespace(crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(workbook)) }
        }
    }

    impl ArtifactBuilder for XlsxTransitionalBuilderConstruction {
        type Snapshot = XlsxSnapshot;
        type Mutation = XlsxMutation;
        type Diff = XlsxDiff;

        fn empty() -> Self {
            Self::new(XlsxWorkbook::default())
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<XlsxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::standards::v_ecma_376::subsets::base::schema::mutations::apply_xlsx_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <XlsxDiff as protocol::MutationDiff<XlsxSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ The real construction gate: however `self.snapshot` got here, a hard Transitional
        /// violation (Strict-shaped namespace/conformance attribute, or an unparsable workbook.xml)
        /// fails `build()`; the soft diagnostic passes through as advisory.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let hard: Vec<Diagnostic> = check_transitional_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
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
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
    use crate::standards::v_ecma_376::subsets::base::io::XlsxAnalyzer as XlsxAnyAnalyzer;
    pub use crate::standards::v_ecma_376::subsets::base::io::XlsxParts;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};
    use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };

    //#region 🔖️Conformance
    pub const CODE_NAMESPACE_MISMATCH: &str = "stdio.xlsx.transitional.namespace-mismatch";
    pub const CODE_RELATIONSHIPS_NAMESPACE_MISMATCH: &str = "stdio.xlsx.transitional.relationships-namespace-mismatch";
    pub const CODE_CONFORMANCE_ATTRIBUTE: &str = "stdio.xlsx.transitional.conformance-attribute";
    pub const CODE_WORKSHEET_CONTENT_TYPE: &str = "stdio.xlsx.transitional.worksheet-content-type-missing";

    /// 🏷️ ISO/IEC 29500-4 Transitional SpreadsheetML main namespace (same value the shared
    /// `⚙️engine`'s private `SML_NS` uses -- duplicated here as a `pub` constant since the engine's
    /// copy isn't exported).
    pub const TRANSITIONAL_SML_NS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
    /// 🔗️ ISO/IEC 29500-4 Transitional officeDocument relationships (markup) namespace.
    pub const TRANSITIONAL_R_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    const WORKSHEET_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
    const WORKBOOK_PART: &str = "xl/workbook.xml";

    /// 🔎️ Real scan of `xl/workbook.xml`'s root element attrs -- `(xmlns, xmlns:r, conformance)`,
    /// each `None` when absent. `None` overall only when the part is missing or unparsable as XML.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn workbook_root_attrs(snapshot: &XlsxSnapshot) -> Option<(Option<String>, Option<String>, Option<String>)> {
        let path = snapshot.workbook_part_path()?;
        let XmlNode::Element { name, attrs, .. } = snapshot.xml_part(&path)?.document.root.as_ref()? else { return None };
        if name.rsplit_once(':').map_or(name.as_str(), |(_, local)| local) != "workbook" {
            return None;
        }
        let get = |n: &str| attrs.iter().find(|a| a.name == n).map(|a| a.value.clone());
        Some((get("xmlns"), get("xmlns:r"), get("conformance")))
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
    /// reading the package-declared type -- same check as 🔒️strict's own copy, duplicated (small enough, CODE_* consts
    /// stay subset-namespaced) rather than a cross-subset dependency.
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

    /// 🛡️ Real ISO/IEC 29500-4 (Transitional) conformance checks against one already-decoded
    /// `XlsxSnapshot`. Same single-source-of-truth role as 🔒️strict's `check_strict_conformance`:
    /// `XlsxTransitionalComposer::compose` and `XlsxTransitionalBuilder::build` hard-gate on this, and
    /// the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_transitional_conformance(snapshot: &XlsxSnapshot) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        let Some((xmlns, xmlns_r, conformance)) = workbook_root_attrs(snapshot) else {
            out.push(hard(CODE_NAMESPACE_MISMATCH, format!("{WORKBOOK_PART} is missing or unparsable as XML -- cannot verify ISO/IEC 29500-4 Transitional conformance")));
            return out;
        };
        if xmlns.as_deref() != Some(TRANSITIONAL_SML_NS) {
            out.push(hard(CODE_NAMESPACE_MISMATCH, format!("{WORKBOOK_PART} root xmlns is {xmlns:?}, expected the Transitional SpreadsheetML namespace {TRANSITIONAL_SML_NS:?} (ISO/IEC 29500-4)")));
        }
        if xmlns_r.as_deref() != Some(TRANSITIONAL_R_NS) {
            out.push(hard(CODE_RELATIONSHIPS_NAMESPACE_MISMATCH, format!("{WORKBOOK_PART} root xmlns:r is {xmlns_r:?}, expected the Transitional officeDocument relationships namespace {TRANSITIONAL_R_NS:?}")));
        }
        if conformance.as_deref() == Some("strict") {
            out.push(hard(CODE_CONFORMANCE_ATTRIBUTE, format!("{WORKBOOK_PART} workbook@conformance is \"strict\" -- a document that declares Strict conformance cannot be honestly stamped Transitional")));
        }
        out.extend(worksheet_content_type_gaps(snapshot));
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.xlsx` (ecma-376/🌉️transitional): delegates the real parse to the 🧱️base
    /// subset's analyzer (same `XlsxSnapshot`), then folds real Transitional conformance diagnostics
    /// on top. `sniff` also delegates -- same rationale as 🔒️strict's analyzer.
    pub struct XlsxTransitionalAnalyzerAnalysis;

    impl ArtifactAnalysis for XlsxTransitionalAnalyzerAnalysis {
        type Parts = XlsxParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            XlsxAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = XlsxAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_transitional_conformance(snapshot);
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
    pub spec XlsxTransitionalBuilderFacets {
        construction: XlsxTransitionalBuilderConstruction,
        analysis: XlsxTransitionalAnalyzerAnalysis,
        composition: super::io::derived_composition::XlsxTransitionalComposerComposition,
    }
    builder: XlsxTransitionalBuilder,
    analyzer: XlsxTransitionalAnalyzer,
    composer: XlsxTransitionalComposer,
);
