//! 🧬️ TsvArtifact schema — full artifact state, mirrors `TsvSnapshot` field for field.

use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::{LineEnding, TsvSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tsv")]
pub struct TsvArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub records: Vec<Vec<String>>,
    #[state(persistent)]
    #[serde(default)]
    pub trailing_newline: bool,
    #[state(persistent)]
    #[serde(default)]
    pub line_ending: LineEnding,
}

impl Default for TsvArtifact {
    fn default() -> Self { Self::from_snapshot(TsvSnapshot::default()) }
}

impl TsvArtifact {
    pub fn to_snapshot(&self) -> TsvSnapshot {
        TsvSnapshot {
            schema: self.schema.clone(),
            records: self.records.clone(),
            trailing_newline: self.trailing_newline,
            line_ending: self.line_ending,
        }
    }
    pub fn from_snapshot(snapshot: TsvSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            records: snapshot.records,
            trailing_newline: snapshot.trailing_newline,
            line_ending: snapshot.line_ending,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: TsvSnapshot) {
        self.schema = snapshot.schema;
        self.records = snapshot.records;
        self.trailing_newline = snapshot.trailing_newline;
        self.line_ending = snapshot.line_ending;
    }
}

pub fn tsv_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.tsv",
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
    use crate::artifacts::tsv::standards::iana::subsets::any::schema::diff::TsvDiff;
    use crate::artifacts::tsv::standards::iana::subsets::any::schema::mutations::{TsvMutation, apply_tsv_mutation};
    use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct TsvBuilderConstruction { snapshot: TsvSnapshot }

    impl ArtifactBuilder for TsvBuilderConstruction {
        type Snapshot = TsvSnapshot;
        type Mutation = TsvMutation;
        type Diff = TsvDiff;
        fn empty() -> Self { Self { snapshot: TsvSnapshot::default() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<TsvSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<TsvSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = apply_tsv_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <TsvDiff as protocol::MutationDiff<TsvSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::{TsvSnapshot, STDIO_TSV_DOCUMENT_SCHEMA};
    use crate::artifacts::tsv::standards::iana::engine as engine;

    #[derive(Clone, Debug, Default)]
    pub struct TsvParts { pub snapshot: Option<TsvSnapshot> }

    pub struct TsvAnalyzerAnalysis;

    impl ArtifactAnalysis for TsvAnalyzerAnalysis {
        type Parts = TsvParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tsv", standard: StandardId("iana"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if engine::sniff_real_bytes(bytes) {
                        return IoConfidence::High;
                    }
                    let marker = STDIO_TSV_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    if engine::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_TSV_DOCUMENT_SCHEMA) {
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
                    AnalyzeSource::Text(text) => match <TsvSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <TsvSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec TsvBuilderFacets {
        construction: derived_construction::TsvBuilderConstruction,
        analysis: derived_analysis::TsvAnalyzerAnalysis,
        composition: super::super::io::derived_composition::TsvComposerComposition,
    }
    builder: TsvBuilder,
    analyzer: TsvAnalyzer,
    composer: TsvComposer,
);
//#endregion 🧬️DerivedArtifactFacets
