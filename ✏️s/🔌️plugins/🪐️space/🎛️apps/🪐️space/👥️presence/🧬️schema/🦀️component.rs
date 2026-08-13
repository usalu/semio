//! 🧬️ schema leaf
use crate::apps::space::config::SpaceWindowCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.space.space.presence")]
pub struct SpacePresence {
    #[state(presence)] pub selected_node_ids: Vec<String>,
    #[state(presence)] pub hovered_node_id: Option<String>,
    #[state(presence)] pub camera: BTreeMap<String, SpaceWindowCamera>,
    #[state(presence)] pub active_node_id: Option<String>,
    #[state(presence)] pub focused_node_id: Option<String>,
    #[state(presence)] pub collapsed_node_ids: Vec<String>,
    #[state(presence)] pub preview_off_node_ids: Vec<String>,
}
