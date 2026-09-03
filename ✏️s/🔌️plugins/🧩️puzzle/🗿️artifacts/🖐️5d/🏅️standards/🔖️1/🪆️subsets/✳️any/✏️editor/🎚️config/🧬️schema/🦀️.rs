//! 🧬️ schema leaf
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct WorldSunConfig {
    pub enabled: bool,
    pub azimuth: f64,
    pub elevation: f64,
    pub intensity: f64,
    pub color: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCamera2d {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCamera3d {
    pub position: [f64; 3],
    pub target: [f64; 3],
    pub zoom: f64,
}

#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle5d.config")]
pub struct Puzzle5dConfig {
    #[state(config)]
    pub camera2d: Puzzle5dCamera2d,
    #[state(config)]
    pub camera3d: Puzzle5dCamera3d,
    #[state(config)]
    pub fill_count: u32,
    #[state(config)]
    pub brush_candidate_index: usize,
    #[state(config)]
    pub overlap_budget: f64,
    #[state(config)]
    pub lod_mode: String,
    #[state(config)]
    pub suggestion_offset: f64,
    #[state(config)]
    pub grid_snap_enabled: bool,
    #[state(config)]
    pub grid_factor: f64,
    #[state(config)]
    pub engagement_input_by_window: BTreeMap<String, String>,
    #[state(config)]
    pub object_kind_weights: HashMap<String, f64>,
    #[state(config)]
    pub vortex_kind_weights: HashMap<String, f64>,
    #[state(config)]
    pub sun: WorldSunConfig,
    #[state(config)]
    pub active_utility_by_window_id: BTreeMap<String, String>,
    #[state(config)]
    pub terminology: String,
    #[state(config)]
    pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 `s.puzzle.puzzle5d`'s config+presence schema descriptor — returned, not self-registered
/// (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d, replacing the old self-registering
/// `register_app_schema()` this file used to export); `ArtifactApp::app_schema` (on
/// `Puzzle5dPlayApp`) hands it to `register_document_app` for registration, exactly like
/// `🗒️note`'s own `app_schema_descriptor()`.
pub fn app_schema_descriptor() -> artifact_schema::AppSchemaDescriptor {
    artifact_schema::AppSchemaDescriptor {
        id: "s.puzzle.puzzle5d",
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
