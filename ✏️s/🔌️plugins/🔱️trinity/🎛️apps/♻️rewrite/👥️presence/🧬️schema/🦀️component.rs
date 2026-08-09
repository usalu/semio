//! 🧬️ schema leaf
use crate::artifacts::jack::Camera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.trinity.rewrite.presence")]
pub struct RewritePresence {
    #[state(shared_ui)] pub selected_node_ids: Vec<String>,
    #[state(shared_ui)] pub active_hover_var: String,
    #[state(shared_ui)] pub active_select_var: String,
    #[state(shared_ui)] pub before_pane_camera: Camera,
    #[state(shared_ui)] pub lod_mode_by_window: BTreeMap<String, String>,
}
