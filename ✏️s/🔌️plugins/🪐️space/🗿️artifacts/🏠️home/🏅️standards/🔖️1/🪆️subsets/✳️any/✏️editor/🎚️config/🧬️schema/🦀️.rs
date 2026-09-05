//! 🧬️ schema leaf
use schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.space.home.config")]
pub struct HomeConfig {
    #[state(config)]
    pub active_panel_tab: String,
    #[state(config)]
    pub locale: String,
    #[state(config)]
    pub directory_json: String,
    #[state(config)]
    pub directory_session_binding_sha256: String,
    #[state(config)]
    pub directory_authorization_generation: u64,
    #[state(config)]
    pub directory_receipt_sha256: String,
    #[state(config)]
    pub client_id: String,
    #[state(config)]
    pub client_name: String,
}

//region 📎 App-schema descriptor
/// 📎 The `s.space.home` app-schema descriptor (config + presence facets) — returned, not
/// self-registered; `ArtifactEditor::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// W1c) hands it to `register_document_app` for registration, mirroring the `🗒️note` pattern.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.space.home",
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
