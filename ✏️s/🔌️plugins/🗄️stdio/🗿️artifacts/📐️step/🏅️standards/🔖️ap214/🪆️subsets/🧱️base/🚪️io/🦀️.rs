//! 🚪️ IO stdio.step (ap214/🧱️base) — registration now flows through the `s.stdio.step`
//! `ArtifactDeclaration` (`crate::declaration`), not per-leaf register().
//#region 🔖️Submodules
/// 🧱 BrepMesh analyzer view, derived from the generic graph — never persisted itself.
#[path = "🧱️brep/🦀️.rs"]
pub mod brep;
/// 🪜 Shared CC ladder classification + FILE_SCHEMA/PRODUCT-chain scans, reused by all six
/// `✳️ccN` subset analyzers (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES).
#[path = "🪜️ladder/🦀️.rs"]
pub mod ladder;
//#endregion 🔖️Submodules

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_ap214::subsets::base::io::StepAnalyzer;
    use crate::StepSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct StepComposerComposition;

    impl ArtifactComposition for StepComposerComposition {
        type Snapshot = StepSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_TXT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "StepComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = StepAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "StepComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🚪️ Dissolved out of `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// the composed `any` + `cc1`..`cc6` entry union `declaration()`'s `.composers(...)` reaches
/// through the `engine` barrel shim (`🦀️.rs`'s `pub mod engine { pub use super::subsets::
/// any::io::*; pub use super::subsets::any::schema::*; }`).
pub mod io_registry {
    use crate::standards::v_ap214::subsets::base::io::StepComposer as StepRawAnyComposer;
    use crate::standards::v_ap214::subsets::cc1::io::StepCc1Composer;
    use crate::standards::v_ap214::subsets::cc2::io::StepCc2Composer;
    use crate::standards::v_ap214::subsets::cc3::io::StepCc3Composer;
    use crate::standards::v_ap214::subsets::cc4::io::StepCc4Composer;
    use crate::standards::v_ap214::subsets::cc5::io::StepCc5Composer;
    use crate::standards::v_ap214::subsets::cc6::io::StepCc6Composer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<StepRawAnyComposer>(),
                    composer_entry_of::<StepCc1Composer>(),
                    composer_entry_of::<StepCc2Composer>(),
                    composer_entry_of::<StepCc3Composer>(),
                    composer_entry_of::<StepCc4Composer>(),
                    composer_entry_of::<StepCc5Composer>(),
                    composer_entry_of::<StepCc6Composer>(),
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::{StepDiff, StepMutation, StepSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.step` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct StepBuilderConstruction {
        snapshot: StepSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for StepBuilderConstruction {
        type Snapshot = StepSnapshot;
        type Mutation = StepMutation;
        type Diff = StepDiff;
        fn empty() -> Self {
            Self { snapshot: StepSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<StepSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<StepSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_step_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <StepDiff as protocol::MutationDiff<StepSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::StepSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.step` parts.
    #[derive(Clone, Debug, Default)]
    pub struct StepParts {
        pub snapshot: Option<StepSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.step` (ap214/🧱️base) sources.
    pub struct StepAnalyzerAnalysis;

    impl ArtifactAnalysis for StepAnalyzerAnalysis {
        type Parts = StepParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = StepParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <StepSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <StepSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec StepBuilderFacets {
        construction: StepBuilderConstruction,
        analysis: StepAnalyzerAnalysis,
        composition: crate::standards::v_ap214::subsets::base::io::derived_composition::StepComposerComposition,
    }
    builder: StepBuilder,
    analyzer: StepAnalyzer,
    composer: StepComposer,
);
