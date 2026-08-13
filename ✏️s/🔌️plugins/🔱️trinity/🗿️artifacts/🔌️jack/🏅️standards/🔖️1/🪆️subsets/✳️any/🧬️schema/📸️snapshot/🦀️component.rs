//! 🧬️ Jack snapshot schema — artifact-lane fields only.

use crate::artifacts::jack::{Camera, Edge, Manifest, Node};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted trinity graph document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.jack")]
pub struct JackSnapshot {
    #[state(artifact)] pub schema: String,
    #[state(artifact)] pub name: String,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<String>,
    #[state(artifact)]
    #[serde(default)]
    pub manifest: Manifest,
    #[state(artifact)] pub camera: Camera,
    #[state(artifact)] pub nodes: Vec<Node>,
    #[state(artifact)] pub edges: Vec<Edge>,
    #[state(artifact)]
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
