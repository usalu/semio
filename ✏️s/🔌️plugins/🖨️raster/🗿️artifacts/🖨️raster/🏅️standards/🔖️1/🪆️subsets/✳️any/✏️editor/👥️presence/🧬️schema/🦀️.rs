//! 🧬️ schema leaf
use schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.raster.raster.presence")]
pub struct RasterPresence {
    #[state(presence)]
    pub brush_size: f64,
    #[state(presence)]
    pub brush_opacity: f64,
    #[state(presence)]
    pub camera: RasterPresenceCamera,
    #[state(presence)]
    pub active_utility_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.raster.raster.rasterpresencecamera")]
pub struct RasterPresenceCamera {
    #[state(presence)]
    pub x: f64,
    #[state(presence)]
    pub y: f64,
    #[state(presence)]
    pub zoom: f64,
}
