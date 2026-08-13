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
    #[state(artifact)] pub schema: Option<String>,
    #[state(artifact)] pub name: Option<String>,
    #[state(artifact)] pub manifest_id: Option<Option<String>>,
    #[state(artifact)] pub manifest: Option<crate::artifacts::jack::Manifest>,
    #[state(artifact)] pub camera: Option<Camera>,
    #[state(artifact)] pub nodes: Option<JackNodesDelta>,
    #[state(artifact)] pub edges: Option<JackEdgesDelta>,
    #[state(artifact)] pub root_node_id: Option<Option<String>>,
    #[state(presence)] pub selected_node_ids: Option<JackStringList>,
    #[state(presence)] pub active_fixture_id: Option<String>,
    #[state(presence)] pub jack_query: Option<String>,
    #[state(presence)] pub lod_mode_by_window: Option<BTreeMap<String, Option<String>>>,
    #[state(config)] pub viewport_camera: Option<Camera>,
    #[state(config)] pub jack_result_json: Option<String>,
    #[state(config)] pub editor_engagement_input: Option<String>,
    #[state(config)] pub graph_engagement_input: Option<String>,
    #[state(config)] pub results_engagement_input: Option<String>,
    #[state(config)] pub reorganize_epoch: Option<u64>,
    #[state(config)] pub editor_selection: Option<Option<JackEditorSelection>>,
    #[state(config)] pub revision: Option<u64>,
    #[state(config)] pub locale: Option<String>,
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
    pub key: Option<String>,
    pub value_json: Option<Option<String>>,
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
