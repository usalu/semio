//! 🚪️ IO stdio.xlsx (ecma-376/🔒️strict) — reuses the 🧱️base subset's `zip`/`xml` raw-codec DAG
//! leaves rather than duplicating them (same `XlsxSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! `🧱️base/🚪️io` and pdf `1.7/✳️a/🚪️io` already established for this artifact family.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
    use crate::standards::v_ecma_376::subsets::base::io::XlsxComposer as XlsxAnyComposer;
    use crate::standards::v_ecma_376::subsets::strict::schema::check_strict_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_STRICT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct XlsxStrictComposerComposition;

    impl ArtifactComposition for XlsxStrictComposerComposition {
        type Snapshot = XlsxSnapshot;
        const WRITES: Dialect = DIALECT_STRICT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_STRICT, DEP_ZIP]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = XlsxAnyComposer::compose(sources)?;
            let checks = check_strict_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("ISO/IEC 29500-1 Strict conformance violated: {} hard issue(s) -- not stamping the strict dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `ecma-376/strict` -- see the module doc comment for
    /// how this relates to (and honestly differs from) the composer's own pre-serialization hard gate
    /// above.
    pub struct XlsxStrictValidator;

    impl SubsetValidator for XlsxStrictValidator {
        const DIALECT: Dialect = DIALECT_STRICT;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <XlsxSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_strict_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.xlsx.strict.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "Xlsx Strict SubsetValidator: payload did not decode as an XlsxSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<XlsxStrictValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the ecma-376 standard's own `⚙️engine::register()`, which
    /// is already invoked from the artifact-level `crate::io_registry::register()`. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::standards::v_ecma_376::engine::io_registry::entries()`), matching how `🧱️base`'s
    /// own entry is registered.
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
    use crate::standards::v_ecma_376::subsets::strict::schema::{check_strict_conformance, stamp_strict_namespace};
    use crate::{XlsxDiff, XlsxMutation};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::Severity;
    use semio_framework_plugin::ArtifactBuilder;
    use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};

    //#region 🔖️Builder
    #[derive(Clone, Debug)]
    pub struct XlsxStrictBuilderConstruction {
        snapshot: XlsxSnapshot,
    }

    impl XlsxStrictBuilderConstruction {
        /// ➕️ The recommended entry point: builds a minimal package from `workbook` via the shared
        /// ecma-376 engine, then stamps it Strict.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new(workbook: XlsxWorkbook) -> Self {
            Self { snapshot: stamp_strict_namespace(crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(workbook)) }
        }
    }

    impl ArtifactBuilder for XlsxStrictBuilderConstruction {
        type Snapshot = XlsxSnapshot;
        type Mutation = XlsxMutation;
        type Diff = XlsxDiff;

        /// ⚠️ `ArtifactBuilder::empty()` is mandated no-arg by the SDK trait (generic UI/mutation
        /// dispatch needs every builder facet uniform) -- falls back to an empty workbook, stamped
        /// Strict regardless. Prefer `XlsxStrictBuilderConstruction::new(workbook)` directly wherever real content
        /// is known up front.
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

        /// 🛡️ The real construction gate: however `self.snapshot` got here (`new`, `from_binary`, a
        /// raw `from_snapshot`), a hard Strict violation fails `build()` -- the soft
        /// diagnostic (worksheet content-type mismatch) passes through as an advisory `Diagnostic`;
        /// the `Err` path is NOT taken for it, only hard ones block.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let hard: Vec<Diagnostic> = check_strict_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
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
    use crate::standards::v_ecma_376::subsets::strict::schema::check_strict_conformance;
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
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.xlsx` (ecma-376/🔒️strict): delegates the real parse to the 🧱️base subset's
    /// analyzer (same `XlsxSnapshot`), then folds real Strict conformance diagnostics on top. `sniff`
    /// also delegates -- a subset-level sniff for `strict` is "is this recognizable as an xlsx at
    /// all", the same probe every ecma-376 dialect shares; conformance is a separate, heavier
    /// question answered by `analyze`/`check_strict_conformance`, not by `sniff`.
    pub struct XlsxStrictAnalyzerAnalysis;

    impl ArtifactAnalysis for XlsxStrictAnalyzerAnalysis {
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
                let checks = check_strict_conformance(snapshot);
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
    pub spec XlsxStrictBuilderFacets {
        construction: XlsxStrictBuilderConstruction,
        analysis: XlsxStrictAnalyzerAnalysis,
        composition: super::io::derived_composition::XlsxStrictComposerComposition,
    }
    builder: XlsxStrictBuilder,
    analyzer: XlsxStrictAnalyzer,
    composer: XlsxStrictComposer,
);

#[path = "📝️text"]
pub mod text { #[path = "🧬️mutations/🦀️.rs"] pub mod mutations; }
#[path = "💾️binary"]
pub mod binary { #[path = "🧬️mutations/🦀️.rs"] pub mod mutations; }
