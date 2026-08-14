//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel.remodelworldcamera")]
pub struct RemodelWorldCamera {
    #[state(config)] pub position: [f64; 3],
    #[state(config)] pub target: [f64; 3],
    #[state(config)] pub fov: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel.remodellayervisibility")]
pub struct RemodelLayerVisibility {
    #[state(config)] pub mesh: bool,
    #[state(config)] pub dense: bool,
    #[state(config)] pub sparse: bool,
    #[state(config)] pub cameras: bool,
    #[state(config)] pub gcps: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel.remodelframecursor")]
pub struct RemodelFrameCursor {
    #[state(config)] pub stream_id: Option<String>,
    #[state(config)] pub frame_index: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel.config")]
pub struct RemodelConfig {
    #[state(config)] pub camera: RemodelWorldCamera,
    #[state(config)] pub layers: RemodelLayerVisibility,
    #[state(config)] pub frame_cursor: RemodelFrameCursor,
    #[state(config)] pub report_table: String,
    #[state(config)] pub active_utility_id: String,
    #[state(config)] pub locale: String,
}

//region 📎 App-schema descriptor
/// 📎 The `s.remodel.remodel.remodelworldcamera` app-schema descriptor (config + presence facets) —
/// returned, not self-registered; `ArtifactApp::app_schema` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) hands it to `register_document_app` for
/// registration.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.remodel.remodel.remodelworldcamera",
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

