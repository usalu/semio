//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodeling.remodeling.remodelingworldcamera")]
pub struct RemodelingWorldCamera {
    #[state(config)]
    pub position: [f64; 3],
    #[state(config)]
    pub target: [f64; 3],
    #[state(config)]
    pub fov: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodeling.remodeling.remodelinglayervisibility")]
pub struct RemodelingLayerVisibility {
    #[state(config)]
    pub mesh: bool,
    #[state(config)]
    pub dense: bool,
    #[state(config)]
    pub sparse: bool,
    #[state(config)]
    pub cameras: bool,
    #[state(config)]
    pub gcps: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodeling.remodeling.remodelingframecursor")]
pub struct RemodelingFrameCursor {
    #[state(config)]
    pub stream_id: Option<String>,
    #[state(config)]
    pub frame_index: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodeling.remodeling.config")]
pub struct RemodelingConfig {
    #[state(config)]
    pub camera: RemodelingWorldCamera,
    #[state(config)]
    pub layers: RemodelingLayerVisibility,
    #[state(config)]
    pub frame_cursor: RemodelingFrameCursor,
    #[state(config)]
    pub report_table: String,
    #[state(config)]
    pub active_utility_id: String,
    #[state(config)]
    pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 The `s.remodeling.remodeling.remodelingworldcamera` app-schema descriptor (config + presence facets) —
/// returned, not self-registered; `ArtifactEditor::app_schema` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) hands it to `register_document_app` for
/// registration.
pub async fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.remodeling.remodeling.remodelingworldcamera",
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
