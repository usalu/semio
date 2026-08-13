//! 🧬️ schema leaf
use crate::artifacts::jack::Camera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.rewrite.config")]
pub struct RewriteConfig {
    #[state(config)] pub selected_node_ids: Vec<String>,
    #[state(config)] pub before_pane_camera: Camera,
    #[state(config)] pub reorganize_epoch: u64,
    #[state(config)] pub active_hover_var: String,
    #[state(config)] pub hover_epoch: u64,
    #[state(config)] pub active_select_var: String,
    #[state(config)] pub select_epoch: u64,
    #[state(config)] pub lod_mode_by_window: BTreeMap<String, String>,
    #[state(config)] pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 `s.trinity.rewrite`'s config+presence schema descriptor — returned, not self-registered;
/// `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) hands it to
/// `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
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
    }
}
//endregion 📎 App-schema descriptor

