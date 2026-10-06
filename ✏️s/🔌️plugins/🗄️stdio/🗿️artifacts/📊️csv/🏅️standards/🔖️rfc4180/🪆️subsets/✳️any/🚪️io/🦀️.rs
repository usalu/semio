//! 🚪️ IO stdio.csv (rfc4180/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_rfc4180::subsets::any::io::CsvAnalyzer;
    use crate::CsvSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct CsvComposerComposition;

    impl ArtifactComposition for CsvComposerComposition {
        type Snapshot = CsvSnapshot;
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
                return Err(ComposeError { message: "CsvComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = CsvAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "CsvComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🦑 Dissolved out of the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — pure `ComposerEntry` aggregation, no
/// engine needed. NOTE: always reach this via a fully-qualified path
/// (`standards::v_rfc4180::subsets::any::io::io_registry::entries()`) — the artifact root's OWN
/// `io_registry` (`🗿️artifacts/📊️csv/🦀️.rs`) shadows this name with a DIFFERENT return
/// type (`&'static [&'static ComposerEntry]` vs this module's `&'static [ComposerEntry]`); a bare
/// `io_registry::entries()` silently rebinds to the wrong one.
pub mod io_registry {
    use crate::standards::v_rfc4180::subsets::any::io::CsvComposer as CsvRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<CsvRawAnyComposer>()]).as_slice()
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
    use crate::{CsvDiff, CsvMutation, CsvSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.csv` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct CsvBuilderConstruction {
        snapshot: CsvSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for CsvBuilderConstruction {
        type Snapshot = CsvSnapshot;
        type Mutation = CsvMutation;
        type Diff = CsvDiff;
        fn empty() -> Self {
            Self { snapshot: CsvSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(crate::standards::v_rfc4180::subsets::any::io::text::snapshot::read_csv_source_text(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(crate::standards::v_rfc4180::subsets::any::io::binary::snapshot::read_csv_source_binary(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_csv_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <CsvDiff as protocol::MutationDiff<CsvSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::CsvSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.csv` parts.
    #[derive(Clone, Debug, Default)]
    pub struct CsvParts {
        pub snapshot: Option<CsvSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.csv` (rfc4180/✳️any) sources.
    pub struct CsvAnalyzerAnalysis;

    /// 🔍 CSV has no magic bytes — sniff by checking that a real RFC4180 parse of the
    /// first few lines yields a consistent field count across records (a strong tabular
    /// signal) and that at least one delimiter/quote is actually present.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn looks_like_csv(text: &str) -> IoConfidence {
        let sample: String = text.lines().take(20).collect::<Vec<_>>().join("\n");
        if sample.trim().is_empty() {
            return IoConfidence::Low;
        }
        let snapshot = crate::standards::v_rfc4180::subsets::any::io::text::snapshot::decode_csv_with(&sample, false);
        if snapshot.records.is_empty() {
            return IoConfidence::Low;
        }
        let width = snapshot.records[0].fields.len();
        if width == 0 {
            return IoConfidence::Low;
        }
        let consistent = snapshot.records.iter().all(|r| r.fields.len() == width);
        let has_delimiter = sample.contains(',');
        match (consistent, width > 1, has_delimiter) {
            (true, true, true) => IoConfidence::High,
            (true, _, true) => IoConfidence::Medium,
            _ => IoConfidence::Low,
        }
    }

    impl ArtifactAnalysis for CsvAnalyzerAnalysis {
        type Parts = CsvParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Text(text) => if text.starts_with("semio "){if crate::standards::v_rfc4180::subsets::any::io::text::snapshot::read_csv_source_text(text).is_ok(){IoConfidence::High}else{IoConfidence::Low}}else{looks_like_csv(text)},
                AnalyzeSource::Binary(bytes) => if bytes.starts_with(&[137,83,69,77,13,10,26,10]){if crate::standards::v_rfc4180::subsets::any::io::binary::snapshot::read_csv_source_binary(bytes).is_ok(){IoConfidence::High}else{IoConfidence::Low}}else{std::str::from_utf8(bytes).map(looks_like_csv).unwrap_or(IoConfidence::Low)},
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = CsvParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match crate::standards::v_rfc4180::subsets::any::io::text::snapshot::read_csv_source_text(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match crate::standards::v_rfc4180::subsets::any::io::binary::snapshot::read_csv_source_binary(bytes) {
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

    //#region 🧪️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec CsvBuilderFacets {
        construction: CsvBuilderConstruction,
        analysis: CsvAnalyzerAnalysis,
        composition: crate::standards::v_rfc4180::subsets::any::io::derived_composition::CsvComposerComposition,
    }
    builder: CsvBuilder,
    analyzer: CsvAnalyzer,
    composer: CsvComposer,
);
