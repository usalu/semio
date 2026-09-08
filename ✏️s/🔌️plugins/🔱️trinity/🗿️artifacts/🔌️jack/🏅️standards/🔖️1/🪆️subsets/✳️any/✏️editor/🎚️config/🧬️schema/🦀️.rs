//! 🧬️ schema leaf
use crate::Camera;
use ::semio_framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.jack.config")]
pub struct JackConfig {
    #[state(config)]
    pub camera: Camera,
    #[state(config)]
    pub jack_query: String,
    #[state(config)]
    pub lod_mode_by_window: BTreeMap<String, String>,
}

//region 📎 App-schema descriptor
/// 📎 `s.trinity.jack`'s config schema descriptor — returned, not self-registered;
/// `ArtifactApp::app_schema` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) hands it to
/// `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::semio_framework_schema::AppSchemaDescriptor {
    ::semio_framework_schema::AppSchemaDescriptor {
        id: "s.trinity.jack",
        config: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        presence: ::semio_framework_schema::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },
    }
}
//endregion 📎 App-schema descriptor
