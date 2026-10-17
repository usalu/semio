//! 🧬️ Raster diff schema — sparse field delta over the artifact.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

use semio_framework_pixels::editing::{paint_stroke_in_place, validate_extent, PixelAlphaBrush, PixelBrush, PixelOperation};
use semio_framework_pixels::RasterImage;
use semio_s_artifact_raster_raster_model::RasterLayerMask;
use semio_s_artifact_raster_raster_model::RasterTransform;
use semio_s_artifact_raster_raster_model::semio_image_from_rgba8;
use semio_s_artifact_raster_raster_model::layer_protection;
use semio_s_artifact_raster_raster_model::RasterMaskContent;
use semio_s_artifact_raster_raster_model::RasterPixelContent;
use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;
use semio_s_artifact_raster_raster_model::{RasterLayerNode, RasterLayerPatch};
use schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the raster artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.raster.raster")]
pub struct RasterDiff {
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
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub pixels: Vec<RasterPixelRegion>,
}
//#endregion 🔖️Diff

//#endregion 🔖️DeltaHelpers

use semio_s_artifact_raster_raster_model::find_layer;
use semio_s_artifact_raster_raster_model::flatten_raster_layers;
use semio_s_artifact_raster_raster_model::layer_node_id;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use protocol::MutationDiff;

#[path = "🌲️layer-tree-apply/🦀️.rs"]
mod layer_tree_apply;
pub use layer_tree_apply::*;

#[path = "🧮️inverse-and-absorb/🦀️.rs"]
mod inverse_and_absorb;
use inverse_and_absorb::*;

/// ➕ Sparse insertion diff — tree-aware (`parent_id: None` = document root), so `create-layer` never
/// needs to fall back to whole-snapshot capture even when inserting into a nested `Group`.
pub fn diff_add_layer(parent_id: Option<String>, index: usize, layer: RasterLayerNode) -> RasterDiff {
    RasterDiff { layers: Some(RasterLayersDelta { inserted: vec![RasterLayerInsertion { parent_id, index, layer }], ..Default::default() }), ..Default::default() }
}

/// ➖️ Sparse removal diff: one removal row at the layer's base address. Empty when the layer is not in `base`.
pub fn diff_remove_layer(base: &[RasterLayerNode], layer_id: &str) -> RasterDiff {
    let Some((parent_id, index)) = semio_s_artifact_raster_raster_model::locate_layer(base, layer_id) else { return RasterDiff::default() };
    RasterDiff { layers: Some(RasterLayersDelta { removed: vec![RasterLayerRemoval { id: layer_id.to_string(), parent_id, index }], ..Default::default() }), ..Default::default() }
}

pub fn diff_patch_layer(layer_id: &str, patch: RasterLayerPatch) -> RasterDiff {
    RasterDiff { layers: Some(RasterLayersDelta { modified: vec![RasterLayerModification { id: layer_id.to_string(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔀 Sparse reposition diff (`reorder-layers`): one tree-aware move row from the layer's base address to the after address
/// `(parent_id, index)` — never the layer itself. The index saturates at the end of the destination list. Empty when the layer is
/// not in `base`.
pub fn diff_move_layer(base: &[RasterLayerNode], layer_id: &str, parent_id: Option<String>, index: usize) -> RasterDiff {
    let Some((from_parent, from_index)) = semio_s_artifact_raster_raster_model::locate_layer(base, layer_id) else { return RasterDiff::default() };
    let length = container_of(base, parent_id.as_deref()).map_or(0, <[RasterLayerNode]>::len);
    let room = if parent_id == from_parent { length.saturating_sub(1) } else { length };
    RasterDiff {
        layers: Some(RasterLayersDelta {
            moved: vec![RasterLayerRelocation { id: layer_id.to_string(), from: RasterLayerAddress { parent_id: from_parent, index: from_index }, to: RasterLayerAddress { parent_id, index: index.min(room) } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// 🖇️ Sparse asset-map insertion diff (`add-layer-asset`).
pub fn diff_add_asset(asset_id: &str, asset: SemioImageSnapshot) -> RasterDiff {
    let mut entries = BTreeMap::new();
    entries.insert(asset_id.to_string(), Some(asset));
    RasterDiff { assets: Some(RasterAssetsDelta { entries }), ..Default::default() }
}

/// 🗂️ Sparse asset-map removal diff (`remove-layer-asset`).
pub fn diff_remove_asset(asset_id: &str) -> RasterDiff {
    let mut entries = BTreeMap::new();
    entries.insert(asset_id.to_string(), None);
    RasterDiff { assets: Some(RasterAssetsDelta { entries }), ..Default::default() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️layer-tree-delta/🦀️.rs"]
mod layer_tree_delta;

#[path = "🖌️repaint-canvas/🦀️.rs"]
mod repaint_canvas;
pub use repaint_canvas::*;

//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod io {
    #[path = "../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
    pub mod binary;
    #[path = "../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
    pub mod text;
}
//#endregion 🪢️TaxonomyMounts

#[path = "🧱️delta-model/🦀️.rs"]
mod delta_model;
pub use delta_model::*;
