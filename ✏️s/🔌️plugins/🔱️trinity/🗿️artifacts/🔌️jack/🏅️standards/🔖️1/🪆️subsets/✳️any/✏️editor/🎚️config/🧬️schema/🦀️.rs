//! 🧬️ schema leaf
use super::JackEditorSelection;
use crate::artifacts::jack::Camera;
use schema::ArtifactSchema;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.jack.config")]
pub struct JackConfig {
    #[state(config)]
    pub camera: Camera,
    #[state(config)]
    pub active_fixture_id: String,
    #[state(config)]
    pub jack_query: String,
    #[state(config)]
    pub jack_result_json: String,
    #[state(config)]
    pub editor_engagement_input: String,
    #[state(config)]
    pub graph_engagement_input: String,
    #[state(config)]
    pub results_engagement_input: String,
    #[state(config)]
    pub reorganize_epoch: u64,
    #[state(config)]
    pub editor_selection: Option<JackEditorSelection>,
    #[state(config)]
    pub lod_mode_by_window: BTreeMap<String, String>,
    #[state(config)]
    pub revision: u64,
    #[state(config)]
    pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 `s.trinity.jack`'s config+presence schema descriptor — returned, not self-registered;
/// `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) hands it to
/// `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.trinity.jack",
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
