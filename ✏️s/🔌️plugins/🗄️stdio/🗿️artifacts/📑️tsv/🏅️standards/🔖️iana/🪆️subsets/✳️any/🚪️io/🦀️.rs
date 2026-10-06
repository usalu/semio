//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;
    use crate::standards::iana::subsets::any::io::TsvAnalyzer;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tsv", standard: StandardId("iana"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct TsvComposerComposition;

    impl ArtifactComposition for TsvComposerComposition {
        type Snapshot = TsvSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "TsvComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = TsvAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "TsvComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::iana::subsets::any::schema::tsv_artifact_schema_descriptor()).expect("schema descriptor publication");
        register_artifact_inferences();
        semio_framework_plugin::io::register_native_snapshot_codec(semio_framework_plugin::Dialect { artifact_kind: "s.stdio.tsv", standard: semio_framework_plugin::StandardId("iana"), subset: semio_framework_plugin::SubsetId("*") }, store::ArtifactCodec::of::<TsvSnapshot, crate::standards::iana::subsets::any::schema::mutations::TsvMutation>(crate::standards::iana::subsets::any::schema::snapshot::STDIO_TSV_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
    }

    /// 💡️ Registers `s.stdio.tsv.inference`'s facet leaves into the OS-wide inference catalog —
    /// sibling to the artifact schema descriptor above (separate registry, ticket
    /// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::iana::subsets::any::schema::inferences::tsv_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🦑 Dissolved out of the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — pure `ComposerEntry` aggregation, no
/// engine needed.
pub mod io_registry {
    use crate::standards::iana::subsets::any::io::TsvComposer as TsvRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<TsvRawAnyComposer>()]).as_slice()
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
    use crate::standards::iana::subsets::any::schema::diff::TsvDiff;
    use crate::standards::iana::subsets::any::schema::mutations::{apply_tsv_mutation, TsvMutation};
    use crate::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct TsvBuilderConstruction {
        snapshot: TsvSnapshot,
    }

    impl ArtifactBuilder for TsvBuilderConstruction {
        type Snapshot = TsvSnapshot;
        type Mutation = TsvMutation;
        type Diff = TsvDiff;
        fn empty() -> Self {
            Self { snapshot: TsvSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(crate::standards::iana::subsets::any::io::text::snapshot::read_tsv_source_text(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(crate::standards::iana::subsets::any::io::binary::snapshot::read_tsv_source_binary(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_tsv_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <TsvDiff as protocol::MutationDiff<TsvSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            Ok(self.snapshot)
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::iana::subsets::any::schema::snapshot;
    use crate::standards::iana::subsets::any::schema::snapshot::{TsvSnapshot, STDIO_TSV_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct TsvParts {
        pub snapshot: Option<TsvSnapshot>,
    }

    pub struct TsvAnalyzerAnalysis;

    impl ArtifactAnalysis for TsvAnalyzerAnalysis {
        type Parts = TsvParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tsv", standard: StandardId("iana"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if snapshot::sniff_real_bytes(bytes) {
                        return IoConfidence::High;
                    }
                    let marker = STDIO_TSV_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if snapshot::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_TSV_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = TsvParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match crate::standards::iana::subsets::any::io::text::snapshot::read_tsv_source_text(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match crate::standards::iana::subsets::any::io::binary::snapshot::read_tsv_source_binary(bytes) {
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
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec TsvBuilderFacets {
        construction: TsvBuilderConstruction,
        analysis: TsvAnalyzerAnalysis,
        composition: crate::standards::iana::subsets::any::io::derived_composition::TsvComposerComposition,
    }
    builder: TsvBuilder,
    analyzer: TsvAnalyzer,
    composer: TsvComposer,
);
