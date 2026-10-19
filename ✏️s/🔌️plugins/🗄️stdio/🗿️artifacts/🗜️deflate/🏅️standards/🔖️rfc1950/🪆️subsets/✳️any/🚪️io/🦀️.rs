//! 🚪️ IO stdio.deflate (rfc1950/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_rfc1950::subsets::any::io::DeflateAnalyzer;
    use crate::DeflateSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct DeflateComposerComposition;

    impl ArtifactComposition for DeflateComposerComposition {
        type Snapshot = DeflateSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "DeflateComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = DeflateAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "DeflateComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

pub use semio_s_artifact_stdio_deflate_zlib::standards::v_rfc1950::subsets::any::io::zlib;
pub use zlib::*;
pub use binary::snapshot::{decode_deflate_snapshot, encode_deflate_snapshot};

//#region 🚪️DerivedIoRegistry
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
/// MACHINES) — pure `ComposerEntry` aggregation, no engine needed. NOTE: always reach this via a
/// fully-qualified path (`standards::v_rfc1950::subsets::any::io::io_registry::entries()`) — the
/// artifact root's OWN `io_registry` (`🗿️artifacts/🗜️deflate/🦀️.rs`) shadows this name
/// with a DIFFERENT return type (`&'static [&'static ComposerEntry]` vs this module's
/// `&'static [ComposerEntry]`); a bare `io_registry::entries()` silently rebinds to the wrong one.
pub mod io_registry {
    use crate::standards::v_rfc1950::subsets::any::io::DeflateComposer as DeflateRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<DeflateRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🔖️RegisterSchemaSpecs
/// 📇️ P2-FG2: `dsl::registry::register_schema_spec` (P2-M3's `FullResolver` insertion API) —
/// real, non-fabricated call: `DeflateSnapshot` derives `#[derive(dsl::DslRecord)]`, so
/// `__dsl_spec` genuinely exists (`../🧬️schema/📸️snapshot/🦀️.rs`). Only the snapshot
/// schema id is registered — `DeflateDiff` has NO derivable `RecordSpec` (its `dict_id:
/// Option<Option<u32>>` tri-state field blocks the derive), so `"stdio.deflate#diff"` is
/// deliberately not called here, matching the recipe's own "skip rather than fabricate" rule.
/// Dissolved out of the former `⚙️engine::register_schema_specs` — this is NOT one of stdio's 10
/// protected imperative plugin-root calls; it is a narrow `.setup(...)` gap-filler survivor of
/// `deflate::declaration()`'s own conversion, repointed directly at this new location by stdio's
/// plugin root (`🗄️stdio/🦀️.rs`).
#[cfg(not(target_arch = "wasm32"))]
pub fn register_schema_specs() {
    ::semio_framework_async::poll::resolve_ready(dsl::registry::register_schema_spec("stdio.deflate", DeflateSnapshot::__dsl_spec));
}

#[cfg(target_arch = "wasm32")]
pub fn register_schema_specs() {}
//#endregion 🔖️RegisterSchemaSpecs

pub mod binary {
    pub use semio_s_artifact_stdio_deflate_schema::standards::v_rfc1950::subsets::any::io::binary::*;
    pub use semio_s_artifact_stdio_deflate_examples::standards::v_rfc1950::subsets::any::io::binary::inferences;
}

pub mod text {
    pub use semio_s_artifact_stdio_deflate_schema::standards::v_rfc1950::subsets::any::io::text::*;
    pub use semio_s_artifact_stdio_deflate_examples::standards::v_rfc1950::subsets::any::io::text::inferences;
}

pub use semio_s_artifact_stdio_deflate_snapshot::standards::v_rfc1950::subsets::any::io::sqlite;

pub mod derived_construction {
    use crate::{DeflateDiff, DeflateMutation, DeflateSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.deflate` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct DeflateBuilderConstruction {
        snapshot: DeflateSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for DeflateBuilderConstruction {
        type Snapshot = DeflateSnapshot;
        type Mutation = DeflateMutation;
        type Diff = DeflateDiff;
        fn empty() -> Self {
            Self { snapshot: DeflateSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<DeflateSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<DeflateSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::apply_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
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
    use crate::DeflateSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.deflate` parts.
    #[derive(Clone, Debug, Default)]
    pub struct DeflateParts {
        pub snapshot: Option<DeflateSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.deflate` (rfc1950/✳️any) sources.
    pub struct DeflateAnalyzerAnalysis;

    impl ArtifactAnalysis for DeflateAnalyzerAnalysis {
        type Parts = DeflateParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            semio_framework_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = DeflateParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <DeflateSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <DeflateSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
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
    pub spec DeflateBuilderFacets {
        construction: DeflateBuilderConstruction,
        analysis: DeflateAnalyzerAnalysis,
        composition: DeflateComposerComposition,
    }
    builder: DeflateBuilder,
    analyzer: DeflateAnalyzer,
    composer: DeflateComposer,
);
