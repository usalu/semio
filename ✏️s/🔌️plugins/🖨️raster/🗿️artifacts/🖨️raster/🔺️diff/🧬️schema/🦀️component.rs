//! 🧬️ Raster diff schema — sparse field delta over the artifact.

use crate::artifacts::raster::{RasterImageAsset, RasterLayerNode, RasterLayerPatch, RasterViewportSize};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the raster artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.raster.raster")]
pub struct RasterDiff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::raster::schema::RasterArtifact>>,
    #[state(persistent)] pub schema: Option<String>,
    #[state(persistent)] pub id: Option<String>,
    #[state(persistent)] pub title: Option<Option<String>>,
    #[state(persistent)] pub layers: Option<RasterLayersDelta>,
    #[state(persistent)] pub assets: Option<RasterAssetsDelta>,
    #[state(shared_ui)] pub selected_ids: Option<RasterStringList>,
    #[state(shared_ui)] pub active_utility_id: Option<String>,
    #[state(local_ui)] pub brush_size: Option<f64>,
    #[state(local_ui)] pub brush_opacity: Option<f64>,
    #[state(local_ui)] pub composite_viewport: Option<Option<RasterViewportSize>>,
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
pub struct RasterAssetsDelta {
    pub entries: BTreeMap<String, Option<RasterImageAsset>>,
}

/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RasterStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `layers`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RasterLayersDelta {
    pub added: Vec<RasterLayerNode>,
    pub removed: Vec<String>,
    pub patched: Vec<RasterLayerPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched layer entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RasterLayerPatchEntry {
    pub id: String,
    pub patch: RasterLayerPatch,
}
//#endregion 🔖️DeltaHelpers
