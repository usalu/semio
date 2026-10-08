//! 🚪️ IO stdio.dxf (r12/📰️header) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_r12::subsets::any::io::DxfAnalyzer;
    use crate::DxfSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct DxfComposerComposition;

    impl ArtifactComposition for DxfComposerComposition {
        type Snapshot = DxfSnapshot;
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
                return Err(ComposeError { message: "DxfComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = DxfAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "DxfComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v_r12::subsets::any::io::DxfComposer as DxfRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<DxfRawAnyComposer>()]).as_slice()
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
    use crate::{DxfDiff, DxfMutation, DxfSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.dxf` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct DxfBuilderConstruction {
        snapshot: DxfSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for DxfBuilderConstruction {
        type Snapshot = DxfSnapshot;
        type Mutation = DxfMutation;
        type Diff = DxfDiff;
        fn empty() -> Self {
            Self { snapshot: DxfSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<DxfSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<DxfSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
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
    use crate::DxfSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.dxf` parts.
    #[derive(Clone, Debug, Default)]
    pub struct DxfParts {
        pub snapshot: Option<DxfSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.dxf` (r12/📰️header) sources.
    pub struct DxfAnalyzerAnalysis;

    impl ArtifactAnalysis for DxfAnalyzerAnalysis {
        type Parts = DxfParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };

        /// 🧭️ DXF ASCII has no fixed magic byte (unlike binary formats), so this is a structural
        /// heuristic rather than an exact match: the first non-blank line must trim to a valid
        /// integer group code, and one of the DXF section/version markers (`SECTION`, `HEADER`,
        /// `ENTITIES`, or an `AC10xx`-style version string) must appear among the first tags.
        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            let text = match source {
                AnalyzeSource::Text(text) => Some(*text),
                AnalyzeSource::Binary(_) => None,
            };
            let Some(text) = text else { return semio_framework_plugin::io::Confidence::Low };
            if let Ok((envelope, _)) = store::semio_format::split_text_preamble(text) {
                return if envelope.matches_identity("stdio.dxf", store::semio_format::Component::Dsl, 1) { semio_framework_plugin::io::Confidence::High } else { semio_framework_plugin::io::Confidence::Low };
            }
            let lines: Vec<&str> = text.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
            let Some(first) = lines.first() else { return semio_framework_plugin::io::Confidence::Low };
            if first.parse::<i32>().is_err() {
                return semio_framework_plugin::io::Confidence::Low;
            }
            let has_marker = lines.iter().take(64).any(|l| matches!(*l, "SECTION" | "HEADER" | "ENTITIES" | "EOF") || (l.len() == 6 && l.starts_with("AC") && l[2..].chars().all(|c| c.is_ascii_digit())));
            if has_marker {
                semio_framework_plugin::io::Confidence::High
            } else {
                semio_framework_plugin::io::Confidence::Medium
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = DxfParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match if text.lines().find(|line| !line.trim().is_empty()).is_some_and(|line| line.trim().parse::<i32>().is_ok()) { crate::standards::v_r12::subsets::any::io::text::snapshot::parse_dxf_document(text).map_err(|error|semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error,semio_framework_diagnostic::TextSpan::at(1,1))) } else { <DxfSnapshot as store::ArtifactDsl>::parse_dsl(text) } {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <DxfSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec DxfBuilderFacets {
        construction: DxfBuilderConstruction,
        analysis: DxfAnalyzerAnalysis,
        composition: crate::standards::v_r12::subsets::any::io::derived_composition::DxfComposerComposition,
    }
    builder: DxfBuilder,
    analyzer: DxfAnalyzer,
    composer: DxfComposer,
);
