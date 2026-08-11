//! 🧬️ schema leaf
use crate::artifacts::block3d::{Block3dBrushPreview, Block3dWindowView};
use crate::BlockCamera3d;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.block.3d.config")]
pub struct Block3dConfig {
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub active_representation_id: Option<String>,
    #[state(local_ui)] pub wanted_tags: Vec<String>,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub windows: Vec<Block3dWindowView>,
    #[state(local_ui)] pub brush_vortex_kind_id: Option<String>,
    #[state(local_ui)] pub brush_radius: f64,
    #[state(local_ui)] pub brush_flip: bool,
    #[state(local_ui)] pub brush_preview: Option<Block3dBrushPreview>,
    #[state(local_ui)] pub camera: Option<BlockCamera3d>,
    #[state(local_ui)] pub hovered_vortex_full_id: Option<String>,
}

//region 📎 App-schema self-registration
/// 📎 Self-registers this app's schema descriptor into the open `AppSchemaRegistry`, mirroring the
/// same construction the framework's closed catalog previously hardcoded for `s.block.3d`.
pub fn register_app_schema() {
    ::schema::register_app_schema_descriptor(::schema::AppSchemaDescriptor {
        id: "s.block.3d",
        config: ::schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        presence: ::schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️component.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️component.proto"),
        },
    });
}
//endregion 📎 App-schema self-registration

