//! 🧬️ schema leaf
use crate::apps::space::config::SpaceWindowCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.space.space.presence")]
pub struct SpacePresence {
    #[state(shared_ui)] pub selected_node_ids: Vec<String>,
    #[state(shared_ui)] pub hovered_node_id: Option<String>,
    #[state(shared_ui)] pub camera: BTreeMap<String, SpaceWindowCamera>,
    #[state(shared_ui)] pub active_node_id: Option<String>,
    #[state(shared_ui)] pub focused_node_id: Option<String>,
    #[state(shared_ui)] pub collapsed_node_ids: Vec<String>,
    #[state(shared_ui)] pub preview_off_node_ids: Vec<String>,
}
