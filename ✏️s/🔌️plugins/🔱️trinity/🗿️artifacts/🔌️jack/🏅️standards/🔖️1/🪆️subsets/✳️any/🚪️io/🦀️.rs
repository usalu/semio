/// 🧩️ Native composition accepts only this artifact's declared codecs; authored installations supply foreign formats.
pub mod derived_composition{
 use crate::JackSnapshot;
 use semio_framework_plugin::{ArtifactComposition,io::{AnalyzeSource,ComposeError,ComposeSource,Composition}};
 pub struct JackComposerComposition;
 impl ArtifactComposition for JackComposerComposition{
  type Snapshot=JackSnapshot;
  const WRITES:semio_framework_artifact_reference::Dialect=crate::TRINITY_JACK_DIALECT;
  fn reads()->&'static[semio_framework_artifact_reference::Dialect]{&[crate::TRINITY_JACK_DIALECT]}
  fn compose(sources:&[ComposeSource<'_>])->Result<Composition<JackSnapshot>,ComposeError>{use semio_framework_plugin::ArtifactAnalysis;for source in sources{if source.dialect!=Self::WRITES{continue}let payload=match&source.payload{AnalyzeSource::Text(text)=>AnalyzeSource::Text(text),AnalyzeSource::Binary(bytes)=>AnalyzeSource::Binary(bytes)};let analysis=super::JackAnalyzer::analyze(&[payload]);if let Some(snapshot)=analysis.parts.snapshot{return Ok(Composition{snapshot,confidence:analysis.confidence,diagnostics:analysis.diagnostics})}}Err(ComposeError{message:"Jack native composition requires its declared source dialect".into(),diagnostics:Vec::new()})}
 }
}
pub use derived_composition::*;
pub mod io_registry{pub fn entries()->&'static[semio_framework_plugin::io::ComposerEntry]{static ENTRIES:std::sync::LazyLock<Vec<semio_framework_plugin::io::ComposerEntry>>=std::sync::LazyLock::new(||vec![semio_framework_plugin::composer_entry_of::<super::JackComposer>()]);ENTRIES.as_slice()}}

#[path="🔣️json/🦀️.rs"] pub mod json_native;

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::{JackDiff, JackSnapshot, TrinityGraphMutation};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct JackBuilderConstruction {
        snapshot: JackSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for JackBuilderConstruction {
        type Snapshot = JackSnapshot;
        type Mutation = TrinityGraphMutation;
        type Diff = JackDiff;
        fn empty() -> Self {
            Self { snapshot: JackSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<JackSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<JackSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match protocol::apply_diff(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("build.apply", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
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
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::JackSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct JackParts {
        pub snapshot: Option<JackSnapshot>,
    }

    pub struct JackAnalyzerAnalysis;

    impl ArtifactAnalysis for JackAnalyzerAnalysis {
        type Parts = JackParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.trinity.jack", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            semio_framework_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = JackParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <JackSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <JackSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
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
    pub spec JackBuilderFacets {
        construction: JackBuilderConstruction,
        analysis: JackAnalyzerAnalysis,
        composition: JackComposerComposition,
    }
    builder: JackBuilder,
    analyzer: JackAnalyzer,
    composer: JackComposer,
);
