//! 🧬️ schema leaf
use crate::engine::space::config::SpaceWindowCamera;
use schema::ArtifactSchema;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.space.space.presence")]
pub struct SpacePresence {
    #[state(presence)]
    pub camera: BTreeMap<String, SpaceWindowCamera>,
    #[state(presence)]
    pub active_node_id: Option<String>,
    #[state(presence)]
    pub focused_node_id: Option<String>,
    #[state(presence)]
    pub collapsed_node_ids: Vec<String>,
    #[state(presence)]
    pub preview_off_node_ids: Vec<String>,
}
