//! 🧬️ Jack snapshot schema — persistent fields only.

use crate::artifacts::jack::{Camera, Edge, Manifest, Node};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted trinity graph document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.jack")]
pub struct JackSnapshot {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub name: String,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<String>,
    #[state(persistent)]
    #[serde(default)]
    pub manifest: Manifest,
    #[state(persistent)] pub camera: Camera,
    #[state(persistent)] pub nodes: Vec<Node>,
    #[state(persistent)] pub edges: Vec<Edge>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_node_id: Option<String>,
}
//#endregion 🔖️Snapshot

impl Default for JackSnapshot {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::jack::TRINITY_GRAPH_SCHEMA.into(),
            name: String::new(),
            manifest_id: None,
            manifest: Manifest::default(),
            camera: Camera::default(),
            nodes: Vec::new(),
            edges: Vec::new(),
            root_node_id: None,
        }
    }
}
