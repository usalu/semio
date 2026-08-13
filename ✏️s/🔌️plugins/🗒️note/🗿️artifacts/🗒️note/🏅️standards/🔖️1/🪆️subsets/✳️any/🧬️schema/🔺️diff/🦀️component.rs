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
    #[state(artifact)] pub artifact: Option<Box<crate::artifacts::note::schema::NoteArtifact>>,
    #[state(artifact)] pub schema: Option<String>,
    #[state(artifact)] pub id: Option<String>,
    #[state(artifact)] pub title: Option<Option<String>>,
    #[state(artifact)] pub blocks: Option<NoteBlocksDelta>,
    #[state(artifact)] pub grid_visible: Option<Option<bool>>,
    #[state(artifact)] pub grid_spacing: Option<Option<f64>>,
    #[state(artifact)] pub grid_subdivisions: Option<Option<f64>>,
    #[state(artifact)] pub grid_opacity: Option<Option<f64>>,
    #[state(artifact)] pub snap_enabled: Option<Option<bool>>,
    #[state(artifact)] pub snap_grid_spacing: Option<Option<f64>>,
    #[state(artifact)] pub pencil_width: Option<Option<f64>>,
    #[state(artifact)] pub eraser_radius: Option<Option<f64>>,
    #[state(artifact)] pub assets: Option<NoteAssetsDelta>,
    /// 🔗️ Same double-`Option` shape as every optional-slot field in this ticket's plugins, for the
    /// `R:any` forward link slot — schema/codec-complete, currently unset by any mutation (see the
    /// snapshot field's own doc comment).
    #[state(artifact)] pub linked_artifact: Option<Option<store::ArtifactLink>>,
    #[state(presence)] pub selected_block_ids: Option<NoteStringList>,
    #[state(presence)] pub active_utility_id: Option<String>,
    #[state(config)] pub engagement_input: Option<String>,
    #[state(config)] pub camera_x: Option<f64>,
    #[state(config)] pub camera_y: Option<f64>,
    #[state(config)] pub camera_zoom: Option<f64>,
    #[state(config)] pub locale: Option<String>,
    #[state(artifact)] pub hovered_block_id: Option<Option<String>>,
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
    pub added: Vec<NoteAddedBlockEntry>,
    pub removed: Vec<String>,
    pub patched: Vec<NoteBlockPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// ➕ One added/reparented block: `parent_id` (`None` = document root) and `index` (`None` =
/// append) place it — `create-block`/`duplicate-block(s)`/`move-block-to-container` all diff
/// through this, never a whole-`blocks` vec swap. No `Default` derive: `NoteBlockNode` (a tagged
/// enum) has no sensible default value, and every construction site fills all three fields anyway.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteAddedBlockEntry {
    pub parent_id: Option<String>,
    pub index: Option<usize>,
    pub block: NoteBlockNode,
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
