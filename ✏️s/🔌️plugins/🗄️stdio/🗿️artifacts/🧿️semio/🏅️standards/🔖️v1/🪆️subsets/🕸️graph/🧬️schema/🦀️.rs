//! 🧬️ SemioGraphArtifact schema — full artifact state, mirrors `SemioGraphSnapshot` field for
//! field (see `🔤️text`'s `SemioTextArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{SemioGraphEdge, SemioGraphNode, SemioGraphSnapshot};
use schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.graph")]
pub struct SemioGraphArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub nodes: Vec<SemioGraphNode>,
    #[state(artifact)]
    #[value(default)]
    pub edges: Vec<SemioGraphEdge>,
}

impl Default for SemioGraphArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioGraphSnapshot::default())
    }
}

impl SemioGraphArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SemioGraphSnapshot {
        SemioGraphSnapshot { schema: self.schema.clone(), nodes: self.nodes.clone(), edges: self.edges.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SemioGraphSnapshot) -> Self {
        Self { schema: snapshot.schema, nodes: snapshot.nodes, edges: snapshot.edges }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SemioGraphSnapshot) {
        self.schema = snapshot.schema;
        self.nodes = snapshot.nodes;
        self.edges = snapshot.edges;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_graph_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.graph",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
    use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::SemioGraphDiff;
    use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{apply_semio_graph_mutation, SemioGraphMutation};
    use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, SemioGraphEdge, SemioGraphNode, SemioGraphPort, SemioGraphSnapshot};
    use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioGraphBuilderConstruction {
        snapshot: SemioGraphSnapshot,
    }

    //#region 🔖️TypedConstructors
    impl SemioGraphBuilderConstruction {
        /// 🏗️ Starts a fresh, empty graph document.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            Self { snapshot: SemioGraphSnapshot::default() }
        }
        /// 🏗️ Appends one node, in insertion order (id-keyed set — order carries no display
        /// meaning, but insertion order is preserved for determinism).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_node(mut self, id: impl Into<String>, kind: impl Into<String>, label: impl Into<String>, position: SemioPoint2, ports: Vec<SemioGraphPort>, properties: Vec<SemioValueEntry>) -> Self {
            self.snapshot.nodes.push(SemioGraphNode { id: GraphNodeId::new(id), kind: kind.into(), label: label.into(), position, ports, properties });
            self
        }
        /// 🏗️ Appends one edge, in insertion order.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_edge(mut self, id: impl Into<String>, source: impl Into<String>, target: impl Into<String>, kind: impl Into<String>, label: impl Into<String>) -> Self {
            self.snapshot.edges.push(SemioGraphEdge { id: GraphEdgeId::new(id), source: GraphNodeId::new(source), target: GraphNodeId::new(target), kind: kind.into(), label: label.into() });
            self
        }
    }
    //#endregion 🔖️TypedConstructors

    impl ArtifactBuilder for SemioGraphBuilderConstruction {
        type Snapshot = SemioGraphSnapshot;
        type Mutation = SemioGraphMutation;
        type Diff = SemioGraphDiff;
        fn empty() -> Self {
            Self { snapshot: SemioGraphSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioGraphSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioGraphSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_semio_graph_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SemioGraphDiff as protocol::MutationDiff<SemioGraphSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            Ok(self.snapshot)
        }
    }

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn typed_constructors_build_a_populated_snapshot() {
            let snapshot = SemioGraphBuilderConstruction::new()
                .add_node("n1", "source", "Source", SemioPoint2 { x: 0.0, y: 0.0 }, vec![], vec![])
                .add_node("n2", "sink", "Sink", SemioPoint2 { x: 10.0, y: 10.0 }, vec![], vec![])
                .add_edge("e1", "n1", "n2", "flow", "Main")
                .build()
                .expect("build");
            assert_eq!(snapshot.nodes.len(), 2);
            assert_eq!(snapshot.edges.len(), 1);
        }
    }
    //#endregion 🔖️Tests
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{SemioGraphSnapshot, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioGraphParts {
        pub snapshot: Option<SemioGraphSnapshot>,
    }

    pub struct SemioGraphAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioGraphAnalyzerAnalysis {
        type Parts = SemioGraphParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("graph") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioGraphParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioGraphSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.graph", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioGraphSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec SemioGraphBuilderFacets {
        construction: SemioGraphBuilderConstruction,
        analysis: SemioGraphAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioGraphComposerComposition,
    }
    builder: SemioGraphBuilder,
    analyzer: SemioGraphAnalyzer,
    composer: SemioGraphComposer,
);
//#endregion 🧬️DerivedArtifactFacets
