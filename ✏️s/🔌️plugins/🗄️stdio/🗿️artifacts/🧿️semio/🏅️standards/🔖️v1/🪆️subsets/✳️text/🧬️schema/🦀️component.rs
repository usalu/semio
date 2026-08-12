//! 🧬️ SemioTextArtifact schema — full artifact state, mirrors `SemioTextSnapshot` field for
//! field (see `✳️image`'s `SemioImageArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.text")]
pub struct SemioTextArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub runs: Vec<SemioTextRun>,
}

impl Default for SemioTextArtifact {
    fn default() -> Self { Self::from_snapshot(SemioTextSnapshot::default()) }
}

impl SemioTextArtifact {
    pub fn to_snapshot(&self) -> SemioTextSnapshot {
        SemioTextSnapshot { schema: self.schema.clone(), runs: self.runs.clone() }
    }
    pub fn from_snapshot(snapshot: SemioTextSnapshot) -> Self {
        Self { schema: snapshot.schema, runs: snapshot.runs }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioTextSnapshot) {
        self.schema = snapshot.schema;
        self.runs = snapshot.runs;
    }
}

pub fn semio_text_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.text",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::SemioTextDiff;
    use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
    use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextMark, SemioTextRun, SemioTextSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct SemioTextBuilderConstruction { snapshot: SemioTextSnapshot }

    //#region 🔖️TypedConstructors
    impl SemioTextBuilderConstruction {
        /// 🏗️ Starts a fresh, empty text document.
        pub fn new() -> Self { Self { snapshot: SemioTextSnapshot::default() } }
        /// 🏗️ Appends one run, in order.
        pub fn add_run(mut self, language: impl Into<String>, content: impl Into<String>, marks: Vec<SemioTextMark>) -> Self {
            self.snapshot.runs.push(SemioTextRun { language: language.into(), content: content.into(), marks });
            self
        }
    }
    //#endregion 🔖️TypedConstructors

    impl ArtifactBuilder for SemioTextBuilderConstruction {
        type Snapshot = SemioTextSnapshot;
        type Mutation = SemioTextMutation;
        type Diff = SemioTextDiff;
        fn empty() -> Self { Self { snapshot: SemioTextSnapshot::default() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioTextSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioTextSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<SemioTextSnapshot>>::diff(&mutation, &self.snapshot);
            self.snapshot = <Self::Diff as protocol::MutationDiff<SemioTextSnapshot>>::apply(&diff, &self.snapshot);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <SemioTextDiff as protocol::MutationDiff<SemioTextSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
    }

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextMarkKind;

        #[test]
        fn typed_constructors_build_a_populated_snapshot() {
            let snapshot = SemioTextBuilderConstruction::new()
                .add_run("en", "hello", vec![])
                .add_run("en", "world", vec![SemioTextMark { kind: SemioTextMarkKind::Bold, href: String::new() }])
                .build()
                .expect("build");
            assert_eq!(snapshot.runs.len(), 2);
            assert_eq!(snapshot.runs[1].marks.len(), 1);
        }
    }
    //#endregion 🔖️Tests
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextSnapshot, STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};

    #[derive(Clone, Debug, Default)]
    pub struct SemioTextParts { pub snapshot: Option<SemioTextSnapshot> }

    pub struct SemioTextAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioTextAnalyzerAnalysis {
        type Parts = SemioTextParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("text") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOTEXT_DOCUMENT_SCHEMA) { IoConfidence::High } else { IoConfidence::Low }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioTextParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioTextSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioTextSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec SemioTextBuilderFacets {
        construction: derived_construction::SemioTextBuilderConstruction,
        analysis: derived_analysis::SemioTextAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioTextComposerComposition,
    }
    builder: SemioTextBuilder,
    analyzer: SemioTextAnalyzer,
    composer: SemioTextComposer,
);
//#endregion 🧬️DerivedArtifactFacets
