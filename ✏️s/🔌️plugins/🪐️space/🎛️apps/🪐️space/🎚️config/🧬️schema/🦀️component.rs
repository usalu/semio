//! 🧬️ schema leaf
use super::SpaceWindowCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.space.space.config")]
pub struct SpaceConfig {
    #[state(config)] pub camera: BTreeMap<String, SpaceWindowCamera>,
    #[state(config)] pub selected_node_ids: Vec<String>,
    #[state(config)] pub hovered_node_id: Option<String>,
    #[state(config)] pub collapsed_node_ids: Vec<String>,
    #[state(config)] pub preview_off_node_ids: Vec<String>,
    #[state(config)] pub active_node_id: Option<String>,
    #[state(config)] pub focused_node_id: Option<String>,
    #[state(config)] pub clipboard_node_ids: Vec<String>,
    #[state(config)] pub workflow_engagement_input: String,
    #[state(config)] pub compiled_dag_engagement_input: String,
    #[state(config)] pub pending_import_node_id: Option<String>,
    #[state(config)] pub pending_import_format: Option<String>,
    #[state(config)] pub active_panel_tab: String,
    #[state(config)] pub space_id: Option<String>,
    #[state(config)] pub client_id: Option<String>,
    #[state(config)] pub client_name: Option<String>,
    #[state(config)] pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 The `s.space.space` app-schema descriptor (config + presence facets) — returned, not
/// self-registered; `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// W1c) hands it to `register_document_app` for registration, mirroring the `🗒️note` pattern.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.space.space",
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

