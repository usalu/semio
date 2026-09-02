//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.raster.raster.config")]
pub struct RasterConfig {
    #[state(config)]
    pub brush_size: f64,
    #[state(config)]
    pub brush_opacity: f64,
    #[state(config)]
    pub composite_viewport: Option<RasterConfigViewportSize>,
    #[state(config)]
    pub camera: RasterCamera,
    #[state(config)]
    pub active_utility_id: String,
    #[state(config)]
    pub locale: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.raster.raster.rastercamera")]
pub struct RasterCamera {
    #[state(config)]
    pub x: f64,
    #[state(config)]
    pub y: f64,
    #[state(config)]
    pub zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.raster.raster.rasterconfigviewportsize")]
pub struct RasterConfigViewportSize {
    #[state(config)]
    pub width: f64,
    #[state(config)]
    pub height: f64,
}

//region 📎 App-schema descriptor
/// 📎 `s.raster.raster`'s config+presence schema descriptor — returned, not self-registered;
/// `ArtifactEditor::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) hands it to
/// `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.raster.raster",
        config: ::schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        presence: ::schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
//endregion 📎 App-schema descriptor
