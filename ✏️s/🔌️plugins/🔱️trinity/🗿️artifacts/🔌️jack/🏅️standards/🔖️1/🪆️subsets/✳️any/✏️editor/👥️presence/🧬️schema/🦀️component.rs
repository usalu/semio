//! 🧬️ schema leaf
use crate::artifacts::jack::Camera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.trinity.jack.presence")]
pub struct JackPresence {
    #[state(presence)]
    pub active_fixture_id: String,
    #[state(presence)]
    pub jack_query: String,
    #[state(presence)]
    pub camera: Camera,
    #[state(presence)]
    pub lod_mode_by_window: BTreeMap<String, String>,
}
