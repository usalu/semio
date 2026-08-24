//! 🧬️ Puzzle2d artifact schema — every field of the artifact with its state class.

use crate::artifacts::puzzle2d::{Puzzle2dCamera, Puzzle2dEdge, Puzzle2dMeta, Puzzle2dNode, Puzzle2dSnapshot};
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full puzzle2d artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle2d")]
pub struct Puzzle2dArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub camera: Puzzle2dCamera,
    #[state(artifact)]
    pub nodes: Vec<Puzzle2dNode>,
    #[state(artifact)]
    pub edges: Vec<Puzzle2dEdge>,
    #[state(artifact)]
    pub meta: Puzzle2dMeta,
    #[state(presence)]
    pub selected_ids: Vec<String>,
    #[state(presence)]
    pub active_utility_id: String,
    #[state(config)]
    pub camera_x: f64,
    #[state(config)]
    pub camera_y: f64,
    #[state(config)]
    pub camera_zoom: f64,
    #[state(config)]
    pub selection_method: String,
    #[state(config)]
    pub grid_snap_enabled: bool,
    #[state(config)]
    pub grid_factor: f64,
    #[state(config)]
    pub suggestion_offset: f64,
    #[state(config)]
    pub fill_count: u32,
    #[state(config)]
    pub brush_candidate_index: u32,
    #[state(config)]
    pub brush_candidate_source_handle_id: String,
    #[state(config)]
    pub locale: String,
    #[state(config)]
    pub terminology: String,
    #[state(config)]
    pub lod_mode_by_pane_json: String,
    #[state(config)]
    pub engagement_input_by_pane_json: String,
    #[state(config)]
    pub brush_candidates_json: String,
    #[state(config)]
    pub node_kind_weights_json: String,
    #[state(config)]
    pub handle_kind_weights_json: String,
    #[state(config)]
    pub active_utility_by_window_id_json: String,
    #[state(artifact)]
    pub hovered_node_id: Option<String>,
    #[state(artifact)]
    pub preview_seq: i64,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Puzzle2dArtifact {
    fn default() -> Self {
        Self::from_snapshot(Puzzle2dSnapshot::default())
    }
}

impl Puzzle2dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Puzzle2dSnapshot {
        Puzzle2dSnapshot { schema: self.schema.clone(), camera: self.camera.clone(), nodes: self.nodes.clone(), edges: self.edges.clone(), meta: self.meta.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: Puzzle2dSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            camera: snapshot.camera,
            nodes: snapshot.nodes,
            edges: snapshot.edges,
            meta: snapshot.meta,
            selected_ids: Vec::new(),
            active_utility_id: "select".into(),
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            selection_method: "rectangle".into(),
            grid_snap_enabled: false,
            grid_factor: 1.0,
            suggestion_offset: 80.0,
            fill_count: 0,
            brush_candidate_index: 0,
            brush_candidate_source_handle_id: String::new(),
            locale: "en-US".into(),
            terminology: "native".into(),
            lod_mode_by_pane_json: "{}".into(),
            engagement_input_by_pane_json: "{}".into(),
            brush_candidates_json: "{}".into(),
            node_kind_weights_json: "{}".into(),
            handle_kind_weights_json: "{}".into(),
            active_utility_by_window_id_json: "{}".into(),
            hovered_node_id: None,
            preview_seq: 0,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Puzzle2dSnapshot) {
        self.schema = snapshot.schema;
        self.camera = snapshot.camera;
        self.nodes = snapshot.nodes;
        self.edges = snapshot.edges;
        self.meta = snapshot.meta;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.puzzle.puzzle2d` — twenty handcrafted schema leaves.
pub fn puzzle2d_artifact_schema_descriptor() -> artifact_schema::ArtifactSchemaDescriptor {
    artifact_schema::ArtifactSchemaDescriptor {
        id: "s.puzzle.puzzle2d",
        artifact: artifact_schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: artifact_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: artifact_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: artifact_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::puzzle2d::{Puzzle2dDiff, Puzzle2dMutation, Puzzle2dSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Puzzle2dBuilderConstruction {
        snapshot: Puzzle2dSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Puzzle2dBuilderConstruction {
        type Snapshot = Puzzle2dSnapshot;
        type Mutation = Puzzle2dMutation;
        type Diff = Puzzle2dDiff;
        async fn empty() -> Self {
            Self { snapshot: Puzzle2dSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self { snapshot: <Puzzle2dSnapshot as store::ArtifactDsl>::parse_dsl(text)?, diagnostics: Vec::new() })
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self { snapshot: <Puzzle2dSnapshot as store::ArtifactPack>::decode_pack(bytes)?, diagnostics: Vec::new() })
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Puzzle2dDiff as protocol::MutationDiff<Puzzle2dSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Puzzle2dParts {
        pub snapshot: Option<Puzzle2dSnapshot>,
    }

    pub struct Puzzle2dAnalyzerAnalysis;

    impl ArtifactAnalysis for Puzzle2dAnalyzerAnalysis {
        type Parts = Puzzle2dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.puzzle2d", standard: StandardId("1"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Puzzle2dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Puzzle2dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Puzzle2dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
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
    pub spec Puzzle2dBuilderFacets {
        construction: Puzzle2dBuilderConstruction,
        analysis: Puzzle2dAnalyzerAnalysis,
        composition: super::super::io::derived_composition::Puzzle2dComposerComposition,
    }
    builder: Puzzle2dBuilder,
    analyzer: Puzzle2dAnalyzer,
    composer: Puzzle2dComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 📄️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// W1e): a pure default-snapshot constructor over document types, no `AppIo`/app dependency.
pub fn empty_puzzle2d_snapshot() -> Puzzle2dSnapshot {
    Puzzle2dSnapshot::default()
}
//#endregion 🔖️DocumentHelpers
