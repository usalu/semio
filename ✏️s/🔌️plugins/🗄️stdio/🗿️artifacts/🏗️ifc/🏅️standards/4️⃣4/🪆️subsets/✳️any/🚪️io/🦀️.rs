//! 🚪️ IO stdio.ifc (4/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register(). ifc's
//! own imperative registration is left alone (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
//! STATE-MACHINES explicit instruction — `ArtifactDeclaration` has exactly one `.schema()`/
//! `.document_codec()` slot and cannot hold both `4`'s and `2x3`'s independent descriptors/codecs
//! at once); only physically dissolved out of `⚙️engine`.
//#region 🔖️Submodules
/// 🏛️ Spatial structure + placement matrices + property sets, built on the shared
/// `semio_s_artifact_stdio_contract::part21` generic graph — never persisted itself. Reused externally by
/// `🧿️semio`'s own IFC4 deserializer, so this stays reachable at the same `engine::spatial` path
/// through the `engine` barrel shim.
#[path = "🏛️spatial/🦀️.rs"]
pub mod spatial;
//#endregion 🔖️Submodules

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v4::subsets::any::io::IfcAnalyzer;
    use crate::IfcSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct IfcComposerComposition;

    impl ArtifactComposition for IfcComposerComposition {
        type Snapshot = IfcSnapshot;
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
                return Err(ComposeError { message: "IfcComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = IfcAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "IfcComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
/// 🚪️ Dissolved out of `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub mod io_registry {
    use crate::standards::v4::subsets::any::io::IfcComposer as IfcRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<IfcRawAnyComposer>()]).as_slice()
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
    use crate::{IfcDiff, IfcMutation, IfcSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.ifc` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct IfcBuilderConstruction {
        snapshot: IfcSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for IfcBuilderConstruction {
        type Snapshot = IfcSnapshot;
        type Mutation = IfcMutation;
        type Diff = IfcDiff;
        fn empty() -> Self {
            Self { snapshot: IfcSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<IfcSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<IfcSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_ifc_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <IfcDiff as protocol::MutationDiff<IfcSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::IfcSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.ifc` parts.
    #[derive(Clone, Debug, Default)]
    pub struct IfcParts {
        pub snapshot: Option<IfcSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.ifc` (4/✳️any) sources.
    pub struct IfcAnalyzerAnalysis;

    impl ArtifactAnalysis for IfcAnalyzerAnalysis {
        type Parts = IfcParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = IfcParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <IfcSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <IfcSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec IfcBuilderFacets {
        construction: IfcBuilderConstruction,
        analysis: IfcAnalyzerAnalysis,
        composition: crate::standards::v4::subsets::any::io::derived_composition::IfcComposerComposition,
    }
    builder: IfcBuilder,
    analyzer: IfcAnalyzer,
    composer: IfcComposer,
);
