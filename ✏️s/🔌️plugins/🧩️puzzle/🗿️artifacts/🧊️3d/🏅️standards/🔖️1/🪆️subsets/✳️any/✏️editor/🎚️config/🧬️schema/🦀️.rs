//! 🧬️ schema leaf
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldSunConfig {
    pub enabled: bool,
    pub azimuth: f64,
    pub elevation: f64,
    pub intensity: f64,
    pub color: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldProjectionConfig {
    pub kind: String,
    pub orthographic_view: String,
    pub axonometric_variant: String,
    pub axonometric_angle_a: f64,
    pub axonometric_angle_b: f64,
    pub axonometric_quadrant: String,
    pub oblique_variant: String,
    pub oblique_angle: f64,
    pub oblique_depth: f64,
    pub one_point_axis: String,
    pub fov: f64,
    pub two_point_shift: f64,
    pub curvilinear_fov: f64,
    pub curvilinear_strength: f64,
    pub curvilinear_mapping: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCamera {
    pub position: [f64; 3],
    pub target: [f64; 3],
    pub zoom: f64,
    pub up: Option<[f64; 3]>,
    pub projection: WorldProjectionConfig,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dSelectableKinds {
    pub objects: bool,
    pub vortices: bool,
    pub attractions: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dSuggestionMenu {
    pub x: f64,
    pub y: f64,
    pub window_id: String,
    pub vortex_full_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dWindowOptions {
    pub lod_automatic: bool,
    pub lod_depth_variable: bool,
    pub grid_visible: bool,
    pub lod_manual: f64,
    pub grid_snap_enabled: bool,
    pub grid_spacing: f64,
    pub selectable_kinds: Puzzle3dSelectableKinds,
    pub engagement_input: String,
    pub proximity_radius: f64,
    pub chunk_size: f64,
    pub voxel_dims: [u32; 3],
    pub transform_move: bool,
    pub transform_rotate: bool,
    pub vortex_show: String,
    pub vortex_direction: String,
    pub sun: WorldSunConfig,
    pub camera: Puzzle3dCamera,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle3d.config")]
pub struct Puzzle3dConfig {
    #[state(config)]
    pub suggestion_menu: Option<Puzzle3dSuggestionMenu>,
    #[state(config)]
    pub overlap_budget: f64,
    #[state(config)]
    pub fill_count: u32,
    #[state(config)]
    pub fill_apply_generation: u64,
    #[state(config)]
    pub fill_applied_count: u32,
    #[state(config)]
    pub fill_checkpoint: Vec<u8>,
    #[state(config)]
    pub brush_candidate_index: usize,
    #[state(config)]
    pub object_kind_weights: HashMap<String, f64>,
    #[state(config)]
    pub vortex_kind_weights: HashMap<String, f64>,
    #[state(config)]
    pub lod_automatic: bool,
    #[state(config)]
    pub lod_depth_variable: bool,
    #[state(config)]
    pub grid_visible: bool,
    #[state(config)]
    pub lod_manual: f64,
    #[state(config)]
    pub grid_snap_enabled: bool,
    #[state(config)]
    pub grid_spacing: f64,
    #[state(config)]
    pub selectable_kinds: Puzzle3dSelectableKinds,
    #[state(config)]
    pub engagement_input: String,
    #[state(config)]
    pub proximity_radius: f64,
    #[state(config)]
    pub chunk_size: f64,
    #[state(config)]
    pub voxel_dims: [u32; 3],
    #[state(config)]
    pub transform_move: bool,
    #[state(config)]
    pub transform_rotate: bool,
    #[state(config)]
    pub vortex_show: String,
    #[state(config)]
    pub vortex_direction: String,
    #[state(config)]
    pub sun: WorldSunConfig,
    #[state(config)]
    pub camera: Puzzle3dCamera,
    #[state(config)]
    pub window_options: BTreeMap<String, Puzzle3dWindowOptions>,
    #[state(config)]
    pub active_utility_by_window_id: BTreeMap<String, String>,
    #[state(config)]
    pub active_tool_id: Option<String>,
    #[state(config)]
    pub terminology: String,
    #[state(config)]
    pub locale: String,
    #[state(config)]
    pub window_ids: Vec<String>,
}

//region 📎 App-schema descriptor
/// 📎 `s.puzzle.puzzle3d`'s config+presence schema descriptor — returned, not self-registered
/// (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d, replacing the old self-registering
/// `register_app_schema()` this file used to export); `ArtifactApp::app_schema` (on
/// `Puzzle3dPlayApp`) hands it to `register_document_app` for registration, exactly like
/// `🗒️note`'s own `app_schema_descriptor()`.
pub fn app_schema_descriptor() -> artifact_schema::AppSchemaDescriptor {
    artifact_schema::AppSchemaDescriptor {
        id: "s.puzzle.puzzle3d",
        config: artifact_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        presence: artifact_schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
//endregion 📎 App-schema descriptor
