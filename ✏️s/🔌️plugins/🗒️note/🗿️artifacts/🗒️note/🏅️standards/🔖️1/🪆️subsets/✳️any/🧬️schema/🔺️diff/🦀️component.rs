//! 🧬️ Note diff schema — sparse field delta over the artifact.

use crate::artifacts::note::{NoteBlockNode, NoteImageAsset};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the note artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.note.note")]
pub struct NoteDiff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::note::schema::NoteArtifact>>,
    #[state(persistent)] pub schema: Option<String>,
    #[state(persistent)] pub id: Option<String>,
    #[state(persistent)] pub title: Option<Option<String>>,
    #[state(persistent)] pub blocks: Option<NoteBlocksDelta>,
    #[state(persistent)] pub grid_visible: Option<Option<bool>>,
    #[state(persistent)] pub grid_spacing: Option<Option<f64>>,
    #[state(persistent)] pub grid_subdivisions: Option<Option<f64>>,
    #[state(persistent)] pub grid_opacity: Option<Option<f64>>,
    #[state(persistent)] pub snap_enabled: Option<Option<bool>>,
    #[state(persistent)] pub snap_grid_spacing: Option<Option<f64>>,
    #[state(persistent)] pub pencil_width: Option<Option<f64>>,
    #[state(persistent)] pub eraser_radius: Option<Option<f64>>,
    #[state(persistent)] pub assets: Option<NoteAssetsDelta>,
    #[state(shared_ui)] pub selected_block_ids: Option<NoteStringList>,
    #[state(shared_ui)] pub active_utility_id: Option<String>,
    #[state(local_ui)] pub engagement_input: Option<String>,
    #[state(local_ui)] pub camera_x: Option<f64>,
    #[state(local_ui)] pub camera_y: Option<f64>,
    #[state(local_ui)] pub camera_zoom: Option<f64>,
    #[state(local_ui)] pub locale: Option<String>,
    #[state(preview)] pub hovered_block_id: Option<Option<String>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🗂️ Asset-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteAssetsDelta {
    pub entries: BTreeMap<String, Option<NoteImageAsset>>,
}

/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `blocks`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteBlocksDelta {
    pub added: Vec<NoteBlockNode>,
    pub removed: Vec<String>,
    pub patched: Vec<NoteBlockPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched block entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteBlockPatchEntry {
    pub id: String,
    pub patch: NoteBlockPatch,
}

/// 🩹 Sparse block field patch (JSON blob for whole-block replacement).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NoteBlockPatch {
    pub block_json: Option<String>,
}
//#endregion 🔖️DeltaHelpers
