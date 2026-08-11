//! 🧬️ schema leaf
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionSet {
    pub ids: Vec<String>,
}

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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dSelection {
    pub object_ids: SelectionSet,
    pub vortex_ids: SelectionSet,
    pub attraction_ids: SelectionSet,
    pub target_volume_ids: SelectionSet,
    pub reference_ids: SelectionSet,
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
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dWindowOptions {
    pub selection_method: String,
    pub lod_automatic: bool,
    pub lod_depth_variable: bool,
    pub grid_visible: bool,
    pub lod_manual: f64,
    pub grid_snap_enabled: bool,
    pub grid_spacing: f64,
    pub selectable_kinds: Puzzle3dSelectableKinds,
    pub engagement_input: String,
    pub selection_mode_default: String,
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
    #[state(local_ui)] pub selection: Puzzle3dSelection,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub hovered_object_id: Option<String>,
    #[state(local_ui)] pub hovered_vortex_full_id: Option<String>,
    #[state(local_ui)] pub suggestion_menu: Option<Puzzle3dSuggestionMenu>,
    #[state(local_ui)] pub overlap_budget: f64,
    #[state(local_ui)] pub fill_count: u32,
    #[state(local_ui)] pub brush_candidate_index: usize,
    #[state(local_ui)] pub object_kind_weights: HashMap<String, f64>,
    #[state(local_ui)] pub vortex_kind_weights: HashMap<String, f64>,
    #[state(local_ui)] pub lod_automatic: bool,
    #[state(local_ui)] pub lod_depth_variable: bool,
    #[state(local_ui)] pub grid_visible: bool,
    #[state(local_ui)] pub lod_manual: f64,
    #[state(local_ui)] pub grid_snap_enabled: bool,
    #[state(local_ui)] pub grid_spacing: f64,
    #[state(local_ui)] pub selectable_kinds: Puzzle3dSelectableKinds,
    #[state(local_ui)] pub hovered_kind_id: Option<String>,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub selection_mode_default: String,
    #[state(local_ui)] pub proximity_radius: f64,
    #[state(local_ui)] pub chunk_size: f64,
    #[state(local_ui)] pub voxel_dims: [u32; 3],
    #[state(local_ui)] pub transform_move: bool,
    #[state(local_ui)] pub transform_rotate: bool,
    #[state(local_ui)] pub vortex_show: String,
    #[state(local_ui)] pub vortex_direction: String,
    #[state(local_ui)] pub sun: WorldSunConfig,
    #[state(local_ui)] pub camera: Puzzle3dCamera,
    #[state(local_ui)] pub window_options: BTreeMap<String, Puzzle3dWindowOptions>,
    #[state(local_ui)] pub active_utility_by_window_id: BTreeMap<String, String>,
    #[state(local_ui)] pub active_tool_id: Option<String>,
    #[state(local_ui)] pub terminology: String,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub window_ids: Vec<String>,
}

//region 📎 App-schema self-registration
/// 📎 Registers the `s.puzzle.puzzle3d` app-schema descriptor (config + presence facets) into the
/// open [`artifact_schema::AppSchemaRegistry`], mirroring the transplanted-from-framework
/// closed-catalog entry — see
/// `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs::register_all_app_schema_descriptors()`.
pub fn register_app_schema() {
    artifact_schema::register_app_schema_descriptor(artifact_schema::AppSchemaDescriptor {
        id: "s.puzzle.puzzle3d",
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
    });
}
//endregion 📎 App-schema self-registration

