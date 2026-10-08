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
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};
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
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxSnapshot, XlsxWorkbook};
    use crate::standards::v_ecma_376::subsets::transitional::schema::{check_transitional_conformance, stamp_transitional_namespace};
    use crate::{XlsxDiff, XlsxMutation};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::Severity;
    use semio_framework_plugin::ArtifactBuilder;
    use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};

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
            Self { snapshot: stamp_transitional_namespace(crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(workbook)) }
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
            let (next, diff) = store::apply_outcome(&self.snapshot, protocol::Mutation::diff(&mutation, &self.snapshot));
            self.snapshot = next;
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = semio_framework_os_kernel::apply_diff(&diff, &self.snapshot)?;
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
    use crate::standards::v_ecma_376::subsets::transitional::schema::check_transitional_conformance;
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
    use crate::standards::v_ecma_376::subsets::base::io::XlsxAnalyzer as XlsxAnyAnalyzer;
    pub use crate::standards::v_ecma_376::subsets::base::io::XlsxParts;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
    use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.xlsx` (ecma-376/🌉️transitional): delegates the real parse to the 🧱️base
    /// subset's analyzer (same `XlsxSnapshot`), then folds real Transitional conformance diagnostics
    /// on top. `sniff` also delegates -- same rationale as 🔒️strict's analyzer.
    pub struct XlsxTransitionalAnalyzerAnalysis;

    impl ArtifactAnalysis for XlsxTransitionalAnalyzerAnalysis {
        type Parts = XlsxParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            XlsxAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = XlsxAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_transitional_conformance(snapshot);
                if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                    confidence = semio_framework_plugin::io::Confidence::Low;
                }
                diagnostics.extend(checks);
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer


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
