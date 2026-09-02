//! 🧬️ schema leaf
use crate::artifacts::jack::Camera;
use schema::ArtifactSchema;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.rewrite.config")]
pub struct RewriteConfig {
    #[state(config)]
    pub before_pane_camera: Camera,
    #[state(config)]
    pub reorganize_epoch: u64,
    #[state(config)]
    pub lod_mode_by_window: BTreeMap<String, String>,
    #[state(config)]
    pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 `s.trinity.rewrite`'s config+presence schema descriptor — returned, not self-registered;
/// `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) hands it to
/// `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.trinity.rewrite",
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
