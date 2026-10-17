//! 🎯️ Host-action bridge from shell actions into the closed command channel: key folding and argument decoding.

use super::*;
use semio_s_artifact_raster_raster_editor_commands::RasterCommand;
use semio_framework_plugin::Fault;
use semio_framework_plugin::{FaultCode, FaultOrigin};

fn snake(key: &str) -> String {
    let mut out = String::with_capacity(key.len() + 4);
    for ch in key.chars() {
        if ch.is_ascii_uppercase() {
            out.push('_');
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn camel(key: &str) -> String {
    let mut out = String::with_capacity(key.len());
    let mut upper = false;
    for ch in key.chars() {
        if ch == '_' {
            upper = true;
        } else if upper {
            out.push(ch.to_ascii_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    out
}

fn put(entries: &mut Vec<(String, semio_framework_value::DslValue)>, key: &str, value: semio_framework_value::DslValue) {
    entries.retain(|(existing, _)| existing != key);
    entries.push((key.to_string(), value));
}

/// 🔢️ The host's JSON round trip turns every integer into `Number::Float`; the exact-integer
/// codecs refuse `Float(1.0)`, so whole finite floats are restored to `UInt`/`Int` (every `f64`
/// field accepts any `Number` variant, so nothing else changes).
fn integral(value: semio_framework_value::DslValue) -> semio_framework_value::DslValue {
    match value {
        semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(float)) if float.is_finite() && float.fract() == 0.0 && float.abs() < 9.007_199_254_740_992e15 => {
            if float >= 0.0 { semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(float as u64)) } else { semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(float as i64)) }
        }
        semio_framework_value::DslValue::Array(items) => semio_framework_value::DslValue::Array(items.into_iter().map(integral).collect()),
        semio_framework_value::DslValue::Object(entries) => semio_framework_value::DslValue::Object(entries.into_iter().map(|(key, value)| (key, integral(value))).collect()),
        other => other,
    }
}

/// 🔁️ Emits every key of `args` under BOTH spellings (`FromValue` ignores keys it does not know)
/// after applying `aliases` (snake_case source → destination).
fn fold(args: Option<&semio_framework_value::DslValue>, aliases: &[(&str, &str)]) -> semio_framework_value::DslValue {
    let mut entries: Vec<(String, semio_framework_value::DslValue)> = Vec::new();
    if let Some(semio_framework_value::DslValue::Object(object)) = args {
        for (key, value) in object {
            let mut key = snake(key);
            if let Some((_, to)) = aliases.iter().find(|(from, _)| *from == key) {
                key = (*to).to_string();
            }
            let value = integral(value.clone());
            put(&mut entries, &camel(&key), value.clone());
            put(&mut entries, &key, value);
        }
    }
    semio_framework_value::DslValue::Object(entries)
}

fn patch_args(args: Option<&semio_framework_value::DslValue>, aliases: &[(&str, &str)]) -> semio_framework_value::DslValue {
    let mut value = fold(args, aliases);
    if let semio_framework_value::DslValue::Object(entries) = &mut value {
        for (key, value) in entries {
            if key == "value" && !matches!(value, semio_framework_value::DslValue::String(_)) { *value = semio_framework_value::DslValue::String(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(value))); }
        }
    }
    value
}

fn decode<T: semio_framework_value::FromValue>(action: &str, value: semio_framework_value::DslValue) -> Result<T, Fault> {
    T::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("raster action '{action}' arguments do not decode: {error}")))
}

pub fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<RasterCommand, Fault> {
    // 🧱️ Tree rows and context menus address a layer by its bare `id`; the payloads spell `layer_id`.
    const LAYER: &[(&str, &str)] = &[("id", "layer_id")];
    let plain = || fold(args, &[]);
    let layer = || fold(args, LAYER);
    Ok(match action {
        "exportPng" => RasterCommand::ExportPng(decode(action, plain())?),
        "addLayer" => RasterCommand::AddLayer(decode(action, plain())?),
        "flattenLayers" => RasterCommand::FlattenLayers(decode(action, plain())?),
        "mergeDown" => RasterCommand::MergeDown(decode(action, plain())?),
        "paintStroke" => RasterCommand::PaintStroke(decode(action, layer())?),
        "fillRegion" => RasterCommand::FillRegion(decode(action, layer())?),
        "applyFilter" => RasterCommand::ApplyFilter(decode(action, layer())?),
        "transformImage" => RasterCommand::TransformImage(decode(action, layer())?),
        "fillSelection" => RasterCommand::FillSelection(decode(action, layer())?),
        "maskFromSelection" => RasterCommand::MaskFromSelection(decode(action, layer())?),
        "dropLayerKind" => RasterCommand::DropLayerKind(decode(action, plain())?),
        "setLayerVisible" => RasterCommand::SetLayerVisible(decode(action, layer())?),
        "toggleLayerVisible" => RasterCommand::ToggleLayerVisible(decode(action, layer())?),
        "deleteLayer" => RasterCommand::DeleteLayer(decode(action, layer())?),
        "duplicateLayer" => RasterCommand::DuplicateLayer(decode(action, layer())?),
        "patchLayer" => RasterCommand::PatchLayer(decode(action, patch_args(args, LAYER))?),
        "patchLayers" => RasterCommand::PatchLayers(decode(action, patch_args(args, &[("ids", "layer_ids")]))?),
        "moveLayer" => RasterCommand::MoveLayer(decode(action, fold(args, &[("id", "layer_id"), ("target_id", "target_row_id"), ("position", "drop_position")]))?),
        "setBrushSize" => RasterCommand::SetBrushSize(decode(action, fold(args, &[("size", "value")]))?),
        "setBrushColor" => RasterCommand::SetBrushColor(decode(action, plain())?),
        "setBrushHardness" => RasterCommand::SetBrushHardness(decode(action, plain())?),
        "setPaintTarget"=>RasterCommand::SetPaintTarget(decode(action,plain())?),
        "setPixelSelection"=>RasterCommand::SetPixelSelection(decode(action,plain())?),
        "setMaskValue"=>RasterCommand::SetMaskValue(decode(action,plain())?),
        "setFillTolerance" => RasterCommand::SetFillTolerance(decode(action, plain())?),
        "setBrushOpacity" => RasterCommand::SetBrushOpacity(decode(action, fold(args, &[("opacity", "value")]))?),
        "setCompositeViewport" => RasterCommand::SetCompositeViewport(decode(action, plain())?),
        "setCamera" => RasterCommand::SetCamera(decode(action, plain())?),
        "setCameraZoom" => RasterCommand::SetCameraZoom(decode(action, fold(args, &[("value", "zoom")]))?),
        "setActiveExample" => RasterCommand::SetActiveExample(decode(action, fold(args, &[("id", "example_id"), ("value", "example_id")]))?),
        _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the raster editor has no command for action '{action}'"))),
    })
}
