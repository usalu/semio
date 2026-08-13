//! 🧬️ schema leaf
use crate::artifacts::jack::Camera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.trinity.rewrite.presence")]
pub struct RewritePresence {
    #[state(presence)] pub selected_node_ids: Vec<String>,
    #[state(presence)] pub active_hover_var: String,
    #[state(presence)] pub active_select_var: String,
    #[state(presence)] pub before_pane_camera: Camera,
    #[state(presence)] pub lod_mode_by_window: BTreeMap<String, String>,
}
