//! 🚪️ IO stdio.tiff (6.0/🧱️baseline) — reuses the ✳️any subset's `binary` raw-codec DAG leaf
//! rather than duplicating it (same `TiffSnapshot` type, same catalog DAG edges). Registration
//! flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator,
//! and the `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io`
//! already established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v6_0::subsets::baseline::schema::check_tiff_baseline_conformance;
    use crate::standards::v6_0::subsets::document::schema::snapshot::TiffSnapshot;
    use crate::standards::v6_0::subsets::document::io::TiffComposer as TiffAnyComposer;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::register_subset_validator,semio_framework_plugin::subset_validator_entry_of,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::SubsetValidator,semio_framework_plugin::SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_BASELINE: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("baseline") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct TiffBaselineComposerComposition;

    impl ArtifactComposition for TiffBaselineComposerComposition {
        type Snapshot = TiffSnapshot;
        const WRITES: Dialect = DIALECT_BASELINE;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_BASELINE, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = TiffAnyComposer::compose(sources)?;
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(check_tiff_baseline_conformance(&inner.snapshot));
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct TiffBaselineValidator;

    impl SubsetValidator for TiffBaselineValidator {
        const DIALECT: Dialect = DIALECT_BASELINE;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <TiffSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <TiffSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_tiff_baseline_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.tiff.baseline.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "Baseline TIFF (6.0) SubsetValidator: payload did not decode as a TiffSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<TiffBaselineValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator`. Called from 6.0's own `⚙️engine::register()`.
    /// The `ComposerEntry` itself is registered separately via this standard's own
    /// `composer::entries()` aggregation.
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
    use crate::standards::v6_0::subsets::baseline::schema::check_tiff_baseline_conformance;
    use crate::standards::v6_0::subsets::document::schema::{diff::TiffDiff, mutations::TiffMutation, snapshot::TiffSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct TiffBaselineBuilderConstruction {
        snapshot: TiffSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for TiffBaselineBuilderConstruction {
        type Snapshot = TiffSnapshot;
        type Mutation = TiffMutation;
        type Diff = TiffDiff;

        fn empty() -> Self {
            Self { snapshot: TiffSnapshot::default(), diagnostics: Vec::new() }
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<TiffSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::standards::v6_0::subsets::document::schema::mutations::apply_tiff_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <TiffDiff as protocol::MutationDiff<TiffSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ Re-runs the honestly-scope-limited Baseline TIFF check -- always SOFT at this schema,
        /// so `build()` never fails; the diagnostics still surface via the analyzer/composer/
        /// validator paths for anyone inspecting them.
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            let _ = check_tiff_baseline_conformance(&self.snapshot);
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v6_0::subsets::document::schema::snapshot::{TiffSnapshot};
    use crate::standards::v6_0::subsets::document::io::{TiffAnalyzer as TiffAnyAnalyzer, TiffParts};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoConfidence,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("baseline") };

    //#region 🔖️Conformance
    use crate::standards::v6_0::subsets::baseline::schema::conformance::*;
    pub fn check_tiff_baseline_conformance_controlled(snapshot:&TiffSnapshot,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<Vec<Diagnostic>,semio_framework_value::ValueError>{check_tiff_baseline_conformance_with(snapshot,&mut |at,total|control.checkpoint(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot,at,total))}
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.tiff` (6.0/🧱️baseline): delegates the real parse to the ✳️any subset's
    /// analyzer (same `TiffSnapshot`), then folds the real Baseline TIFF diagnostics on top.
    pub struct TiffBaselineAnalyzerAnalysis;

    impl ArtifactAnalysis for TiffBaselineAnalyzerAnalysis {
        type Parts = TiffParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            TiffAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = TiffAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            if let Some(snapshot) = &inner.parts.snapshot {
                diagnostics.extend(check_tiff_baseline_conformance(snapshot));
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence: inner.confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec TiffBaselineBuilderFacets {
        construction: TiffBaselineBuilderConstruction,
        analysis: TiffBaselineAnalyzerAnalysis,
        composition: super::io::derived_composition::TiffBaselineComposerComposition,
    }
    builder: TiffBaselineBuilder,
    analyzer: TiffBaselineAnalyzer,
    composer: TiffBaselineComposer,
);
