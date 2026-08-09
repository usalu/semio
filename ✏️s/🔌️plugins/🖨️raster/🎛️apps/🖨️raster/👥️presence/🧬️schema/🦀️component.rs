//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.raster.raster.presence")]
pub struct RasterPresence {
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub hovered_id: Option<String>,
    #[state(shared_ui)] pub brush_size: f64,
    #[state(shared_ui)] pub brush_opacity: f64,
    #[state(shared_ui)] pub camera: RasterPresenceCamera,
    #[state(shared_ui)] pub active_utility_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.raster.raster.rasterpresencecamera")]
pub struct RasterPresenceCamera {
    #[state(shared_ui)] pub x: f64,
    #[state(shared_ui)] pub y: f64,
    #[state(shared_ui)] pub zoom: f64,
}
