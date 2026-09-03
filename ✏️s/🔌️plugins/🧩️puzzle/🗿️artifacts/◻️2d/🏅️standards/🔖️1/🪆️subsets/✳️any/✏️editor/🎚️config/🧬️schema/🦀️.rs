//! 🧬️ schema leaf
use artifact_schema::ArtifactSchema;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum Puzzle2dFillLifecycle {
    #[default]
    Idle,
    Capturing,
    Queued,
    Running,
    CheckpointReady,
    Applying,
    AwaitingAdoption,
    Closing,
    Completed,
    Cancelled,
    Faulted,
    Discarded,
}

#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
    pub brush_candidates: Vec<dsl::DslValue>,
    #[state(config)]
    pub brush_candidate_source_handle_id: String,
    #[state(config)]
    pub fill_count: u32,
    #[state(config)]
    pub fill_job_operation: u64,
    #[state(config)]
    pub fill_job_generation: u64,
    #[state(config)]
    pub fill_job_seed: u64,
    #[state(config)]
    pub fill_job_base_revision: u64,
    #[state(config)]
    pub fill_job_checkpoint_sequence: u64,
    #[state(config)]
    pub fill_job_accepted_count: u64,
    #[state(config)]
    pub fill_job_search_count: u64,
    #[state(config)]
    pub fill_job_stage: String,
    #[state(config)]
    pub fill_job_lifecycle: Puzzle2dFillLifecycle,
    #[state(config)]
    pub fill_job_fault_code: Option<String>,
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
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        presence: artifact_schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
//endregion 📎 App-schema descriptor
