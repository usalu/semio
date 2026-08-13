//! 🧬️ SemioFlowArtifact schema — full artifact state, mirrors `SemioFlowSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::{SemioFlowSnapshot, FlowNode, FlowEdge};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.flow")]
pub struct SemioFlowArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub nodes: Vec<FlowNode>,
    #[state(artifact)]
    #[serde(default)]
    pub edges: Vec<FlowEdge>,
}

impl Default for SemioFlowArtifact {
    fn default() -> Self { Self::from_snapshot(SemioFlowSnapshot::default()) }
}

impl SemioFlowArtifact {
    pub fn to_snapshot(&self) -> SemioFlowSnapshot {
        SemioFlowSnapshot {
            schema: self.schema.clone(),
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioFlowSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            nodes: snapshot.nodes,
            edges: snapshot.edges,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioFlowSnapshot) {
        self.schema = snapshot.schema;
        self.nodes = snapshot.nodes;
        self.edges = snapshot.edges;
    }
}

pub fn semio_flow_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.flow",
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
    use crate::artifacts::semio::standards::v1::subsets::flow::schema::diff::SemioFlowDiff;
    use crate::artifacts::semio::standards::v1::subsets::flow::schema::mutations::{SemioFlowMutation, apply_semio_flow_mutation};
    use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct SemioFlowBuilderConstruction { snapshot: SemioFlowSnapshot }

    impl ArtifactBuilder for SemioFlowBuilderConstruction {
        type Snapshot = SemioFlowSnapshot;
        type Mutation = SemioFlowMutation;
        type Diff = SemioFlowDiff;
        fn empty() -> Self { Self { snapshot: SemioFlowSnapshot::default() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioFlowSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioFlowSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = apply_semio_flow_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <SemioFlowDiff as protocol::MutationDiff<SemioFlowSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::{SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};

    #[derive(Clone, Debug, Default)]
    pub struct SemioFlowParts { pub snapshot: Option<SemioFlowSnapshot> }

    pub struct SemioFlowAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioFlowAnalyzerAnalysis {
        type Parts = SemioFlowParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("flow") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOFLOW_DOCUMENT_SCHEMA) { IoConfidence::High } else { IoConfidence::Low }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioFlowParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioFlowSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioFlowSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec SemioFlowBuilderFacets {
        construction: derived_construction::SemioFlowBuilderConstruction,
        analysis: derived_analysis::SemioFlowAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioFlowComposerComposition,
    }
    builder: SemioFlowBuilder,
    analyzer: SemioFlowAnalyzer,
    composer: SemioFlowComposer,
);
//#endregion 🧬️DerivedArtifactFacets
