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
    #[state(local_ui)]
    pub selected_ids: Vec<String>,
    #[state(local_ui)]
    pub hovered_id: Option<String>,
    #[state(local_ui)]
    pub engagement_input: String,
    #[state(local_ui)]
    pub camera: DrawCamera,
    #[state(local_ui)]
    pub active_utility_id: String,
    #[state(local_ui)]
    pub locale: String,
}
//#endregion 🔖️Config

