//! 🧬️ Jack snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes`/`edges` are gone from this STRUCT —
//! replaced by a single composed `content: JackContentChild` slot (`s.stdio.semio.graph`). See
//! `🗿️artifacts/🔌️jack/🦀️component.rs`'s `🔖️ContentBridge`/`🔖️WorkingScene` regions for the
//! converter/handle/cache machinery this field depends on.

use crate::artifacts::jack::{Camera, JackContentChild, Manifest};
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
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.graph")]
    pub content: JackContentChild,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_node_id: Option<String>,
}
//#endregion 🔖️Snapshot

impl Default for JackSnapshot {
    async fn default() -> Self {
        Self {
            schema: crate::artifacts::jack::TRINITY_GRAPH_SCHEMA.into(),
            name: String::new(),
            manifest_id: None,
            manifest: Manifest::default(),
            camera: Camera::default(),
            content: crate::artifacts::jack::jack_content_child_handle_and_cache(Vec::new(), Vec::new()),
            root_node_id: None,
        }
    }
}
