//! 🧬️ schema leaf
use crate::artifacts::jack::Camera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.trinity.jack.presence")]
pub struct JackPresence {
    #[state(shared_ui)] pub selected_node_ids: Vec<String>,
    #[state(shared_ui)] pub active_fixture_id: String,
    #[state(shared_ui)] pub jack_query: String,
    #[state(shared_ui)] pub camera: Camera,
    #[state(shared_ui)] pub lod_mode_by_window: BTreeMap<String, String>,
}
