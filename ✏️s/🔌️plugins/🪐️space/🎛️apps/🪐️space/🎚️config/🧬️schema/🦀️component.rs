//! 🧬️ schema leaf
use super::SpaceWindowCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.space.space.config")]
pub struct SpaceConfig {
    #[state(local_ui)] pub camera: BTreeMap<String, SpaceWindowCamera>,
    #[state(local_ui)] pub selected_node_ids: Vec<String>,
    #[state(local_ui)] pub hovered_node_id: Option<String>,
    #[state(local_ui)] pub collapsed_node_ids: Vec<String>,
    #[state(local_ui)] pub preview_off_node_ids: Vec<String>,
    #[state(local_ui)] pub active_node_id: Option<String>,
    #[state(local_ui)] pub focused_node_id: Option<String>,
    #[state(local_ui)] pub clipboard_node_ids: Vec<String>,
    #[state(local_ui)] pub workflow_engagement_input: String,
    #[state(local_ui)] pub compiled_dag_engagement_input: String,
    #[state(local_ui)] pub pending_import_node_id: Option<String>,
    #[state(local_ui)] pub pending_import_format: Option<String>,
    #[state(local_ui)] pub active_panel_tab: String,
    #[state(local_ui)] pub space_id: Option<String>,
    #[state(local_ui)] pub client_id: Option<String>,
    #[state(local_ui)] pub client_name: Option<String>,
    #[state(local_ui)] pub locale: String,
}

//region 📎 App-schema self-registration
/// 📎 Registers the `s.space.space` app-schema descriptor (config + presence facets) into the open
/// [`::schema::AppSchemaRegistry`], mirroring the transplanted-from-framework closed-catalog entry —
/// see `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs::register_all_app_schema_descriptors()`.
pub fn register_app_schema() {
    ::schema::register_app_schema_descriptor(::schema::AppSchemaDescriptor {
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
    });
}
//endregion 📎 App-schema self-registration

