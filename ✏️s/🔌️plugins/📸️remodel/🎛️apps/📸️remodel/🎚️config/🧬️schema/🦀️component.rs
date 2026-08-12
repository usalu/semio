//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel.remodelworldcamera")]
pub struct RemodelWorldCamera {
    #[state(local_ui)] pub position: [f64; 3],
    #[state(local_ui)] pub target: [f64; 3],
    #[state(local_ui)] pub fov: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel.remodelselection")]
pub struct RemodelSelection {
    #[state(local_ui)] pub mode: String,
    #[state(local_ui)] pub ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel.remodellayervisibility")]
pub struct RemodelLayerVisibility {
    #[state(local_ui)] pub mesh: bool,
    #[state(local_ui)] pub dense: bool,
    #[state(local_ui)] pub sparse: bool,
    #[state(local_ui)] pub cameras: bool,
    #[state(local_ui)] pub gcps: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel.remodelframecursor")]
pub struct RemodelFrameCursor {
    #[state(local_ui)] pub stream_id: Option<String>,
    #[state(local_ui)] pub frame_index: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.remodel.remodel.config")]
pub struct RemodelConfig {
    #[state(local_ui)] pub camera: RemodelWorldCamera,
    #[state(local_ui)] pub selection: RemodelSelection,
    #[state(local_ui)] pub layers: RemodelLayerVisibility,
    #[state(local_ui)] pub frame_cursor: RemodelFrameCursor,
    #[state(local_ui)] pub report_table: String,
    #[state(local_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub locale: String,
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

