//! 🧬️ Drawing app config schema — every local-ui field of DrawingConfig.

use crate::artifacts::drawing::DrawingCamera;
use schema::ArtifactSchema;

//#region 🔖️Config
/// 🎚️ Drawing app config — unshared local app state.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.draw.drawing.config")]
pub struct DrawingConfig {
    #[state(config)]
    pub engagement_input: String,
    #[state(config)]
    pub camera: DrawingCamera,
    #[state(config)]
    pub active_utility_id: String,
    #[state(config)]
    pub trace_pointer_generation: u64,
    #[state(config)]
    pub trace_pointer_completed_work: u64,
    #[state(config)]
    pub trace_pointer_pending_work: u64,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Config

//#region 🔖️AppSchemaDescriptor
/// 📎 The `s.draw.drawing` app-schema descriptor (config + presence facets) — returned, not
/// self-registered; `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// W1c) hands it to `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.draw.drawing",
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
//#endregion 🔖️AppSchemaDescriptor
