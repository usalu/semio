//! 🧬️ Draw diff schema — sparse field delta over the artifact.

use crate::artifacts::draw::{DrawArtboard, DrawImageAsset, DrawLayerNode};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the draw artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.draw.draw")]
pub struct DrawDiff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::draw::schema::DrawArtifact>>,
    #[state(persistent)] pub schema: Option<String>,
    #[state(persistent)] pub id: Option<String>,
    #[state(persistent)] pub title: Option<Option<String>>,
    #[state(persistent)] pub layers: Option<DrawLayersDelta>,
    #[state(persistent)] pub assets: Option<DrawAssetsDelta>,
    #[state(persistent)] pub artboard: Option<Option<DrawArtboard>>,
    #[state(shared_ui)] pub selected_ids: Option<DrawStringList>,
    #[state(shared_ui)] pub active_utility_id: Option<String>,
    #[state(local_ui)] pub engagement_input: Option<String>,
    #[state(local_ui)] pub camera_x: Option<f64>,
    #[state(local_ui)] pub camera_y: Option<f64>,
    #[state(local_ui)] pub camera_zoom: Option<f64>,
    #[state(local_ui)] pub locale: Option<String>,
    #[state(preview)] pub hovered_id: Option<Option<String>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🗂️ Asset-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DrawAssetsDelta {
    pub entries: BTreeMap<String, Option<DrawImageAsset>>,
}

/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DrawStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `layers`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DrawLayersDelta {
    pub added: Vec<DrawLayerAddition>,
    pub removed: Vec<String>,
    pub patched: Vec<DrawLayerPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// ➕️ One inserted layer with its real target location (parent-aware — a bare `Vec<DrawLayerNode>`
/// can only ever describe a root-level append, which silently dropped nested `create`/`reorder`
/// targets into group children; `create-layer`/`reorder-layer`'s handcrafted diffs need the real
/// address to stay sparse instead of falling back to a whole-snapshot capture).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerAddition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub index: usize,
    pub layer: DrawLayerNode,
}

/// 🩹 One patched layer entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerPatchEntry {
    pub id: String,
    pub patch: DrawLayerPatch,
}

/// 🩹 Sparse layer field patch (JSON blobs for complex nested values).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DrawLayerPatch {
    pub visible: Option<bool>,
    pub locked: Option<bool>,
    pub name: Option<String>,
    pub opacity: Option<f64>,
    pub blend_mode: Option<String>,
    pub transform_json: Option<String>,
    pub fill_json: Option<String>,
    pub stroke_json: Option<String>,
    pub boolean_operation: Option<String>,
    pub trace_params_json: Option<String>,
    pub layer_json: Option<String>,
}
//#endregion 🔖️DeltaHelpers
