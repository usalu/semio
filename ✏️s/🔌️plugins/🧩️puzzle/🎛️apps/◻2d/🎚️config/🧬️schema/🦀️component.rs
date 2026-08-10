//! 🧬️ schema leaf
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle2d.config")]
pub struct Puzzle2dConfig {
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub camera_x: f64,
    #[state(local_ui)] pub camera_y: f64,
    #[state(local_ui)] pub camera_zoom: f64,
    #[state(local_ui)] pub lod_mode_by_pane: BTreeMap<String, String>,
    #[state(local_ui)] pub engagement_input_by_pane: BTreeMap<String, String>,
    #[state(local_ui)] pub brush_candidate_index: usize,
    #[state(local_ui)] pub brush_candidates: Vec<Value>,
    #[state(local_ui)] pub brush_candidate_source_handle_id: String,
    #[state(local_ui)] pub fill_count: u32,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub grid_snap_enabled: bool,
    #[state(local_ui)] pub grid_factor: f64,
    #[state(local_ui)] pub suggestion_offset: f64,
    #[state(local_ui)] pub node_kind_weights: BTreeMap<String, f64>,
    #[state(local_ui)] pub handle_kind_weights: BTreeMap<String, f64>,
    #[state(local_ui)] pub active_utility_by_window_id: BTreeMap<String, String>,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub terminology: String,
}

