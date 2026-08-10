//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.raster.raster.config")]
pub struct RasterConfig {
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub hovered_id: Option<String>,
    #[state(local_ui)] pub brush_size: f64,
    #[state(local_ui)] pub brush_opacity: f64,
    #[state(local_ui)] pub composite_viewport: Option<RasterConfigViewportSize>,
    #[state(local_ui)] pub camera: RasterCamera,
    #[state(local_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub locale: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.raster.raster.rastercamera")]
pub struct RasterCamera {
    #[state(local_ui)] pub x: f64,
    #[state(local_ui)] pub y: f64,
    #[state(local_ui)] pub zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.raster.raster.rasterconfigviewportsize")]
pub struct RasterConfigViewportSize {
    #[state(local_ui)] pub width: f64,
    #[state(local_ui)] pub height: f64,
}

