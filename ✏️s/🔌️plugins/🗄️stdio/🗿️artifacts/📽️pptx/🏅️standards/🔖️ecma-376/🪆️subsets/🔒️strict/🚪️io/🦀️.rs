//! 🚪️ IO stdio.pptx (ecma-376/🔒️strict) — reuses the 🧱️base subset's `zip`/`xml` raw-codec DAG
//! leaves rather than duplicating them (same `PptxSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! `🧱️base/🚪️io` already established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_ecma_376::subsets::base::io::PptxComposer as PptxAnyComposer;
    use crate::standards::v_ecma_376::subsets::strict::schema::conformance::check_strict_conformance;
    use crate::PptxSnapshot;
    use semio_framework_diagnostic::Diagnostic;
    use semio_framework_diagnostic::FaultCode;
    use semio_framework_diagnostic::Severity;
    use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_STRICT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct PptxStrictComposerComposition;

    impl ArtifactComposition for PptxStrictComposerComposition {
        type Snapshot = PptxSnapshot;
        const WRITES: Dialect = DIALECT_STRICT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_STRICT, DEP_ZIP]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = PptxAnyComposer::compose(sources)?;
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
    pub struct PptxStrictValidator;

    impl SubsetValidator for PptxStrictValidator {
        const DIALECT: Dialect = DIALECT_STRICT;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <PptxSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <PptxSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_strict_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.pptx.strict.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "Strict SubsetValidator: payload did not decode as a PptxSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<PptxStrictValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from ecma-376's own `⚙️engine::register()`. The `ComposerEntry`
    /// itself is registered separately by the standard-level composer aggregator
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
    use crate::standards::v_ecma_376::subsets::base::io::PptxBuilder as PptxAnyBuilder;
    use crate::standards::v_ecma_376::subsets::strict::schema::conformance::check_strict_conformance;
    use crate::{PptxDiff, PptxMutation, PptxSnapshot};
    use semio_framework_diagnostic::Diagnostic;
    use semio_framework_diagnostic::Severity;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct PptxStrictBuilderConstruction {
        inner: PptxAnyBuilder,
    }

    impl ArtifactBuilder for PptxStrictBuilderConstruction {
        type Snapshot = PptxSnapshot;
        type Mutation = PptxMutation;
        type Diff = PptxDiff;

        fn empty() -> Self {
            Self { inner: PptxAnyBuilder::empty() }
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { inner: PptxAnyBuilder::from_snapshot(snapshot) }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self { inner: PptxAnyBuilder::from_text(text)? })
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self { inner: PptxAnyBuilder::from_binary(bytes)? })
        }

        fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let (inner, diff) = self.inner.mutate(mutation);
            (Self { inner }, diff)
        }

        fn absorb(self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            Ok(Self { inner: self.inner.absorb(diff)? })
        }

        /// 🛡️ The real construction gate: however `self`'s inner snapshot got here, a hard
        /// ISO/IEC 29500-1 Strict violation fails `build()` -- soft diagnostics (missing
        /// `conformance="strict"`, `mc:AlternateContent`) pass through as advisory `Diagnostic`s;
        /// the `Err` path is NOT taken for those, only hard ones block.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let snapshot = self.inner.build()?;
            let hard: Vec<Diagnostic> = check_strict_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
            if hard.is_empty() {
                Ok(snapshot)
            } else {
                Err(hard)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v_ecma_376::subsets::base::io::{PptxAnalyzer as PptxAnyAnalyzer, PptxParts};
    use crate::PptxSnapshot;
    use semio_framework_diagnostic::Diagnostic;
    use semio_framework_diagnostic::FaultCode;
    use semio_framework_diagnostic::FaultScope;
    use semio_framework_diagnostic::Severity;
    use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
    use semio_s_artifact_stdio_zip::opc::OpcPackage;

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };

    use crate::standards::v_ecma_376::subsets::strict::schema::conformance::*;


    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.pptx` (ecma-376/🔒️strict): delegates the real parse to the 🧱️base subset's
    /// analyzer (same `PptxSnapshot`), then folds real ISO/IEC 29500-1 Strict conformance diagnostics
    /// on top. `sniff` also delegates -- a subset-level sniff for `strict` is "is this recognizable
    /// as a pptx at all", the same OPC-shaped probe every ecma-376 dialect shares; conformance is a
    /// separate, heavier question answered by `analyze`/`check_strict_conformance`, not by `sniff`.
    pub struct PptxStrictAnalyzerAnalysis;

    impl ArtifactAnalysis for PptxStrictAnalyzerAnalysis {
        type Parts = PptxParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            PptxAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = PptxAnyAnalyzer::analyze(sources);
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

    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec PptxStrictBuilderFacets {
        construction: PptxStrictBuilderConstruction,
        analysis: PptxStrictAnalyzerAnalysis,
        composition: super::io::derived_composition::PptxStrictComposerComposition,
    }
    builder: PptxStrictBuilder,
    analyzer: PptxStrictAnalyzer,
    composer: PptxStrictComposer,
);

#[path = "📝️text"]
pub mod text { #[path = "🧬️mutations/🦀️.rs"] pub mod mutations; }
#[path = "💾️binary"]
pub mod binary { #[path = "🧬️mutations/🦀️.rs"] pub mod mutations; }
