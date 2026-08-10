//! 🧬️ schema leaf
use crate::artifacts::jack::Camera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.rewrite.config")]
pub struct RewriteConfig {
    #[state(local_ui)] pub selected_node_ids: Vec<String>,
    #[state(local_ui)] pub before_pane_camera: Camera,
    #[state(local_ui)] pub reorganize_epoch: u64,
    #[state(local_ui)] pub active_hover_var: String,
    #[state(local_ui)] pub hover_epoch: u64,
    #[state(local_ui)] pub active_select_var: String,
    #[state(local_ui)] pub select_epoch: u64,
    #[state(local_ui)] pub lod_mode_by_window: BTreeMap<String, String>,
    #[state(local_ui)] pub locale: String,
}

