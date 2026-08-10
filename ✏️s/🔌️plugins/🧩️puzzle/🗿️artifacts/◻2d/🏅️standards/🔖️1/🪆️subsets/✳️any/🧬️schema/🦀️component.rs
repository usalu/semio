//! 🧬️ Puzzle2d artifact schema — every field of the artifact with its state class.

use crate::artifacts::puzzle2d::{Puzzle2dCamera, Puzzle2dEdge, Puzzle2dMeta, Puzzle2dNode, Puzzle2dSnapshot, PUZZLE_2D_SCHEMA};
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full puzzle2d artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle2d")]
pub struct Puzzle2dArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub camera: Puzzle2dCamera,
    #[state(persistent)] pub nodes: Vec<Puzzle2dNode>,
    #[state(persistent)] pub edges: Vec<Puzzle2dEdge>,
    #[state(persistent)] pub meta: Puzzle2dMeta,
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub camera_x: f64,
    #[state(local_ui)] pub camera_y: f64,
    #[state(local_ui)] pub camera_zoom: f64,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub grid_snap_enabled: bool,
    #[state(local_ui)] pub grid_factor: f64,
    #[state(local_ui)] pub suggestion_offset: f64,
    #[state(local_ui)] pub fill_count: u32,
    #[state(local_ui)] pub brush_candidate_index: u32,
    #[state(local_ui)] pub brush_candidate_source_handle_id: String,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub terminology: String,
    #[state(local_ui)] pub lod_mode_by_pane_json: String,
    #[state(local_ui)] pub engagement_input_by_pane_json: String,
    #[state(local_ui)] pub brush_candidates_json: String,
    #[state(local_ui)] pub node_kind_weights_json: String,
    #[state(local_ui)] pub handle_kind_weights_json: String,
    #[state(local_ui)] pub active_utility_by_window_id_json: String,
    #[state(preview)] pub hovered_node_id: Option<String>,
    #[state(preview)] pub preview_seq: i64,
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
        Puzzle2dSnapshot {
            schema: self.schema.clone(),
            camera: self.camera.clone(),
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
            meta: self.meta.clone(),
        }
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

