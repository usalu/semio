//! 🖼️ 🖼️ Raster play app commands command — `patch-layer`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::artifacts::raster::schema::{find_layer, layer_opacity, layer_transform, layer_visible};
use crate::artifacts::raster::mutations::{change_layer_adjustment_kind, change_layer_blend_mode, change_layer_opacity, change_layer_visible, rename_layer, resize_layer};
use crate::artifacts::raster::mutations::move_layer as spatial_move_layer;
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Shared
/// 🧭️ Maps a `patchLayer`/`patchLayers` field write onto the one real semantic mutation it now means
/// — replaces the retired option-bag `layer_patch_for_field`/`PatchLayer` pair. `field` keeps the
/// panel's pre-migration wire names (`transformX`/`transformY`/`blendMode`/`adjustmentKind`) so no
/// UI call site needs to change.
async fn raster_mutation_for_field(layer_id: &str, field: &str, value: &Value, prior: &RasterLayerNode) -> Option<RasterMutation> {
    match field {
        "name" => Some(RasterMutation::RenameLayer(rename_layer::mutation::RenameLayer { layer_id: layer_id.into(), new_name: value.as_str().unwrap_or("").into() })),
        "visible" => Some(RasterMutation::ChangeLayerVisible(change_layer_visible::mutation::ChangeLayerVisible { layer_id: layer_id.into(), new_visible: value.as_bool().unwrap_or_else(|| !layer_visible(prior)) })),
        "opacity" => Some(RasterMutation::ChangeLayerOpacity(change_layer_opacity::mutation::ChangeLayerOpacity { layer_id: layer_id.into(), new_opacity: value.as_f64().unwrap_or(layer_opacity(prior) as f64) as f32 })),
        "blendMode" => Some(RasterMutation::ChangeLayerBlendMode(change_layer_blend_mode::mutation::ChangeLayerBlendMode { layer_id: layer_id.into(), new_blend_mode: value.as_str().unwrap_or("normal").into() })),
        "transformX" => {
            let transform = layer_transform(prior);
            Some(RasterMutation::MoveLayer(spatial_move_layer::mutation::MoveLayer { layer_id: layer_id.into(), new_x: value.as_f64().unwrap_or(transform.x), new_y: transform.y }))
        }
        "transformY" => {
            let transform = layer_transform(prior);
            Some(RasterMutation::MoveLayer(spatial_move_layer::mutation::MoveLayer { layer_id: layer_id.into(), new_x: transform.x, new_y: value.as_f64().unwrap_or(transform.y) }))
        }
        "width" => {
            let (width, height) = pixel_extent(prior);
            Some(RasterMutation::ResizeLayer(resize_layer::mutation::ResizeLayer { layer_id: layer_id.into(), new_width: value.as_u64().unwrap_or(width as u64) as u32, new_height: height }))
        }
        "height" => {
            let (width, height) = pixel_extent(prior);
            Some(RasterMutation::ResizeLayer(resize_layer::mutation::ResizeLayer { layer_id: layer_id.into(), new_width: width, new_height: value.as_u64().unwrap_or(height as u64) as u32 }))
        }
        "adjustmentKind" => Some(RasterMutation::ChangeLayerAdjustmentKind(change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind { layer_id: layer_id.into(), new_adjustment_kind: value.as_str().unwrap_or("brightnessContrast").into() })),
        _ => None,
    }
}

/// 📐️ Current `(width, height)` for a `Pixel` layer, `(512, 512)` for any other kind — mirrors
/// `resize-layer`'s own inverse-side default.
async fn pixel_extent(layer: &RasterLayerNode) -> (u32, u32) {
    match layer {
        RasterLayerNode::Pixel { width, height, .. } => (width.unwrap_or(512), height.unwrap_or(512)),
        _ => (512, 512),
    }
}

/// 🩹️ Builds the `RasterMutation`s for a `patchLayer`/`patchLayers` field write across ids — shared by
/// both payloads below (the only two consumers).
async fn raster_patch_layer_operations(document: &RasterSnapshot, layer_ids: &[String], field: &str, value: &Value) -> Vec<RasterMutation> {
    layer_ids
        .iter()
        .filter_map(|layer_id| {
            let prior = find_layer(&document.layers, layer_id)?;
            raster_mutation_for_field(layer_id, field, value, prior)
        })
        .collect()
}

/// 🩹️ Parses a `patchLayer`/`patchLayers` wire `value` as JSON text (falling back to a plain JSON string
/// when it isn't valid JSON) — mirrors `draw_ui::patch_value_json`.
async fn patch_value_json(value: &str) -> Value {
    serde_json::from_str(value).unwrap_or_else(|_| Value::String(value.to_string()))
}
//#endregion 🔖️Shared










#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "patch-layer")]
pub struct PatchLayer {
    pub layer_id: String,
    pub field: String,
    pub value: String,
}

pub async fn handle(payload: &PatchLayer, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let json_value = patch_value_json(&payload.value);
    let operations = raster_patch_layer_operations(doc.snapshot, std::slice::from_ref(&payload.layer_id), &payload.field, &json_value);
    if operations.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::mutations(operations))
    }
}
