//! 🧬️ Puzzle3d diff schema — sparse field delta over the artifact.

use crate::artifacts::puzzle3d::{Puzzle3dAttraction, Puzzle3dMeta, Puzzle3dObject, Puzzle3dReference, Puzzle3dTargetVolume};
use crate::artifacts::puzzle3d::schema::Puzzle3dArtifact;
use artifact_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the puzzle3d artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle3d")]
pub struct Puzzle3dDiff {
    #[state(artifact)] pub artifact: Option<Box<Puzzle3dArtifact>>,
    #[state(artifact)] pub schema: Option<String>,
    #[state(artifact)] pub domain: Option<String>,
    #[state(artifact)] pub meta: Option<Puzzle3dMeta>,
    #[state(artifact)] pub objects: Option<Puzzle3dObjectsDelta>,
    #[state(artifact)] pub attractions: Option<Puzzle3dAttractionsDelta>,
    #[state(artifact)] pub target_volumes: Option<Puzzle3dTargetVolumesDelta>,
    #[state(artifact)] pub references: Option<Puzzle3dReferencesDelta>,
    #[state(presence)] pub selected_object_ids: Option<Puzzle3dStringList>,
    #[state(presence)] pub selected_vortex_ids: Option<Puzzle3dStringList>,
    #[state(presence)] pub selected_attraction_ids: Option<Puzzle3dStringList>,
    #[state(presence)] pub selected_target_volume_ids: Option<Puzzle3dStringList>,
    #[state(presence)] pub selected_reference_ids: Option<Puzzle3dStringList>,
    #[state(presence)] pub active_utility_id: Option<String>,
    #[state(config)] pub camera_position_x: Option<f64>,
    #[state(config)] pub camera_position_y: Option<f64>,
    #[state(config)] pub camera_position_z: Option<f64>,
    #[state(config)] pub camera_target_x: Option<f64>,
    #[state(config)] pub camera_target_y: Option<f64>,
    #[state(config)] pub camera_target_z: Option<f64>,
    #[state(config)] pub camera_zoom: Option<f64>,
    #[state(config)] pub selection_method: Option<String>,
    #[state(config)] pub selection_mode_default: Option<String>,
    #[state(config)] pub engagement_input: Option<String>,
    #[state(config)] pub grid_visible: Option<bool>,
    #[state(config)] pub grid_snap_enabled: Option<bool>,
    #[state(config)] pub grid_spacing: Option<f64>,
    #[state(config)] pub overlap_budget: Option<f64>,
    #[state(config)] pub fill_count: Option<u32>,
    #[state(config)] pub brush_candidate_index: Option<u32>,
    #[state(config)] pub lod_automatic: Option<bool>,
    #[state(config)] pub lod_depth_variable: Option<bool>,
    #[state(config)] pub lod_manual: Option<f64>,
    #[state(config)] pub proximity_radius: Option<f64>,
    #[state(config)] pub locale: Option<String>,
    #[state(config)] pub runtime_extras_json: Option<String>,
    #[state(artifact)] pub hovered_object_id: Option<Option<String>>,
    #[state(artifact)] pub hovered_vortex_full_id: Option<Option<String>>,
    #[state(artifact)] pub hovered_kind_id: Option<Option<String>>,
    #[state(artifact)] pub preview_seq: Option<i64>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle3dStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `objects`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle3dObjectsDelta {
    pub added: Vec<Puzzle3dObject>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle3dObjectPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle3dObject` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dObjectPatchEntry {
    pub id: String,
    pub patch: Puzzle3dObjectPatch,
}

/// 🩹 Sparse patch over `Puzzle3dObject` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle3dObjectPatch {
    pub replacement: Option<Puzzle3dObject>,
}

/// 🧩 Identified-collection delta for `attractions`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle3dAttractionsDelta {
    pub added: Vec<Puzzle3dAttraction>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle3dAttractionPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle3dAttraction` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dAttractionPatchEntry {
    pub id: String,
    pub patch: Puzzle3dAttractionPatch,
}

/// 🩹 Sparse patch over `Puzzle3dAttraction` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle3dAttractionPatch {
    pub replacement: Option<Puzzle3dAttraction>,
}

/// 🧩 Identified-collection delta for `targetVolumes`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle3dTargetVolumesDelta {
    pub added: Vec<Puzzle3dTargetVolume>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle3dTargetVolumePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle3dTargetVolume` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dTargetVolumePatchEntry {
    pub id: String,
    pub patch: Puzzle3dTargetVolumePatch,
}

/// 🩹 Sparse patch over `Puzzle3dTargetVolume` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle3dTargetVolumePatch {
    pub replacement: Option<Puzzle3dTargetVolume>,
}

/// 🧩 Identified-collection delta for `references`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle3dReferencesDelta {
    pub added: Vec<Puzzle3dReference>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle3dReferencePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle3dReference` entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dReferencePatchEntry {
    pub id: String,
    pub patch: Puzzle3dReferencePatch,
}

/// 🩹 Sparse patch over `Puzzle3dReference` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Puzzle3dReferencePatch {
    pub replacement: Option<Puzzle3dReference>,
}

//#endregion 🔖️DeltaHelpers

