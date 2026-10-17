//! 🧱️ Raster diff delta model: the layer, asset and pixel-region delta records the sparse diff is made of.

use super::*;

//#region 🔖️DeltaHelpers
/// 🗂️ Asset-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RasterAssetsDelta {
    pub entries: BTreeMap<String, Option<SemioImageSnapshot>>,
}

/// 🖌️ One rectangle of pixel samples written into the image a layer (or its mask) shows: `width × height` RGBA8 samples,
/// row-major, at `(x, y)` of the target image. Applying it files the rewritten image as a new content-addressed asset and
/// repoints the layer — the handle is derived by the applier from the applied pixels, never carried by the diff.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterPixelRegion {
    pub layer_id: String,
    pub target: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub samples: Vec<u8>,
}

/// 🧩 Positional delta for the layer TREE (the shape of `protocol::list_delta`, addressed per container): `removed` rows carry
/// their BASE address, `inserted` rows their AFTER address, `moved` rows both (a move may cross containers); `parent_id: None`
/// means the document root. No order list and no anchor is ever carried; every index is a coordinate of the base or of the after
/// child list of its parent.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RasterLayersDelta {
    pub removed: Vec<RasterLayerRemoval>,
    pub inserted: Vec<RasterLayerInsertion>,
    pub moved: Vec<RasterLayerRelocation>,
    pub modified: Vec<RasterLayerModification>,
}

/// 📍️ A position in the layer tree: the child list of `parent_id` (the root list when absent) at `index`.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerAddress {
    #[value(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub index: usize,
}

/// ➖️ One removed layer subtree and the BASE address the inverse reinserts it at.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerRemoval {
    pub id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub index: usize,
}

/// ➕ One inserted layer subtree (`create-layer`) and its AFTER address.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerInsertion {
    #[value(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub index: usize,
    pub layer: RasterLayerNode,
}

/// ↕️ One repositioned layer subtree (`reorder-layers`): its BASE address and its AFTER address — the layer itself is never carried.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerRelocation {
    pub id: String,
    pub from: RasterLayerAddress,
    pub to: RasterLayerAddress,
}

/// 🩹 One modified layer entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RasterLayerModification {
    pub id: String,
    pub patch: RasterLayerPatch,
}
