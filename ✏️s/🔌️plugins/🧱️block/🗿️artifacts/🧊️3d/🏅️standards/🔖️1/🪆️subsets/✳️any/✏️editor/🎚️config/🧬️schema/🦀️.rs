//! 🧬️ schema leaf
use crate::artifacts::block3d::{Block3dBrushPreview, Block3dWindowView};
use crate::BlockCamera3d;
use schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.block.3d.config")]
pub struct Block3dConfig {
    #[state(config)]
    pub active_representation_id: Option<String>,
    #[state(config)]
    pub wanted_tags: Vec<String>,
    #[state(config)]
    pub locale: String,
    #[state(config)]
    pub windows: Vec<Block3dWindowView>,
    #[state(config)]
    pub brush_vortex_kind_id: Option<String>,
    #[state(config)]
    pub brush_radius: f64,
    #[state(config)]
    pub brush_flip: bool,
    #[state(config)]
    pub brush_preview: Option<Block3dBrushPreview>,
    #[state(config)]
    pub camera: Option<BlockCamera3d>,
}

//region 📎 App-schema descriptor
/// 📎 `s.block.3d`'s config+presence schema descriptor — returned, not self-registered; `ArtifactEditor::app_schema`
/// (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) hands it to `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.block.3d",
        config: ::schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        presence: ::schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
//endregion 📎 App-schema descriptor
