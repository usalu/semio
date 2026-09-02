//! 🧬️ schema leaf
use super::SpaceWindowCamera;
use schema::ArtifactSchema;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.space.space.config")]
pub struct SpaceConfig {
    #[state(config)]
    pub camera: BTreeMap<String, SpaceWindowCamera>,
    #[state(config)]
    pub collapsed_node_ids: Vec<String>,
    #[state(config)]
    pub preview_off_node_ids: Vec<String>,
    #[state(config)]
    pub active_node_id: Option<String>,
    #[state(config)]
    pub focused_node_id: Option<String>,
    #[state(config)]
    pub clipboard_node_ids: Vec<String>,
    #[state(config)]
    pub workflow_engagement_input: String,
    #[state(config)]
    pub compiled_dag_engagement_input: String,
    #[state(config)]
    pub pending_import_node_id: Option<String>,
    #[state(config)]
    pub pending_import_format: Option<String>,
    #[state(config)]
    pub active_panel_tab: String,
    #[state(config)]
    pub space_id: Option<String>,
    #[state(config)]
    pub client_id: Option<String>,
    #[state(config)]
    pub client_name: Option<String>,
    #[state(config)]
    pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 The `s.space.space` app-schema descriptor (config + presence facets) — returned, not
/// self-registered; `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// W1c) hands it to `register_document_app` for registration, mirroring the `🗒️note` pattern.
pub async fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.space.space",
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
