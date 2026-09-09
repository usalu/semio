//! 🧬️ Raster diff schema — sparse field delta over the artifact.

use crate::{RasterImageAsset, RasterLayerNode, RasterLayerPatch};
use schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the raster artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.raster.raster")]
pub struct RasterDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::standards::v1::subsets::any::schema::RasterArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub title: Option<Option<String>>,
    #[state(artifact)]
    pub layers: Option<RasterLayersDelta>,
    #[state(artifact)]
    pub assets: Option<RasterAssetsDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🗂️ Asset-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RasterAssetsDelta {
    pub entries: BTreeMap<String, Option<RasterImageAsset>>,
}

/// 🧩 Identified-collection delta for `layers` — every entry is tree-aware (`parent_id: None` means
/// the document root) so `create-layer`/`reorder-layers` never fall back to whole-snapshot capture,
/// even when the target lives inside a nested `Group`.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RasterLayersDelta {
    pub added: Vec<RasterLayerInsertion>,
    pub removed: Vec<String>,
    pub patched: Vec<RasterLayerPatchEntry>,
    pub moved: Vec<RasterLayerMove>,
}

/// ➕ One inserted layer (`create-layer`) — carries its own tree address so insertion into a nested
/// `Group` is expressible sparsely.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerInsertion {
    pub parent_id: Option<String>,
    pub index: usize,
    pub layer: RasterLayerNode,
}

/// 🔀 One repositioned layer (`reorder-layers`) — remove-then-insert at a tree address, never a
/// flat top-level-only reorder.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerMove {
    pub id: String,
    pub parent_id: Option<String>,
    pub index: usize,
}

/// 🩹 One patched layer entry.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerPatchEntry {
    pub id: String,
    pub patch: RasterLayerPatch,
}
//#endregion 🔖️DeltaHelpers
