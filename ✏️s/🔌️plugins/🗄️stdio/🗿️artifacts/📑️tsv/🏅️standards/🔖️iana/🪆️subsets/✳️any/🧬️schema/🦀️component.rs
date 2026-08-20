//! 🧬️ TsvArtifact schema — full artifact state, mirrors `TsvSnapshot` field for field.

use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::{LineEnding, TsvSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tsv")]
pub struct TsvArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub records: Vec<Vec<String>>,
    #[state(artifact)]
    #[serde(default)]
    pub trailing_newline: bool,
    #[state(artifact)]
    #[serde(default)]
    pub line_ending: LineEnding,
}

impl Default for TsvArtifact {
    fn default() -> Self {
        Self::from_snapshot(TsvSnapshot::default())
    }
}

impl TsvArtifact {
    pub async fn to_snapshot(&self) -> TsvSnapshot {
        TsvSnapshot { schema: self.schema.clone(), records: self.records.clone(), trailing_newline: self.trailing_newline, line_ending: self.line_ending }
    }
    pub async fn from_snapshot(snapshot: TsvSnapshot) -> Self {
        Self { schema: snapshot.schema, records: snapshot.records, trailing_newline: snapshot.trailing_newline, line_ending: snapshot.line_ending }
    }
    pub async fn set_snapshot(&mut self, snapshot: TsvSnapshot) {
        self.schema = snapshot.schema;
        self.records = snapshot.records;
        self.trailing_newline = snapshot.trailing_newline;
        self.line_ending = snapshot.line_ending;
    }
}

pub async fn tsv_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
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
    use crate::artifacts::tsv::standards::iana::subsets::any::schema::diff::TsvDiff;
    use crate::artifacts::tsv::standards::iana::subsets::any::schema::mutations::{apply_tsv_mutation, TsvMutation};
    use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct TsvBuilderConstruction {
        snapshot: TsvSnapshot,
    }

    impl ArtifactBuilder for TsvBuilderConstruction {
        type Snapshot = TsvSnapshot;
        type Mutation = TsvMutation;
        type Diff = TsvDiff;
        async fn empty() -> Self {
            Self { snapshot: TsvSnapshot::default() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<TsvSnapshot as store::ArtifactDsl>::parse_dsl(text).await?).await)
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<TsvSnapshot as store::ArtifactPack>::decode_pack(bytes).await?).await)
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_tsv_mutation(&mut self.snapshot, &mutation);
            (self, diff.await)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <TsvDiff as protocol::MutationDiff<TsvSnapshot>>::apply(&diff, &self.snapshot).await?;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            Ok(self.snapshot)
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot;
    use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::{TsvSnapshot, STDIO_TSV_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct TsvParts {
        pub snapshot: Option<TsvSnapshot>,
    }

    pub struct TsvAnalyzerAnalysis;

    impl ArtifactAnalysis for TsvAnalyzerAnalysis {
        type Parts = TsvParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tsv", standard: StandardId("iana"), subset: SubsetId("*") };

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if snapshot::sniff_real_bytes(bytes).await {
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
                    if snapshot::sniff_real_bytes(text.as_bytes()).await || text.contains(STDIO_TSV_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = TsvParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <TsvSnapshot as store::ArtifactDsl>::parse_dsl(text).await {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <TsvSnapshot as store::ArtifactPack>::decode_pack(bytes).await {
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
        construction: TsvBuilderConstruction,
        analysis: TsvAnalyzerAnalysis,
        composition: super::super::io::derived_composition::TsvComposerComposition,
    }
    builder: TsvBuilder,
    analyzer: TsvAnalyzer,
    composer: TsvComposer,
);
//#endregion 🧬️DerivedArtifactFacets
