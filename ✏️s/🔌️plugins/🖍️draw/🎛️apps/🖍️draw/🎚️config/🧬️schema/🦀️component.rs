//! 🧬️ Draw app config schema — every local-ui field of DrawConfig.

use crate::artifacts::draw::DrawCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🎚️ Draw app config — unshared local app state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.draw.draw.config")]
pub struct DrawConfig {
    #[state(config)]
    pub selected_ids: Vec<String>,
    #[state(config)]
    pub hovered_id: Option<String>,
    #[state(config)]
    pub engagement_input: String,
    #[state(config)]
    pub camera: DrawCamera,
    #[state(config)]
    pub active_utility_id: String,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Config

//#region 🔖️AppSchemaDescriptor
/// 📎 The `s.draw.draw` app-schema descriptor (config + presence facets) — returned, not
/// self-registered; `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// W1c) hands it to `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.draw.draw",
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
//#endregion 🔖️AppSchemaDescriptor

