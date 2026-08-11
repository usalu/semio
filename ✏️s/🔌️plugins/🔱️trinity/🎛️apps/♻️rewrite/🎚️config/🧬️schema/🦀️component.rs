//! 🧬️ schema leaf
use crate::artifacts::jack::Camera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.rewrite.config")]
pub struct RewriteConfig {
    #[state(local_ui)] pub selected_node_ids: Vec<String>,
    #[state(local_ui)] pub before_pane_camera: Camera,
    #[state(local_ui)] pub reorganize_epoch: u64,
    #[state(local_ui)] pub active_hover_var: String,
    #[state(local_ui)] pub hover_epoch: u64,
    #[state(local_ui)] pub active_select_var: String,
    #[state(local_ui)] pub select_epoch: u64,
    #[state(local_ui)] pub lod_mode_by_window: BTreeMap<String, String>,
    #[state(local_ui)] pub locale: String,
}

//region 📎 App-schema self-registration
/// 📎 Registers `s.trinity.rewrite`'s config+presence schema descriptor into the process-local registry.
pub fn register_app_schema() {
    ::schema::register_app_schema_descriptor(::schema::AppSchemaDescriptor {
        id: "s.trinity.rewrite",
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

