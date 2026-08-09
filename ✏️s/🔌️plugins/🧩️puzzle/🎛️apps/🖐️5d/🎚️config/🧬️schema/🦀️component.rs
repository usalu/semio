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
pub struct Puzzle5dCamera2d {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCamera3d {
    pub position: [f64; 3],
    pub target: [f64; 3],
    pub zoom: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dSelection {
    pub part_ids: SelectionSet,
    pub grip_ids: SelectionSet,
    pub fastener_ids: SelectionSet,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle5d.config")]
pub struct Puzzle5dConfig {
    #[state(local_ui)] pub camera2d: Puzzle5dCamera2d,
    #[state(local_ui)] pub camera3d: Puzzle5dCamera3d,
    #[state(local_ui)] pub selection: Puzzle5dSelection,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub hovered_part_id: Option<String>,
    #[state(local_ui)] pub fill_count: u32,
    #[state(local_ui)] pub brush_candidate_index: usize,
    #[state(local_ui)] pub overlap_budget: f64,
    #[state(local_ui)] pub lod_mode: String,
    #[state(local_ui)] pub suggestion_offset: f64,
    #[state(local_ui)] pub grid_snap_enabled: bool,
    #[state(local_ui)] pub grid_factor: f64,
    #[state(local_ui)] pub engagement_input_by_window: BTreeMap<String, String>,
    #[state(local_ui)] pub object_kind_weights: HashMap<String, f64>,
    #[state(local_ui)] pub vortex_kind_weights: HashMap<String, f64>,
    #[state(local_ui)] pub sun: WorldSunConfig,
    #[state(local_ui)] pub active_utility_by_window_id: BTreeMap<String, String>,
    #[state(local_ui)] pub terminology: String,
    #[state(local_ui)] pub locale: String,
}

//#region 🔖️AppSchemaDescriptor
/// 📚 Handcrafted app schema descriptor for this owner (config + presence facets).
pub fn app_schema_descriptor() -> schema::AppSchemaDescriptor {
    schema::AppSchemaDescriptor {
        id: "s.puzzle.puzzle5d",
        config: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        presence: schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️component.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️component.proto"),
        },
    }
}

/// 📎 Registers this owner's app schema into the OS-wide catalog.
pub fn register_app_schema() {
    schema::register_app_schema_descriptor(app_schema_descriptor());
}
//#endregion 🔖️AppSchemaDescriptor
