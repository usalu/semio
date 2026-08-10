//! 🧬️ Puzzle5d diff schema — sparse field delta over the artifact.

use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dKindCatalogs, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart};
use crate::artifacts::puzzle5d::schema::Puzzle5dArtifact;
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the puzzle5d artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle5d")]
pub struct Puzzle5dDiff {
    #[state(persistent)] pub artifact: Option<Box<Puzzle5dArtifact>>,
    #[state(persistent)] pub schema: Option<String>,
    #[state(persistent)] pub domain: Option<String>,
    #[state(persistent)] pub label: Option<Option<String>>,
    #[state(persistent)] pub meta: Option<Puzzle5dMeta>,
    #[state(persistent)] pub kind_catalogs: Option<Option<Puzzle5dKindCatalogs>>,
    #[state(persistent)] pub kind_compatibility: Option<Puzzle5dKindCompatibilityList>,
    #[state(persistent)] pub parts: Option<Puzzle5dPartsDelta>,
    #[state(persistent)] pub fasteners: Option<Puzzle5dFastenersDelta>,
    #[state(shared_ui)] pub selected_part_ids: Option<Puzzle5dStringList>,
    #[state(shared_ui)] pub selected_grip_ids: Option<Puzzle5dStringList>,
    #[state(shared_ui)] pub selected_fastener_ids: Option<Puzzle5dStringList>,
    #[state(shared_ui)] pub active_utility_id: Option<String>,
    #[state(local_ui)] pub camera2d_x: Option<f64>,
    #[state(local_ui)] pub camera2d_y: Option<f64>,
    #[state(local_ui)] pub camera2d_zoom: Option<f64>,
    #[state(local_ui)] pub camera3d_position_x: Option<f64>,
    #[state(local_ui)] pub camera3d_position_y: Option<f64>,
    #[state(local_ui)] pub camera3d_position_z: Option<f64>,
    #[state(local_ui)] pub camera3d_target_x: Option<f64>,
    #[state(local_ui)] pub camera3d_target_y: Option<f64>,
    #[state(local_ui)] pub camera3d_target_z: Option<f64>,
    #[state(local_ui)] pub camera3d_zoom: Option<f64>,
    #[state(local_ui)] pub selection_method: Option<String>,
    #[state(local_ui)] pub grid_snap_enabled: Option<bool>,
    #[state(local_ui)] pub grid_factor: Option<f64>,
    #[state(local_ui)] pub suggestion_offset: Option<f64>,
    #[state(local_ui)] pub overlap_budget: Option<f64>,
    #[state(local_ui)] pub fill_count: Option<u32>,
    #[state(local_ui)] pub brush_candidate_index: Option<u32>,
    #[state(local_ui)] pub lod_mode: Option<String>,
    #[state(local_ui)] pub locale: Option<String>,
    #[state(local_ui)] pub runtime_extras_json: Option<String>,
    #[state(preview)] pub hovered_part_id: Option<Option<String>>,
    #[state(preview)] pub preview_seq: Option<i64>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers

/// 📋 Kind-compatibility list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle5dKindCompatibilityList {
    pub values: Vec<Puzzle5dKindCompatibility>,
}


/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle5dStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `parts`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle5dPartsDelta {
    pub added: Vec<Puzzle5dPart>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle5dPartPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle5dPart` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dPartPatchEntry {
    pub id: String,
    pub patch: Puzzle5dPartPatch,
}

/// 🩹 Sparse patch over `Puzzle5dPart` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle5dPartPatch {
    pub replacement: Option<Puzzle5dPart>,
}

/// 🧩 Identified-collection delta for `fasteners`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle5dFastenersDelta {
    pub added: Vec<Puzzle5dFastener>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle5dFastenerPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle5dFastener` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dFastenerPatchEntry {
    pub id: String,
    pub patch: Puzzle5dFastenerPatch,
}

/// 🩹 Sparse patch over `Puzzle5dFastener` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle5dFastenerPatch {
    pub replacement: Option<Puzzle5dFastener>,
}

//#endregion 🔖️DeltaHelpers

