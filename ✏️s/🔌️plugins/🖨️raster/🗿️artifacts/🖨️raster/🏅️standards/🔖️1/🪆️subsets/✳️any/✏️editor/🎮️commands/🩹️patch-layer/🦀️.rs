//! 🖼️ 🖼️ Raster play app commands command — `patch-layer`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::mutations::move_layer as spatial_move_layer;
use crate::mutations::{change_layer_adjustment_kind, change_layer_blend_mode, change_layer_opacity, change_layer_visible, rename_layer, resize_layer};
use crate::op::RasterMutation;
use crate::standards::v1::subsets::any::schema::{find_layer, layer_opacity, layer_transform, layer_visible};
use crate::{RasterLayerMask, RasterLayerNode, RasterSnapshot, RasterTransform};
use dsl::os_pack::json::Value;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shared
/// 🧭️ Converts a validated property edit into its semantic document mutation.
fn raster_mutation_for_field(layer_id: &str, field: &str, value: &Value, prior: &RasterLayerNode) -> Option<RasterMutation> {
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
            Some(RasterMutation::ResizeLayer(resize_layer::mutation::ResizeLayer { layer_id: layer_id.into(), new_width: value.as_f64().unwrap_or(f64::from(width)) as u32, new_height: height }))
        }
        "height" => {
            let (width, height) = pixel_extent(prior);
            Some(RasterMutation::ResizeLayer(resize_layer::mutation::ResizeLayer { layer_id: layer_id.into(), new_width: width, new_height: value.as_f64().unwrap_or(f64::from(height)) as u32 }))
        }
        "maskPresent" | "maskEnabled" | "maskInvert" | "maskX" | "maskY" | "maskScaleX" | "maskScaleY" | "maskRotation" | "maskWidth" | "maskHeight" => {
            let (RasterLayerNode::Pixel { mask: expected, .. } | RasterLayerNode::Group { mask: expected, .. }) = prior else { return None; };
            let mut mask = expected.clone();
            match field {
                "maskPresent" if !value.as_bool()? => mask = None,
                "maskPresent" => { mask.get_or_insert_with(|| RasterLayerMask { enabled: true, linked: true, invert: false, width: None, height: None, image_key: None, transform: RasterTransform::default() }); }
                "maskEnabled" => mask.as_mut()?.enabled = value.as_bool()?,
                "maskInvert" => mask.as_mut()?.invert = value.as_bool()?,
                "maskX" => mask.as_mut()?.transform.x = value.as_f64()?,
                "maskY" => mask.as_mut()?.transform.y = value.as_f64()?,
                "maskScaleX" => mask.as_mut()?.transform.scale_x = value.as_f64()?,
                "maskScaleY" => mask.as_mut()?.transform.scale_y = value.as_f64()?,
                "maskRotation" => mask.as_mut()?.transform.rotation = value.as_f64()?,
                "maskWidth" => mask.as_mut()?.width = Some(value.as_f64()? as u32),
                "maskHeight" => mask.as_mut()?.height = Some(value.as_f64()? as u32),
                _ => return None,
            }
            Some(RasterMutation::ChangeLayerMask(crate::mutations::change_layer_mask::ChangeLayerMask { layer_id: layer_id.into(), expected: expected.clone(), mask }))
        }
        "adjustmentKind" => Some(RasterMutation::ChangeLayerAdjustmentKind(change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind { layer_id: layer_id.into(), new_adjustment_kind: value.as_str().unwrap_or("brightnessContrast").into() })),
        _ => None,
    }
}

/// 📐️ Current `(width, height)` for a `Pixel` layer, `(512, 512)` for any other kind — mirrors
/// `resize-layer`'s own inverse-side default.
fn pixel_extent(layer: &RasterLayerNode) -> (u32, u32) {
    match layer {
        RasterLayerNode::Pixel { width, height, .. } => (width.unwrap_or(512), height.unwrap_or(512)),
        _ => (512, 512),
    }
}

/// 🩹️ Builds the `RasterMutation`s for a `patchLayer`/`patchLayers` field write across ids — shared by
/// both payloads below (the only two consumers).
pub(super) fn raster_patch_layer_operations(document: &RasterSnapshot, layer_ids: &[String], field: &str, value: &Value) -> Result<Vec<RasterMutation>, Fault> {
    let valid = match field {
        "name" | "blendMode" | "adjustmentKind" => value.as_str().is_some(),
        "visible" | "maskPresent" | "maskEnabled" | "maskInvert" => value.as_bool().is_some(),
        "opacity" => value.as_f64().is_some_and(|v| v.is_finite() && (0.0..=1.0).contains(&v)),
        "transformX" | "transformY" | "maskX" | "maskY" | "maskRotation" => value.as_f64().is_some_and(f64::is_finite),
        "maskScaleX" | "maskScaleY" => value.as_f64().is_some_and(|v| v.is_finite() && v != 0.0),
        "width" | "height" | "maskWidth" | "maskHeight" => value.as_f64().is_some_and(|v| v.fract() == 0.0 && (1.0..=16384.0).contains(&v)),
        _ => false,
    };
    if !valid { return Err(Fault::from("raster-layer-property-invalid")); }
    layer_ids.iter().map(|id| {
        let layer = find_layer(&document.layers, id).ok_or_else(|| Fault::from("raster-layer-not-found"))?;
        if matches!(field, "width" | "height") && !matches!(layer, RasterLayerNode::Pixel { .. }) { return Err(Fault::from("raster-layer-dimensions-require-pixels")); }
        let mutation = raster_mutation_for_field(id, field, value, layer).ok_or_else(|| Fault::from("raster-layer-property-unsupported"))?;
        if let RasterMutation::ChangeLayerMask(payload) = &mutation {
            crate::mutations::change_layer_mask::validate(payload, document).map_err(Fault::from)?;
        }
        Ok(mutation)
    }).collect()
}

/// 🩹️ Parses a `patchLayer`/`patchLayers` wire `value` as JSON text (falling back to a plain JSON string
/// when it isn't valid JSON) — mirrors `draw_ui::patch_value_json`.
pub(super) fn patch_value_json(field: &str, value: &str) -> Value {
    if matches!(field, "name" | "blendMode" | "adjustmentKind") { Value::String(value.to_string()) } else { dsl::os_pack::json::parse(value).unwrap_or_else(|_| Value::String(value.to_string())) }
}
//#endregion 🔖️Shared

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-layer")]
pub struct PatchLayer {
    pub layer_id: String,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchLayer, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let json_value = patch_value_json(&payload.field, &payload.value);
    let operations = raster_patch_layer_operations(doc.snapshot, std::slice::from_ref(&payload.layer_id), &payload.field, &json_value)?;
    if operations.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::mutations(operations))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
