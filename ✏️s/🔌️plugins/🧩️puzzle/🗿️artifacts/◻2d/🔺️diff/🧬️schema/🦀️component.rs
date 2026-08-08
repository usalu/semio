//! 🧬️ Puzzle2d diff schema — sparse field delta over the artifact.

use crate::artifacts::puzzle2d::{Puzzle2dCamera, Puzzle2dEdge, Puzzle2dMeta, Puzzle2dNode};
use crate::artifacts::puzzle2d::schema::Puzzle2dArtifact;
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the puzzle2d artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle2d")]
pub struct Puzzle2dDiff {
    #[state(persistent)] pub artifact: Option<Box<Puzzle2dArtifact>>,
    #[state(persistent)] pub schema: Option<String>,
    #[state(persistent)] pub camera: Option<Puzzle2dCamera>,
    #[state(persistent)] pub nodes: Option<Puzzle2dNodesDelta>,
    #[state(persistent)] pub edges: Option<Puzzle2dEdgesDelta>,
    #[state(persistent)] pub meta: Option<Puzzle2dMeta>,
    #[state(shared_ui)] pub selected_ids: Option<Puzzle2dStringList>,
    #[state(shared_ui)] pub active_utility_id: Option<String>,
    #[state(local_ui)] pub camera_x: Option<f64>,
    #[state(local_ui)] pub camera_y: Option<f64>,
    #[state(local_ui)] pub camera_zoom: Option<f64>,
    #[state(local_ui)] pub selection_method: Option<String>,
    #[state(local_ui)] pub grid_snap_enabled: Option<bool>,
    #[state(local_ui)] pub grid_factor: Option<f64>,
    #[state(local_ui)] pub suggestion_offset: Option<f64>,
    #[state(local_ui)] pub fill_count: Option<u32>,
    #[state(local_ui)] pub brush_candidate_index: Option<u32>,
    #[state(local_ui)] pub brush_candidate_source_handle_id: Option<String>,
    #[state(local_ui)] pub locale: Option<String>,
    #[state(local_ui)] pub terminology: Option<String>,
    #[state(local_ui)] pub lod_mode_by_pane_json: Option<String>,
    #[state(local_ui)] pub engagement_input_by_pane_json: Option<String>,
    #[state(local_ui)] pub brush_candidates_json: Option<String>,
    #[state(local_ui)] pub node_kind_weights_json: Option<String>,
    #[state(local_ui)] pub handle_kind_weights_json: Option<String>,
    #[state(local_ui)] pub active_utility_by_window_id_json: Option<String>,
    #[state(preview)] pub hovered_node_id: Option<Option<String>>,
    #[state(preview)] pub preview_seq: Option<i64>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle2dStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `nodes`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle2dNodesDelta {
    pub added: Vec<Puzzle2dNode>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle2dNodePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle2dNode` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dNodePatchEntry {
    pub id: String,
    pub patch: Puzzle2dNodePatch,
}

/// 🩹 Sparse patch over `Puzzle2dNode` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle2dNodePatch {
    pub replacement: Option<Puzzle2dNode>,
}

/// 🧩 Identified-collection delta for `edges`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle2dEdgesDelta {
    pub added: Vec<Puzzle2dEdge>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle2dEdgePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle2dEdge` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dEdgePatchEntry {
    pub id: String,
    pub patch: Puzzle2dEdgePatch,
}

/// 🩹 Sparse patch over `Puzzle2dEdge` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle2dEdgePatch {
    pub replacement: Option<Puzzle2dEdge>,
}

//#endregion 🔖️DeltaHelpers

