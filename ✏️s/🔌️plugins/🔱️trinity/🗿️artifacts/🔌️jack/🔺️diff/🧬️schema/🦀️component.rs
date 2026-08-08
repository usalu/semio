//! 🧬️ Jack diff schema — sparse field delta over the artifact.

use crate::artifacts::jack::schema::JackEditorSelection;
use crate::artifacts::jack::{Camera, Edge, Node};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the jack artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.trinity.jack")]
pub struct JackDiff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::jack::schema::JackArtifact>>,
    #[state(persistent)] pub schema: Option<String>,
    #[state(persistent)] pub name: Option<String>,
    #[state(persistent)] pub manifest_id: Option<Option<String>>,
    #[state(persistent)] pub manifest: Option<crate::artifacts::jack::Manifest>,
    #[state(persistent)] pub camera: Option<Camera>,
    #[state(persistent)] pub nodes: Option<JackNodesDelta>,
    #[state(persistent)] pub edges: Option<JackEdgesDelta>,
    #[state(persistent)] pub root_node_id: Option<Option<String>>,
    #[state(shared_ui)] pub selected_node_ids: Option<JackStringList>,
    #[state(shared_ui)] pub active_fixture_id: Option<String>,
    #[state(shared_ui)] pub jack_query: Option<String>,
    #[state(shared_ui)] pub lod_mode_by_window: Option<BTreeMap<String, Option<String>>>,
    #[state(local_ui)] pub viewport_camera: Option<Camera>,
    #[state(local_ui)] pub jack_result_json: Option<String>,
    #[state(local_ui)] pub editor_engagement_input: Option<String>,
    #[state(local_ui)] pub graph_engagement_input: Option<String>,
    #[state(local_ui)] pub results_engagement_input: Option<String>,
    #[state(local_ui)] pub reorganize_epoch: Option<u64>,
    #[state(local_ui)] pub editor_selection: Option<Option<JackEditorSelection>>,
    #[state(local_ui)] pub revision: Option<u64>,
    #[state(local_ui)] pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct JackStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `nodes`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct JackNodesDelta {
    pub added: Vec<Node>,
    pub removed: Vec<String>,
    pub patched: Vec<JackNodePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched node entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JackNodePatchEntry {
    pub id: String,
    pub patch: JackNodePatch,
}

/// 🩹 Node geometry/name patch.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct JackNodePatch {
    pub name: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

/// 🧩 Identified-collection delta for `edges`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct JackEdgesDelta {
    pub added: Vec<Edge>,
    pub removed: Vec<String>,
    pub patched: Vec<JackEdgePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched edge entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JackEdgePatchEntry {
    pub id: String,
    pub patch: JackEdgePatch,
}

/// 🩹 Edge property patch (key cleared when value is null).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct JackEdgePatch {
    pub key: Option<String>,
    pub value_json: Option<Option<String>>,
}
//#endregion 🔖️DeltaHelpers
