//! 🧬️ schema leaf
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle2d.config")]
pub struct Puzzle2dConfig {
    #[state(config)]
    pub camera_x: f64,
    #[state(config)]
    pub camera_y: f64,
    #[state(config)]
    pub camera_zoom: f64,
    #[state(config)]
    pub lod_mode_by_pane: BTreeMap<String, String>,
    #[state(config)]
    pub engagement_input_by_pane: BTreeMap<String, String>,
    #[state(config)]
    pub brush_candidate_index: usize,
    #[state(config)]
    pub brush_candidates: Vec<Value>,
    #[state(config)]
    pub brush_candidate_source_handle_id: String,
    #[state(config)]
    pub fill_count: u32,
    #[state(config)]
    pub fill_job_checkpoint: Option<Vec<u8>>,
    #[state(config)]
    pub fill_job_operation: u64,
    #[state(config)]
    pub fill_job_generation: u64,
    #[state(config)]
    pub fill_job_seed: u64,
    #[state(config)]
    pub fill_job_applied_count: usize,
    #[state(config)]
    pub fill_job_preview: Option<Value>,
    #[state(config)]
    pub grid_snap_enabled: bool,
    #[state(config)]
    pub grid_factor: f64,
    #[state(config)]
    pub suggestion_offset: f64,
    #[state(config)]
    pub node_kind_weights: BTreeMap<String, f64>,
    #[state(config)]
    pub handle_kind_weights: BTreeMap<String, f64>,
    #[state(config)]
    pub active_utility_by_window_id: BTreeMap<String, String>,
    #[state(config)]
    pub locale: String,
    #[state(config)]
    pub terminology: String,
    #[state(config)]
    pub example_load_generation: u64,
    #[state(config)]
    pub example_load_id: Option<String>,
}

//region 📎 App-schema descriptor
/// 📎 `s.puzzle.puzzle2d`'s config+presence schema descriptor — returned, not self-registered
/// (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d, replacing the old self-registering
/// `register_app_schema()` this file used to export); `ArtifactApp::app_schema` (on
/// `Puzzle2dPlayApp`) hands it to `register_document_app` for registration, exactly like
/// `🗒️note`'s own `app_schema_descriptor()`.
pub fn app_schema_descriptor() -> artifact_schema::AppSchemaDescriptor {
    artifact_schema::AppSchemaDescriptor {
        id: "s.puzzle.puzzle2d",
        config: artifact_schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        presence: artifact_schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️component.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️component.proto"),
        },
    }
}
//endregion 📎 App-schema descriptor
