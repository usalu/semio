//! 🖥️ Raster editor — ArtifactEditor impl, render, manifest (constitutional: ui/general). B1: `RasterPlayApp`
//! is a unit struct — every former `RasterConfig` (`ui`-crate `RefCell`) field (selection, hover, brush
//! size/opacity, navigator composite-viewport size, the session-only free camera) now lives in
//! `crate::editor::raster::config::RasterConfig`, written via `RasterConfigMutation`s. Every action
//! dispatches through the single typed `RasterCommand` channel via `app_commands!` — mirrors
//! `shooting_ui`'s B1 pilot.

#[path="📤️export/🦀️.rs"]
mod media_export;

#[path="🖱️selection/🦀️.rs"]
pub(crate) mod layer_selection;

#[cfg(test)]
#[path="💾️document/🧪️tests/🦀️.rs"]
mod document_tests;

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::editor::raster::modes::edit;
use crate::editor::raster::modes::edit::windows::{composite, navigator};
use crate::editor::raster::presence::{RasterPresence, RasterPresenceMutation};
use crate::editor::raster::terminology::raster_play_labels;
use crate::op::RasterMutation;
use crate::{RasterLayerNode, RasterSnapshot, RASTER_DOCUMENT_SCHEMA};
use semio_framework_pack_json::Value;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, JobFault, JobPayloadStream, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionFactory;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppDefinition;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactKindSpec;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactReservedJob;
use semio_framework_plugin::ArtifactReservedToolInput;
use semio_framework_plugin::ArtifactReservedToolJob;
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::ArtifactToolCompletion;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::OsMediaCapability;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use store::ArtifactPack;
use semio_framework_2d::compute::EngineHandles;

/// 🌱️ Load effect for a whole document: example switches, JSON loads and imports replace the document through the
/// artifact's load path, never as mutation rows or history.
pub(crate) fn raster_reset_document_effect(document: &RasterSnapshot) -> semio_framework_plugin::Effect {
    let pack = <RasterSnapshot as ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr(&document.id.to_string(), RASTER_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}

//#region 🔖️Constants
pub const RASTER_PLAY_CONTROLLER_ID: &str = "raster-play";
/// 🌳️ Prefix for every layer-tree row id — shared by the document/masks panels and the `moveLayer`
/// command (which needs to decode a `target_row_id` back into a layer/group id). App-wide tree-encoding
/// concern, not artifact data, so it lives here rather than in any single panel.
pub const RASTER_TREE_PREFIX: &str = "raster-play-layers";
/// 🕹️ The single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM interaction domain this app declares
/// (granularity `layer`, `HierarchyProvider::Topology`, method Pick) — the layer tree binds it and the
/// composite window references it.
pub const RASTER_INTERACTION_DOMAIN: &str = "layers";
pub const RASTER_INTERACTION_GRANULARITY: &str = "layer";
//#endregion 🔖️Constants

//#region 🔖️Document
/// 🌳️ Encodes a layer as its tree-row id — shared by the document/masks panels (which render rows) and
/// `moveLayer` (which decodes a drop target back into an id). More than one consumer, but this is UI row
/// encoding, not artifact data, so it stays app-level rather than in `crate::schema`.
pub fn layer_row_id(layer: &RasterLayerNode) -> String {
    crate::standards::v1::subsets::any::schema::layer_node_id(layer).to_string()
}

pub fn mask_row_id(target_id: &str) -> String {
    format!("{RASTER_TREE_PREFIX}.mask.{target_id}")
}

/// 📡️ Document JSON for the WASM compositor, omitting embedded assets — mirrors premigration
/// `rasterDocumentToSyncJson`. Stays app-level next to {@link raster_scene}, its only caller.
///
/// 🛡️ Built field by field rather than from `RasterSnapshot`'s own `ToValue`, because the compositor
/// reads its pixels from `assetsJson` (resolved content) and must not also carry the `assets` HANDLE
/// pool. The layer forest itself — including a group's children and an adjustment's `params` owned
/// map — goes through the artifact's real derived codec.
fn document_sync_json(document: &RasterSnapshot) -> String {
    let mut fields = vec![("schema".to_string(), semio_framework_value::DslValue::String(document.schema.clone())), ("id".to_string(), semio_framework_value::DslValue::String(document.id.clone()))];
    if let Some(title) = &document.title {
        fields.push(("title".to_string(), semio_framework_value::DslValue::String(title.clone())));
    }
    fields.push(("layers".to_string(), semio_framework_value::DslValue::Array(document.layers.iter().map(semio_framework_value::ToValue::to_value).collect())));
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::DslValue::Object(fields)).to_string()
}

/// 🧩️ Resolves every asset handle on `document.assets` back to its real `RasterImageAsset` bytes
/// through the working-scene cache accessor (`crate::raster_asset`, ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` — `document.assets` now stores composed
/// `s.stdio.semio.image` CHILD handles, not embedded bytes) — the ONE call site the WASM compositor's
/// real pixel bytes funnel through. A handle whose content is not (or no longer) cached is honestly
/// omitted rather than serialized as an empty/garbage blob (documented staleness gap, matches every
/// other exemplar in this ticket).
fn assets_json_from_document(document: &RasterSnapshot) -> String {
    let resolved: std::collections::BTreeMap<String, crate::RasterImageAsset> = document.assets.keys().filter_map(|asset_id| crate::raster_asset(&document.assets, asset_id).map(|asset| (asset_id.clone(), asset))).collect();
    let object: semio_framework_pack_json::Object = resolved.into_iter().map(|(id, asset)| (id, semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&asset)))).collect();
    semio_framework_pack_json::to_string(&Value::Object(object))
}

/// 🎞️ The composite and navigator read the same document, session style and framework selection.
pub fn raster_scene(document: &RasterSnapshot, runtime: &RasterConfig, active_utility: &str, view_mode: &str, selected_ids: &[String], hovered_id: Option<&str>) -> semio_framework_plugin::Paint2dScene {
    semio_framework_plugin::Paint2dScene {
        document_sync_json: document_sync_json(document),
        assets_json: assets_json_from_document(document),
        camera_json: semio_framework_pack_json::to_json_string(&runtime.camera),
        selection_json: semio_framework_pack_json::to_json_string(&selected_ids.to_vec()),
        hovered_id: hovered_id.map(str::to_string),
        active_utility: active_utility.into(),
        brush_size: runtime.brush_size,
        brush_opacity: runtime.brush_opacity,
        brush_color: runtime.brush_color.clone(),
        brush_hardness: runtime.brush_hardness,
        paint_target:runtime.paint_target.clone(),
        mask_value:runtime.mask_value,
        fill_tolerance:runtime.fill_tolerance,
        pixel_selection_json:runtime.pixel_selection.as_ref().map(semio_framework_pack_json::to_json_string),
        view_mode: view_mode.into(),
        composite_viewport_json: runtime.composite_viewport.as_ref().map(semio_framework_pack_json::to_json_string),
        lanes: Vec::new(),
    }
}

/// 🎬️ Builds the semantic-UI action binding dispatched through the raster app's single controller — the
/// one call site every panel/window *node* goes through. Not for `WindowMeasure` chrome, which rides the
/// renderer's own [`ActionDescriptor`] record — see [`raster_measure_action`].
pub fn raster_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    ActionFactory::new(RASTER_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🎚️ Builds the window-chrome `ActionDescriptor` a [`WindowMeasure`] carries — the measure tree is a
/// renderer-side record, not a semantic-UI node, so it takes the descriptor verbatim rather than the
/// fixed-capacity `(ActionId, Option<UiValue>)` pair [`raster_action`] returns.
pub fn raster_measure_action(action: &str) -> ActionDescriptor {
    ActionDescriptor { controller_id: RASTER_PLAY_CONTROLLER_ID.into(), action: action.into(), args: None }
}

/// 🏷️ Admits one resolved raster string into the semantic UI contract's fixed-capacity label owner —
/// every panel/window label goes through here rather than the renderer's unbounded `Label`.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref().to_string()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "raster UI label admission failed"))
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

//#endregion 🔖️Document

//#region 🔖️ActionBridge
/// 🎯️ Host-action bridge into the closed `RasterCommand` enum (ticket
/// 26/09/05/RASTER-PLUGIN-END-TO-END, boot wave). The React/wgpu shells still speak `{action, args}`
/// with camelCase argument keys, while every `🎮️commands/*` payload derives `FromValue` over its own
/// snake_case field names — this boundary folds the keys and decodes. The `ArtifactEditor` default
/// refuses every app action outright (`app.command.unsupported`), which left `addLayer`/`setCamera`/
/// `setActiveExample`/… dead in the shell exactly as `📋️forms` found on 2026-09-16.
mod args_bridge {
    use super::*;
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
}
//#endregion 🔖️ActionBridge

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `RasterPlayApp::Command` — the SOLE dispatch surface for raster's own behavior, assembled from
    /// the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`,
    /// the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the binary/text codec uses) — different vocabularies, copied verbatim off the
    /// old `raster_protocol::RasterCommand` enum's `#[dsl(key)]` attributes and
    /// `RasterPlayApp::command_id` match arms respectively. **Row order is the binary variant ordinal:
    /// appending is safe, reordering is a wire-format break.**
    pub enum RasterCommand for RasterSnapshot, RasterMutation, RasterConfig, RasterConfigMutation {
        "addLayer" as "add-layer" => add_layer::AddLayer,
        "dropLayerKind" as "drop-layer-kind" => drop_layer_kind::DropLayerKind,
        "setLayerVisible" as "set-layer-visible" => set_layer_visible::SetLayerVisible,
        "toggleLayerVisible" as "toggle-layer-visible" => toggle_layer_visible::ToggleLayerVisible,
        "deleteLayer" as "delete-layer" => delete_layer::DeleteLayer,
        "duplicateLayer" as "duplicate-layer" => duplicate_layer::DuplicateLayer,
        "patchLayer" as "patch-layer" => patch_layer::PatchLayer,
        "patchLayers" as "patch-layers" => patch_layers::PatchLayers,
        "moveLayer" as "move-layer" => move_layer::MoveLayer,
        "setBrushSize" as "brush-size" => set_brush_size::SetBrushSize,
        "setBrushOpacity" as "brush-opacity" => set_brush_opacity::SetBrushOpacity,
        "setCompositeViewport" as "composite-viewport" => set_composite_viewport::SetCompositeViewport,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setCameraZoom" as "camera-zoom" => set_camera_zoom::SetCameraZoom,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "flattenLayers" as "flatten-layers" => flatten_layers::FlattenLayers,
        "mergeDown" as "merge-down" => merge_down::MergeDown,
        "maskFromSelection" as "mask-from-selection" => mask_from_selection::MaskFromSelection,
        "setBrushColor" as "brush-color" => set_brush_color::SetBrushColor,
        "setBrushHardness" as "brush-hardness" => set_brush_hardness::SetBrushHardness,
        "setPaintTarget" as "paint-target" => set_paint_target::SetPaintTarget,
        "setMaskValue" as "mask-value" => set_mask_value::SetMaskValue,
        "setPixelSelection" as "pixel-selection" => set_pixel_selection::SetPixelSelection,
        "exportPng" as "export-png" => export_png::ExportPng,
        "paintStroke" as "paint-stroke" => paint_stroke::PaintStroke,
        "fillRegion" as "fill-region" => fill_region::FillRegion,
        "setFillTolerance" as "fill-tolerance" => set_fill_tolerance::SetFillTolerance,
        "applyFilter" as "apply-filter" => apply_filter::ApplyFilter,
        "transformImage" as "transform-image" => transform_image::TransformImage,
        "fillSelection" as "fill-selection" => fill_selection::FillSelection,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::raster::commands::{set_active_example,export_png,set_paint_target,set_mask_value,set_fill_tolerance,set_pixel_selection};
use crate::editor::raster::commands::{mask_from_selection,flatten_layers,merge_down,paint_stroke,fill_region,apply_filter,transform_image,fill_selection};
use crate::editor::raster::commands::{add_layer, delete_layer, drop_layer_kind, duplicate_layer, move_layer, patch_layer, patch_layers, set_layer_visible, toggle_layer_visible};
use crate::editor::raster::commands::{set_brush_opacity, set_brush_size, set_brush_color, set_brush_hardness};
use crate::editor::raster::commands::{set_camera, set_camera_zoom, set_composite_viewport};
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
/// 🧵️ Document and session command routes; PNG download has a separate resumable host-only factory.
const RASTER_RETAINED_TOOL_IDS: &[&str] = &[
    "addLayer",
    "dropLayerKind",
    "setLayerVisible",
    "toggleLayerVisible",
    "deleteLayer",
    "duplicateLayer",
    "patchLayer",
    "patchLayers",
    "moveLayer",
    "setBrushSize",
    "setBrushOpacity",
    "setCompositeViewport",
    "setCamera",
    "setCameraZoom",
    "setActiveExample",
    "flattenLayers",
    "mergeDown",
    "maskFromSelection",
    "setBrushColor",
    "setBrushHardness",
    "setPaintTarget",
    "setMaskValue",
    "setPixelSelection",
    "paintStroke",
    "fillRegion",
    "setFillTolerance",
    "applyFilter",
    "transformImage",
    "fillSelection",
];
const RASTER_RETAINED_PAYLOAD_SCHEMA: &str = "raster.tool-command.v1";
const RASTER_RETAINED_RAW_BYTES: usize = 65_536;
const RASTER_RETAINED_WORK_ITEMS: usize = 4_096;
/// 🛣️ Publication lanes per route, read off each handler's own `Emit` in `🎮️commands/*/🦀️.rs` — the
/// document verbs build `Emit { artifact_mutations, .. }`/`Emit::mutations(..)` over
/// `RasterMutation` and never touch the config, while the session verbs build `Emit::config(..)`
/// over `RasterConfigMutation` and never touch the document. `paintStroke` alone also writes the window-transient lane:
/// a streamed stroke's tool state lives in the Composite window between its dispatches. No raster handler emits the
/// document and the config lane together, a draft or a presence.
///
/// 🎬️ `setActiveExample` is a document verb, not a session one: raster has no whole-document replace
/// mutation, so `🎮️commands/🎬️set-active-example` spells loading an example as an ordered batch of
/// real `RasterMutation`s (delete every root layer, re-point the asset pool, plant the example forest)
/// — the Artifact lane, exactly like every other layer verb.
///
/// `ActionKind::View` declarations in `🔖️Manifest` (ticket 26/07/31 — camera is runtime state, never a
/// document field, and there is no `RasterOperation::SetCamera`); `Config` is precisely the lane that
/// says "this route publishes into the config store, not into artifact history".
const RASTER_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "flattenLayers", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "mergeDown", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "paintStroke", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "fillRegion", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "applyFilter", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "transformImage", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "fillSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "maskFromSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addLayer", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "dropLayerKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setLayerVisible", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "toggleLayerVisible", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "deleteLayer", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "duplicateLayer", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchLayer", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchLayers", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "moveLayer", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setBrushSize", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setBrushOpacity", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setBrushColor", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setBrushHardness", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract {tool_id:"setPaintTarget",lanes:&[ArtifactToolPublicationLane::Config]},
    ArtifactToolPublicationContract {tool_id:"setMaskValue",lanes:&[ArtifactToolPublicationLane::Config]},
    ArtifactToolPublicationContract { tool_id: "setFillTolerance", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract {tool_id:"setPixelSelection",lanes:&[ArtifactToolPublicationLane::Config]},
    ArtifactToolPublicationContract { tool_id: "setCompositeViewport", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setCameraZoom", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

fn raster_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(RASTER_RETAINED_RAW_BYTES, 4_096, 1, 262_144, 7_500)
}

/// 📏️ One bounded first step per retained route, admitted only while the WHOLE layer tree (groups and
/// their nested children, via `flatten_raster_layers`) plus the asset pool plus this edit fit inside
/// `RASTER_RETAINED_WORK_ITEMS`. `layers.len()` alone would understate a grouped document, so the
/// recursive walk is used rather than the top-level count.
fn raster_retained_extent(command: &RasterCommand, snapshot: &RasterSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !RASTER_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return None;
    }
    let items = crate::standards::v1::subsets::any::schema::flatten_raster_layers(&snapshot.layers).len().checked_add(snapshot.assets.len())?.checked_add(1)?;
    (items <= RASTER_RETAINED_WORK_ITEMS).then_some(1)
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
fn raster_retained_reduce(
    command: &RasterCommand,
    snapshot: &RasterSnapshot,
    config: &RasterConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<RasterPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<RasterMutation, RasterConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config, window: None })
}

/// 🏭️ The app-owned retained command job factory — `factory_type:` in the proof block below binds this
/// exact Rust type to `EditorApp<RasterPlayApp>`, which is what turns a bare (and therefore
/// `interactive-job.missing-owned-reducer`) proof into an exact-owner proof.
struct RasterRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl RasterRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: RASTER_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for RasterRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<RasterPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<RasterPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        RASTER_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        raster_retained_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > RASTER_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Raster retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for RasterRetainedCommandJobFactory {
    type Owner = EditorApp<RasterPlayApp>;
    const TOOL_IDS: &'static [&'static str] = RASTER_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = RASTER_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = RASTER_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

struct RasterCommandProofs;
impl RasterCommandProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<RasterPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.raster.raster@1/*#editor",
        artifact_schema: "raster.document",
        factory: "RasterRetainedCommandJobFactory",
        factory_type: RasterRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "addLayer", "dropLayerKind", "setLayerVisible", "toggleLayerVisible", "deleteLayer", "duplicateLayer", "patchLayer", "patchLayers", "moveLayer",
            "setBrushSize", "setBrushOpacity", "setCompositeViewport", "setCamera", "setCameraZoom", "setActiveExample", "flattenLayers", "mergeDown", "maskFromSelection", "setBrushColor", "setBrushHardness", "setPaintTarget", "setMaskValue", "setPixelSelection", "paintStroke", "fillRegion", "setFillTolerance", "applyFilter", "transformImage", "fillSelection"
        ]
    }

}
struct RasterDownloadProofs;
impl RasterDownloadProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<RasterPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.raster.raster@1/*#editor",
        artifact_schema: "raster.document",
        factory: "RasterDownloadJobFactory",
        factory_type: media_export::RasterDownloadJobFactory,
        tools: {"exportPng" => ToolExecutionContract::resumable(4096,4096,1,33_554_432,2000,64,1)}
    }
}

//#region 📬️StorePreparation
/// 📬️ The document lane's one-item retained preparation. Without it every route declaring
/// `ArtifactToolPublicationLane::Artifact` is registered with an unsupported publication contract and
/// stays dispatch-dead, no matter how it is classified.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct RasterStorePreparationFactory;

struct RasterStorePreparation {
    base: Option<store::SnapshotRead<RasterSnapshot>>,
    mutation: Option<RasterMutation>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<RasterSnapshot, RasterMutation>>,
    /// 🧮 The clone-free stepwise apply (`RasterOneItemApply`), live from the first `advance` until
    /// the post snapshot is handed over, or closed through its own retirement on cancel/fault.
    apply: Option<crate::host::owned::RasterOneItemApply>,
    /// 🧹️ A cancelled/faulted item's mutation may carry a populated owned map (a `create-layer` of an
    /// adjustment with params) that must never reach `Drop` — it retires through the mutation
    /// retirement factory instead.
    mutation_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

/// ⛽️ Candidate fuel units one store grant buys — each unit is one bounded owned-value step (a field
/// clone, a page shift), so a two-layer document lands within a handful of grants.
const RASTER_ONE_ITEM_APPLY_FUEL: u64 = 256;

impl store::ArtifactStoreOneItemPreparationFactory<RasterSnapshot, RasterMutation> for RasterStorePreparationFactory {
    fn preflight(&self, mutation: &RasterMutation, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document {
            return Err("Raster Store preparation rejected its lane".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<RasterSnapshot, RasterMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<RasterSnapshot, RasterMutation>>, store::ArtifactStoreOneItemPreparationRequest<RasterSnapshot, RasterMutation>> {
        let item_count = crate::standards::v1::subsets::any::schema::flatten_raster_layers(&request.base.get().layers).len().saturating_add(request.base.get().assets.len());
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || item_count > RASTER_RETAINED_WORK_ITEMS
        {
            return Err(request);
        }
        Ok(Box::new(RasterStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            authority: Some(request.authority),
            prepared: None,
            apply: None,
            mutation_retirement: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<RasterSnapshot, RasterMutation> for RasterStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Raster preparation lost its exact base root".to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Raster preparation lost its Store authority".to_string())?;
        // 🧮 Clone-free apply: `Mutation::diff` + `RasterDiff::apply` would `Clone` the base and every
        // carried layer (a populated `RasterOwnedMap` asserts) and refuse a populated asset map — the
        // retained candidate authority builds the post snapshot one owned value at a time instead.
        let post = {
            let mutation = self.mutation.as_ref().ok_or_else(|| "Raster preparation lost its mutation owner".to_string())?;
            let apply = self.apply.get_or_insert_with(crate::host::owned::RasterOneItemApply::new);
            match apply.advance(base.get(), mutation, authority.operation(), authority.generation(), RASTER_ONE_ITEM_APPLY_FUEL).map_err(semio_framework_value::ValueError::into_message)? {
                Some(post) => post,
                None => return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint)),
            }
        };
        drop(self.apply.take());
        let mutation = self.mutation.take().ok_or_else(|| "Raster preparation lost its mutation owner".to_string())?;
        let inverse = mutation.inverse(base.get()).map_err(semio_framework_value::ValueError::into_message)?;
        let edit = authority.next_edit(mutation, inverse);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<RasterSnapshot, RasterMutation>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<RasterSnapshot, RasterMutation>> {
        self.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(apply) = self.apply.as_mut() {
            let step = apply.close_step(grant.maximum_items, grant.maximum_bytes)?;
            if apply.terminal_is_empty() {
                self.apply = None;
            }
            return Ok(match step {
                store::SnapshotRetirementStep::Complete => store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 },
                other => other,
            });
        }
        if let Some(mutation) = self.mutation.take() {
            self.mutation_retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&crate::host::owned::RasterMutationRetirementFactory, mutation));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(retirement) = self.mutation_retirement.as_mut() {
            let step = retirement.close_step(grant.maximum_items, grant.maximum_bytes)?;
            if retirement.terminal_is_empty() {
                self.mutation_retirement = None;
            }
            return Ok(match step {
                store::SnapshotRetirementStep::Complete => store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 },
                other => other,
            });
        }
        if self.prepared.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Raster preparation could not return its exact base root"));
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.apply.is_none() && self.mutation_retirement.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}

/// 📬️ The config lane's twin of {@link RasterStorePreparationFactory} — raster's seven session verbs
/// (`setBrushSize`/`setBrushOpacity`/`setCompositeViewport`/`setCamera`/`setCameraZoom`/
/// publication contract outright when this factory is absent. `RasterConfig`'s `Diff` is its own sparse
/// per-field diff.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct RasterConfigStorePreparationFactory;

struct RasterConfigStorePreparation {
    base: Option<store::SnapshotRead<RasterConfig>>,
    mutation: Option<RasterConfigMutation>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<RasterConfig, RasterConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<RasterConfig, RasterConfigMutation> for RasterConfigStorePreparationFactory {
    fn preflight(&self, mutation: &RasterConfigMutation, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document {
            return Err("Raster config preparation rejected its lane".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<RasterConfig, RasterConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<RasterConfig, RasterConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<RasterConfig, RasterConfigMutation>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(RasterConfigStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<RasterConfig, RasterConfigMutation> for RasterConfigStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Raster config preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Raster config preparation lost its mutation owner".to_string())?;
        let inverse = mutation.inverse(base.get()).map_err(semio_framework_value::ValueError::into_message)?;
        let post = protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Raster config preparation lost its Store authority".to_string())?;
        let edit = authority.next_edit(mutation, inverse);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<RasterConfig, RasterConfigMutation>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<RasterConfig, RasterConfigMutation>> {
        self.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Raster config preparation could not return its exact base root"));
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️StorePreparation

//#region 🎞️ReservedImport
const RASTER_IMPORT_TOOL_ID: &str = "import-media";
const RASTER_IMPORT_PORT: &str = "image:in";

/// 🎞️ The ONE concrete resumable importer this app owns. The framework registers the reserved
/// `import-media` factory for every app but never a concrete job, so `dispatch_import_media` →
/// `build_artifact_reserved_media_job` fails closed with `interactive-job.missing-reserved-builder`
/// until the app hands one back from `build_reserved_tool_job` — `ArtifactApp::import_media`'s
/// unbounded one-shot seam is no longer on any live route. Two bounded steps: decode the incoming
/// base64 PNG into this artifact's own `(asset_id, asset, layer)` triple, then publish the two real
/// semantic mutations (`add-layer-asset` then `create-layer`, in dependency order) through the
/// completion authority. The decode itself is `crate::standards::v1::subsets::any::io::raster_image_layer_and_asset` — the SAME
/// function the pure seam used — so the import's meaning lives in one place.
struct RasterImportJob {
    port: String,
    media_json: Option<String>,
    snapshot: Option<std::sync::Arc<RasterSnapshot>>,
    mutations: Vec<RasterMutation>,
    decoded: bool,
    completed: bool,
    closing: bool,
    completion: Option<ArtifactToolCompletion<EditorApp<RasterPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<RasterPlayApp>>>,
}

fn raster_job_payload(cx: &mut StepContext<'_>, stream: JobPayloadStream, bytes: &[u8]) -> RetainedJobPayload {
    match cx.payload_from_bytes(stream, bytes) {
        Ok(payload) => payload,
        Err(rejected) => {
            drop(rejected.into_source());
            RetainedJobPayload::empty(stream)
        }
    }
}

fn raster_job_fault(cx: &mut StepContext<'_>, detail: &str) -> StepOutcome {
    let bytes = detail.as_bytes();
    let bounded = &bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)];
    StepOutcome::Fault(JobFault { detail: raster_job_payload(cx, JobPayloadStream::Fault, bounded) })
}

impl RasterImportJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<RasterPlayApp>>, port: String, media: Media) -> Self {
        let media_json = match media.payload {
            MediaPayload::Structured { json, .. } => Some(json),
            MediaPayload::Binary { .. } | MediaPayload::Intrinsic { .. } => None,
        };
        Self { port, media_json, snapshot: Some(request.snapshot), mutations: Vec::new(), decoded: false, completed: false, closing: false, completion: Some(request.completion), pending_completion_rejection: None }
    }

    fn decode(&mut self, cx: &mut StepContext<'_>) -> Option<StepOutcome> {
        if self.port != RASTER_IMPORT_PORT {
            return Some(raster_job_fault(cx, "raster import only implements image:in"));
        }
        let Some(media_json) = self.media_json.as_ref() else {
            return Some(raster_job_fault(cx, "raster image:in only accepts a Structured (base64 PNG) payload"));
        };
        let Some(snapshot) = self.snapshot.as_ref() else {
            return Some(raster_job_fault(cx, "raster import lost its snapshot authority"));
        };
        let index = snapshot.layers.len();
        let (asset_id, asset, layer) = match crate::standards::v1::subsets::any::io::raster_image_layer_and_asset(media_json) { Ok(value) => value, Err(error) => return Some(raster_job_fault(cx, &error)) };
        self.mutations = vec![
            RasterMutation::AddLayerAsset(crate::mutations::add_layer_asset::mutation::AddLayerAsset { asset_id, asset }),
            RasterMutation::CreateLayer(crate::mutations::create_layer::mutation::CreateLayer { parent_id: None, index, layer: Box::new(layer) }),
        ];
        self.decoded = true;
        None
    }
}

impl InteractiveJob for RasterImportJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return raster_job_fault(cx, "raster import completion remains rejected");
        }
        if !self.decoded {
            cx.set_stage("raster-import-decode");
            if let Some(outcome) = self.decode(cx) {
                return outcome;
            }
            cx.consume_fuel(1);
            return StepOutcome::CheckpointReady(Checkpoint { state: raster_job_payload(cx, JobPayloadStream::CheckpointState, &[1]), applied_progress: 1 });
        }
        cx.set_stage("raster-import-publish");
        if !self.completed {
            let mutations = std::mem::take(&mut self.mutations);
            let Some(completion) = self.completion.as_ref() else {
                return raster_job_fault(cx, "raster import lost its completion authority");
            };
            if !completion.has_mounted_consumer() {
                return raster_job_fault(cx, "raster import completion consumer is absent");
            }
            if let Err(rejected) = completion.complete(Ok(Emit { artifact_mutations: mutations, ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }), EphemeralEmit::default()) {
                let message = rejected.fault.message.clone();
                self.pending_completion_rejection = Some(rejected);
                return raster_job_fault(cx, &message);
            }
            self.completed = true;
        }
        StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        match ArtifactReservedJob::close_step(self, maximum_items, maximum_bytes) {
            Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes }) => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
            Ok(semio_framework_plugin::PluginCloseStep::AwaitingInput { .. } | semio_framework_plugin::PluginCloseStep::Blocked { .. }) | Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
            Ok(semio_framework_plugin::PluginCloseStep::Complete) if ArtifactReservedJob::terminal_is_empty(self) => semio_framework_job::InteractiveJobCloseStep::Complete,
            Ok(semio_framework_plugin::PluginCloseStep::Complete) => semio_framework_job::InteractiveJobCloseStep::Blocked,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        ArtifactReservedJob::terminal_is_empty(self)
    }
}

impl ArtifactReservedJob for RasterImportJob {
    /// 🧯️ `CreateLayer` owns a layer subtree (and through an `Adjustment` a fail-closed
    /// `RasterOwnedMap`), so an abandoned mutation is retired through the artifact's own
    /// `retire_raster_mutation` rather than dropped.
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        self.closing = true;
        if maximum_items == 0 {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            if let Ok(emit) = rejected.emit.as_mut() {
                if let Some(step) = emit.close_child_one(maximum_items, maximum_bytes) {
                    return Ok(step);
                }
            }
            self.pending_completion_rejection = None;
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(mutation) = self.mutations.pop() {
            crate::standards::v1::subsets::any::schema::mutations::retire_raster_mutation(mutation);
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.mutations.capacity() > 0 {
            self.mutations = Vec::new();
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.media_json.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.port.is_empty() || self.port.capacity() > 0 {
            self.port = String::new();
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.snapshot.as_ref().is_some_and(|snapshot| std::sync::Arc::strong_count(snapshot) == 1) {
            return Ok(semio_framework_plugin::PluginCloseStep::Blocked { reason: "raster import snapshot has no mounted retained authority" });
        }
        if self.snapshot.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.completion.as_ref().is_some_and(|completion| !completion.has_mounted_consumer()) {
            return Ok(semio_framework_plugin::PluginCloseStep::Blocked { reason: "raster import completion has no mounted consumer authority" });
        }
        if self.completion.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(semio_framework_plugin::PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.port.is_empty()
            && self.port.capacity() == 0
            && self.media_json.is_none()
            && self.snapshot.is_none()
            && self.mutations.is_empty()
            && self.mutations.capacity() == 0
            && self.completion.is_none()
            && self.pending_completion_rejection.is_none()
    }
}
//#endregion 🎞️ReservedImport

//#region 🔖️RasterPlayApp
/// 🧪️ B1: unit struct — every former `RasterConfig` field now lives in
/// `crate::editor::raster::config::RasterConfig`, written through `RasterConfigMutation`s.
#[derive(Default)]
pub struct RasterPlayApp;

impl ArtifactEditor for RasterPlayApp {
    /// 📢️ The localized notices of the raster tool refusals (design §20.12).
    fn fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
        paint_stroke::raster_fault_notices()
    }

    /// ⏳️ The layers panel renders the live operations with their Cancel controls; nothing else shows them.
    fn operation_progress_scope() -> semio_framework::kernel::UiDirtyScope {
        semio_framework::kernel::UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![crate::editor::raster::panels::document::RASTER_PLAY_BODY_LAYERS.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false }
    }

    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::art_raster_demo::source()]
    }
    type Snapshot = RasterSnapshot;
    type Mutation = RasterMutation;
    type Config = RasterConfig;
    type ConfigMutation = RasterConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = RasterPresence;
    type PresenceMutation = RasterPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = RasterCommand;

    const DIALECT: Dialect = crate::RASTER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = RASTER_DOCUMENT_SCHEMA;

    /// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields.
    /// Without it every live envelope load faults with `editor did not declare a loaded-parent child
    /// projection` before the decoded document can replace the store. `assets` declares no child slot
    /// (see `🧬️schema/📸️snapshot`), so the projection is honestly empty.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("raster.child-projection"), error.to_string()))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(RasterStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(RasterConfigStorePreparationFactory))
    }

    fn bounded_first_step_tool_proofs()->Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        RasterCommandProofs::bounded_first_step_tool_proofs().into_iter().chain(RasterDownloadProofs::bounded_first_step_tool_proofs()).collect()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(RasterRetainedCommandJobFactory::new(&controller_id))?;
        registry.register(media_export::RasterMediaExportJobFactory::new(&controller_id))?;
        registry.register(media_export::RasterDownloadJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if request.tool_id=="exportPng" {return media_export::build_download_job(request);}
        if !RASTER_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "Raster command does not match its exact registered tool"));
        }
        if raster_retained_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("raster.document.too-large"), "the raster document exceeds the retained command's work bound"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = if tool_id == "paintStroke" {
            Box::new(paint_stroke::PaintStrokeWork::default())
        } else if tool_id == "flattenLayers" {
            Box::new(flatten_layers::LayerBakeWork::<false>::default())
        } else if tool_id == "mergeDown" {
            Box::new(flatten_layers::LayerBakeWork::<true>::default())
        } else if tool_id == "maskFromSelection" {
            Box::new(mask_from_selection::MaskFromSelectionWork::default())
        } else {
            Box::new(BoundedArtifactCommandWork::new(tool_id, raster_retained_reduce, raster_retained_extent))
        };
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            RasterCommand::command_id,
            RASTER_RETAINED_RAW_BYTES,
            RASTER_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::host::owned::raster_envelope_decode_owner_bundle())
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::host::owned::raster_document_store_owners())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::host::owned::raster_document_store_initialization_job(envelope, operation, generation, actor))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    /// 🧹️ The config and draft lanes' owner catalogs + bounded disposers (forms precedent) — without
    /// them a closing instance faults `interactive-job.close-owned-disposer-missing … config-store`
    /// and then "artifact store has no owner-supplied bounded disposer" (found by the mounted boot
    /// test of ticket 26/09/05/RASTER-PLUGIN-END-TO-END).
    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    /// 👥️ Presence/transient lanes: raster's `RasterPresence` is plain inline data (its own
    /// one-turn retirement in `👥️presence/🦀️.rs`), the transient lane is `NoTransient` — the same
    /// rows forms/process3d declare.
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(crate::editor::raster::presence::raster_presence_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(crate::editor::raster::presence::RasterPresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(crate::editor::raster::presence::RasterPresenceRetirementFactory))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 🫧️ The Composite window's transient partition — where a streamed stroke lives between its dispatches.
    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        composite::transient::register(registry)
    }

    /// 📨️ Every host event ends the window's open stroke with zero trace under the reason the tool records: a blur
    /// `blur`, a lost pointer capture `captureLost`, a utility switch or a closing window `retired`, an opened history
    /// edit `frozen`. A remote edit (`BaseMoved`) keeps the stroke: its leaf names points and brush and repaints on any
    /// base. A window with no stroke in flight takes no write.
    fn host_event(event: &semio_framework_plugin::HostEvent) -> Option<RasterCommand> {
        use semio_framework_plugin::HostEvent;
        use semio_framework_tool_machine::ToolAbortReason;
        let reason = match event {
            HostEvent::WindowBlurred { .. } => ToolAbortReason::Blur,
            HostEvent::PointerCaptureLost { .. } => ToolAbortReason::CaptureLost,
            HostEvent::UtilityChanged { .. } | HostEvent::Retiring { .. } => ToolAbortReason::Retired,
            HostEvent::TimeTravelFrozen { .. } => ToolAbortReason::Frozen,
            HostEvent::BaseMoved { .. } => return None,
        };
        Some(RasterCommand::PaintStroke(paint_stroke::PaintStroke { layer_id: String::new(), tool: String::new(), xs: Vec::new(), ys: Vec::new(), phase: Some("abort".to_string()), reason: Some(reason.as_str().to_string()), gesture: None }))
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::raster::config::schema::app_schema_descriptor())
    }

    /// 📄️ Boots on the constant EMPTY shell `empty_raster_snapshot()` (zero layers, zero assets) —
    /// NOT the `📚️examples/🎬️demo` carrier and not even `empty_raster_document()`'s one Background
    /// layer. The document is event-sourced: the initial snapshot is the constant every replica
    /// agrees on, and the shell replays `setActiveExample demo` on every boot so the demo lands
    /// through the bounded `set-active-example` mutations (the same document flow block2d/forms
    /// use). Found at the first react boots of ticket 26/09/05/RASTER-PLUGIN-END-TO-END
    /// (2026-09-16).
    fn initial_snapshot() -> RasterSnapshot {
        crate::standards::v1::subsets::any::io::text::snapshot::empty_raster_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(raster_io())
    }

    /// 📤️ Captures image output in a retained operation with framework-owned cancellation and disposal.
    fn build_media_export_job(request:semio_framework_plugin::ArtifactMediaExportJobRequest<EditorApp<Self>>)->Result<Option<ArtifactReservedToolJob>,Fault> {
        if request.port!="image:out"||request.tool_id!=media_export::TOOL_ID {return Ok(None);}
        Ok(Some(ArtifactReservedToolJob::new(media_export::RasterImageExportJob::new(request))))
    }

    fn build_snapshot_disposer()->Option<Box<dyn semio_framework_plugin::ArtifactSnapshotDisposer<Self::Snapshot>>> {
        Some(Box::new(media_export::RasterExportSnapshotDisposer::default()))
    }

    fn interaction_topology(doc:&ArtifactView<'_,RasterSnapshot>,_cfg:&ConfigView<'_,RasterConfig>)->Result<semio_framework_plugin::InteractionTopology, semio_framework_value::ValueError> {
        Ok(semio_framework_plugin::InteractionTopology {domains:std::collections::BTreeMap::from([(RASTER_INTERACTION_DOMAIN.into(),layer_selection::layer_topology(&doc.snapshot.layers))])})
    }

    /// 📦️ Batch serializers for structured image media and editable artifact packs.
    fn export_media(port: &str, doc: &ArtifactView<'_, RasterSnapshot>) -> Result<Media, MediaError> {
        match port {
            "image:out" => raster_composite_media(doc.snapshot),
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `image:in` inserts the incoming raster media as a new composited layer + embedded asset —
    /// two real semantic mutations (`add-layer-asset` then `create-layer`, in dependency order)
    /// bundled in one `Emit`, never a whole-document replace (`RasterMutation` has no such variant
    /// anymore). Falls through to the inherited `document:in` load default for any other port.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, RasterSnapshot>) -> Result<Emit<RasterMutation, RasterConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "image:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json: png_base64, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "image:in only accepts a Structured (base64 PNG) payload".into()));
        };
        let (asset_id, asset, layer) = crate::standards::v1::subsets::any::io::raster_image_layer_and_asset(png_base64).map_err(|error| MediaError::Payload(port.to_string(), error))?;
        Ok(Emit::mutations(vec![
            RasterMutation::AddLayerAsset(crate::mutations::add_layer_asset::mutation::AddLayerAsset { asset_id, asset }),
            RasterMutation::CreateLayer(crate::mutations::create_layer::mutation::CreateLayer { parent_id: None, index: doc.snapshot.layers.len(), layer: Box::new(layer) }),
        ]))
    }

    /// 🎞️ The ONE reserved route raster owns: `import-media`. Every inbound media delivery goes
    /// through `dispatch_import_media` → `build_artifact_reserved_media_job`, which fails closed
    /// unless this builder hands back a concrete resumable importer — see [`RasterImportJob`].
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if request.tool_id.as_str() != RASTER_IMPORT_TOOL_ID {
            return Ok(None);
        }
        if !request.raw_wire.is_empty() {
            return Err(Fault::from("raster import-media admits a decoded media value, never a wire payload"));
        }
        let ArtifactReservedToolInput::Media { port, media } = &request.input else {
            return Err(Fault::from("raster import-media requires media input"));
        };
        let (port, media) = (port.clone(), media.clone());
        Ok(Some(ArtifactReservedToolJob::new(RasterImportJob::new(request, port, media))))
    }

    fn command_id(command: &RasterCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ See `args_bridge` — without this override the trait default refuses every shell action.
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        args_bridge::command_from_action(action, args)
    }

    fn handle(
        command: &RasterCommand,
        doc: &ArtifactView<'_, RasterSnapshot>,
        cfg: &ConfigView<'_, RasterConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<RasterMutation, RasterConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn window_measures(_doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(composite::RASTER_PLAY_WINDOW_COMPOSITE.into(), composite::window_measures(cfg.snapshot, raster_play_labels(view_state)))])
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_raster_body(body_key, doc, None, cfg, view_state, &[], None)
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, RasterSnapshot>,
        cfg: &ConfigView<'_, RasterConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let preview = match body_key {
            composite::RASTER_PLAY_BODY_COMPOSITE => transient.window::<composite::transient::RasterCompositeWindowTransientOwner>().and_then(|window| paint_stroke::raster_stroke_preview(doc.snapshot, window)),
            _ => None,
        };
        let rendered = render_raster_body(body_key, doc, preview.as_ref(), cfg, view_state, &interaction.selection(RASTER_INTERACTION_DOMAIN).ids, interaction.hover(RASTER_INTERACTION_DOMAIN, "pointer").ids.first().map(String::as_str));
        if let Some(preview) = preview {
            crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(preview);
        }
        rendered
    }
}
/// 🖼️ Projects live selection into both paint windows without a second selection store; the Composite window paints
/// `preview` (the committed document with its stroke in flight) when it holds one.
fn render_raster_body(body_key: &str, doc: &ArtifactView<'_, RasterSnapshot>, preview: Option<&RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>, view_state: &semio_framework_plugin::ViewModel, selected_ids: &[String], hovered_id: Option<&str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = preview.unwrap_or(doc.snapshot);
        let config = cfg.snapshot;
        let active_utility = view_state.active_utility_id.as_deref().unwrap_or("selectMarquee");
        let labels = raster_play_labels(view_state);
        let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
        let node = match body_key {
            composite::RASTER_PLAY_BODY_COMPOSITE => composite::render(document, config, active_utility, selected_ids, hovered_id)?,
            navigator::RASTER_PLAY_BODY_NAVIGATOR => navigator::render(document, config, active_utility, selected_ids, hovered_id)?,
            crate::editor::raster::panels::document::RASTER_PLAY_BODY_LAYERS => crate::editor::raster::panels::document::render(document, config, labels, &windows)?,
            crate::editor::raster::panels::masks::RASTER_PLAY_BODY_MASKS => crate::editor::raster::panels::masks::render(document, config, labels, &windows)?,
            crate::editor::raster::panels::catalogue::RASTER_PLAY_BODY_CATALOGUE => crate::editor::raster::panels::catalogue::render(labels, &windows)?,
            crate::editor::raster::panels::inspection::RASTER_PLAY_BODY_PROPERTIES => crate::editor::raster::panels::inspection::render(document, config, selected_ids, labels)?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "raster unknown-body label admission failed"))?,
        };
        let node=if body_key==crate::editor::raster::panels::document::RASTER_PLAY_BODY_LAYERS&&!doc.operations().is_empty() {
            use semio_framework_plugin::{Buildable,HasBase,HasChildren};
            let progress=semio_framework_plugin::app::operation_progress::operation_progress_controls(doc.operations(),RASTER_PLAY_CONTROLLER_ID,view_state.locale)?;
            semio_framework_ui_contract::column().try_id("raster.layers.with-progress").map_err(|_|semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity","raster progress container"))?.try_children([progress,node]).map_err(|_|semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity","raster progress children"))?.try_build().map_err(|_|semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity","raster progress build"))?
        }else{node};
        Ok(semio_framework_plugin::built_to_component_tree(node))
}
//#endregion 🔖️RasterPlayApp

//#region 🔖️Io
/// 🔌️ Relocated verbatim from `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
/// rule 4: anything returning `AppIo` or referencing an app type lives in `🎛️apps/<app>/`). This app's
/// typed media I/O surface (`AppDefinition.io`) — mirrors the `2d.raster` `ArtifactKindSpec` literal
/// `crate::artifact_kind` already declares, plus the app-specific `image:in`/
/// `image:out` ports (see below).
pub fn raster_io() -> semio_framework::AppIo {
    semio_framework::AppIo {
        artifact_schema: RASTER_DOCUMENT_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        ports: vec![raster_image_in_port(), raster_image_out_port()],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework::ArtifactPresentation { id: "2d.raster".into(), name: "2D Raster".into(), dimension: "2d".into(), component_kind: "raster".into() },
    }
}

/// 🔌️ `image:in` — accepts raster imagery from upstream producers (e.g. draw's `vector:out`,
/// converted Vector→Raster) as a new composited layer. `Many`/optional: several upstream images may
/// feed in, and the port may sit unconnected.
pub fn raster_image_in_port() -> semio_framework::MediaPortSpec {
    semio_framework::MediaPortSpec {
        id: "image:in".into(),
        label: "Image".into(),
        direction: semio_framework::MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        kind_id: None,
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🔌️ `image:out` — the raster document's current composited raster, as `2d.image` media (workflow
/// port surface; WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe). `kind_id: "2d.image"` — the
/// shared framework-builtin interchange kind (declared on this app's `.artifact_kind(...)` below;
/// `shooting`'s `photos:out` declares the identical shape, harmless duplicate registrations).
pub fn raster_image_out_port() -> semio_framework::MediaPortSpec {
    semio_framework::MediaPortSpec {
        id: "image:out".into(),
        label: "Image".into(),
        direction: semio_framework::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        kind_id: Some("2d.image".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🖼️ Publishes the canonical layer composite as a PNG image port payload.
pub fn raster_composite_media(document: &RasterSnapshot) -> Result<Media, MediaError> {
    let image=crate::standards::v1::subsets::any::io::raster_composite_image(document).map_err(|error|MediaError::Payload("image:out".into(),error))?;
    let bytes=crate::standards::v1::subsets::any::io::png_bytes_from_semio_image(&image).map_err(|error|MediaError::Payload("image:out".into(),error))?;
    let png_base64=base64_codec::base64_standard_encode(bytes);
    Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: png_base64 } })
}
//#endregion 🔖️Io

//#region 🔖️Manifest
/// 🛠️ An internal (non-palette) action declaration — the panel/pointer/gesture-bound vocabulary
/// dispatched by the layer tree, catalogue drops and inspector, never a palette command.
fn raster_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> semio_framework_plugin::ActionDefinition {
    semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog(id, label, kind) }
}

/// 🧰️ One composite-window utility declaration; ids must stay host-compatible (`paint*` prefix paints,
/// `paintEraser` erases, `paintBucket` fills, `selectMarquee` selects) because the scene's active utility feeds `RasterHost`.
fn raster_utility(id: &str, label: impl Into<LocalizedLabel>, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding/utility declarations (which have no dedicated `_def` passthrough) are
/// written out inline.
///
/// 📚️ The navbar dropdown reads `RasterPlayApp::examples()`, which `.editor` stamps onto the
/// manifest. The catalogue lives on the artifact, not on the plugin root.
pub fn create_raster_app() -> AppDefinition {
    Editor::builder(crate::RASTER_DIALECT).document(["semio", "raster"])
            .artifact_kind(crate::artifact_kind())
            // 🖼️ `2d.image` — the interchange kind `image:out` produces (WORKFLOWS-END-TO-END-TYPED-PORTS
            // Wave 2 port recipe); `shooting`'s `photos:out` already declares the identical shape — a
            // harmless duplicate registration (registry dedupes by id).
            .artifact_kind(ArtifactKindSpec {
                id: "2d.image".into(),
                label: semio_framework_ui_locale::LocalizedLabel::native("2D Image", "2D-Bild"),
                source_format: "2d.image".into(),
                component_kind: "image".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
                schema: "2d.image".into(),
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec!["stdio.png".into()],
        import_stdio_kinds: vec!["stdio.png".into()],
    })
            .icon_id("raster")
            .mode_def(edit::definition())
            .default_mode_id(edit::RASTER_PLAY_MODE_EDIT)
            .window_kind_def(composite::definition())
            .window_kind_def(navigator::definition())
            .default_layout(edit::layout())
            .panel_tab_def(crate::editor::raster::panels::document::definition())
            .panel_tab_def(crate::editor::raster::panels::catalogue::definition())
            .panel_tab_def(crate::editor::raster::panels::masks::definition())
            .panel_tab_def(crate::editor::raster::panels::inspection::definition())
            // ✏️ Palette-visible content operations. `setSnapshot` — the old whole-document replace —
            // stays gone (`🎮️commands/📃️document/🦀️.rs` records why); `setActiveExample` is back as a
            // real, undoable batch of `RasterMutation`s rather than a snapshot swap, so it is an
            // ordinary palette mutation like block2d's and puzzle3d's.
            .shell_action("exportPng", LocalizedLabel::native("Export PNG", "PNG exportieren"))
            .action_describe("exportPng", LocalizedLabel::native("Downloads the visible image as PNG with transparency. Layers remain editable.", "Lädt das sichtbare Bild als PNG mit Transparenz herunter. Die Ebenen bleiben bearbeitbar."))
            .action_interactive_job("exportPng", InteractiveJobClassification::Migrated)
            .mutation("addLayer", LocalizedLabel::native("Add Layer", "Ebene hinzufügen"))
            .action_with(raster_internal_action("flattenLayers", LocalizedLabel::native("Flatten Image", "Bild reduzieren"), ActionKind::Mutation))
            .action_describe("flattenLayers", LocalizedLabel::native("Replaces all layers with the visible image. Undo restores the original layers.", "Ersetzt alle Ebenen durch das sichtbare Bild. Rückgängig stellt die ursprünglichen Ebenen wieder her."))
            .action_interactive_job("flattenLayers", InteractiveJobClassification::Migrated)
            .action_with(raster_internal_action("mergeDown", LocalizedLabel::native("Merge Down", "Nach unten vereinen"), ActionKind::Mutation))
            .action_describe("mergeDown", LocalizedLabel::native("Merges the selected visible layer with its lower normal-blend sibling. Undo restores both layers.", "Vereint die ausgewählte sichtbare Ebene mit der darunterliegenden Ebene im normalen Mischmodus. Rückgängig stellt beide Ebenen wieder her."))
            .action_interactive_job("mergeDown", InteractiveJobClassification::Migrated)
            .action_with(raster_internal_action("paintStroke", LocalizedLabel::native("Paint Stroke", "Strich malen"), ActionKind::Mutation))
            .action_describe("paintStroke", LocalizedLabel::native("Paints one brush or eraser stroke on the selected layer's pixels or mask with the session brush. The whole stroke is one undoable edit whose brush and points can be edited in history.", "Malt einen Pinsel- oder Radierstrich mit dem Sitzungspinsel auf die Pixel oder die Maske der gewählten Ebene. Der ganze Strich ist ein rückgängig machbarer Schritt, dessen Pinsel und Punkte im Verlauf bearbeitet werden können."))
            .action_interactive_job("paintStroke", InteractiveJobClassification::Migrated)
            .action_with(raster_internal_action("fillRegion", LocalizedLabel::native("Fill Region", "Region füllen"), ActionKind::Mutation))
            .action_describe("fillRegion", LocalizedLabel::native("Fills the connected region around the clicked pixel of the selected layer's pixels or mask with the session colour, within the colour tolerance and the pixel selection. The fill is one undoable edit whose seed, tolerance and colour can be edited in history.", "Füllt die zusammenhängende Region um das angeklickte Pixel der Pixel oder der Maske der gewählten Ebene mit der Sitzungsfarbe, innerhalb der Farbtoleranz und der Pixelauswahl. Die Füllung ist ein rückgängig machbarer Schritt, dessen Startpixel, Toleranz und Farbe im Verlauf bearbeitet werden können."))
            .action_interactive_job("fillRegion", InteractiveJobClassification::Migrated)
            .action_with(raster_internal_action("applyFilter", LocalizedLabel::native("Apply Filter", "Filter anwenden"), ActionKind::Mutation))
            .action_describe("applyFilter", LocalizedLabel::native("Runs one image filter (invert, grayscale, clear, flip, brightness, contrast, saturation, gamma, threshold, posterize, blur or sharpen, with its amount) over the selected layer's pixels, within the pixel selection. The filter is one undoable edit whose filter and amount stay editable in history.", "Wendet einen Bildfilter (Invertieren, Graustufen, Leeren, Spiegeln, Helligkeit, Kontrast, Sättigung, Gamma, Schwellenwert, Tontrennung, Weichzeichnen oder Schärfen, mit seiner Stärke) auf die Pixel der gewählten Ebene innerhalb der Pixelauswahl an. Der Filter ist eine rückgängig machbare Bearbeitung, deren Filter und Stärke im Verlauf bearbeitbar bleiben."))
            .action_interactive_job("applyFilter", InteractiveJobClassification::Migrated)
            .action_with(raster_internal_action("transformImage", LocalizedLabel::native("Transform Image", "Bild umformen"), ActionKind::Mutation))
            .action_describe("transformImage", LocalizedLabel::native("Rotates the selected layer's image a quarter turn, resizes it or crops it to a window. The layer keeps its place and scale on the canvas; the change is one undoable edit whose operation and extent can be edited in history.", "Dreht das Bild der gewählten Ebene um eine Vierteldrehung, skaliert es oder schneidet es auf ein Fenster zu. Die Ebene behält Lage und Maßstab auf der Leinwand; die Änderung ist ein rückgängig machbarer Schritt, dessen Operation und Ausmaß im Verlauf bearbeitet werden können."))
            .action_interactive_job("transformImage", InteractiveJobClassification::Migrated)
            .action_with(raster_internal_action("fillSelection", LocalizedLabel::native("Fill Selection", "Auswahl füllen"), ActionKind::Mutation))
            .action_describe("fillSelection", LocalizedLabel::native("Fills the pixel selection of the selected layer's pixels or mask with the session colour (the mask value at the brush opacity), or the whole image without a selection. The fill is one undoable edit whose colour and selection can be edited in history.", "Füllt die Pixelauswahl der Pixel oder der Maske der gewählten Ebene mit der Sitzungsfarbe (dem Maskenwert mit der Pinseldeckkraft), ohne Auswahl das ganze Bild. Die Füllung ist ein rückgängig machbarer Schritt, dessen Farbe und Auswahl im Verlauf bearbeitet werden können."))
            .action_interactive_job("fillSelection", InteractiveJobClassification::Migrated)
            .action_with(raster_internal_action("maskFromSelection", LocalizedLabel::native("Mask From Selection", "Maske aus Auswahl"), ActionKind::Mutation))
            .action_describe("maskFromSelection", LocalizedLabel::native("Creates an undoable layer mask from the current pixel selection.", "Erstellt eine rückgängig machbare Ebenenmaske aus der aktuellen Pixelauswahl."))
            .action_interactive_job("maskFromSelection", InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            // 🔧️ Internal content operations — layer-tree / catalogue-drop / inspector bound.
            .action_with(raster_internal_action("setLayerVisible", LocalizedLabel::native("Set Layer Visible", "Ebenensichtbarkeit festlegen"), ActionKind::Mutation))
            .action_with(raster_internal_action("toggleLayerVisible", LocalizedLabel::native("Toggle Layer Visible", "Ebenensichtbarkeit umschalten"), ActionKind::Mutation))
            .action_with(raster_internal_action("dropLayerKind", LocalizedLabel::native("Drop Layer Kind", "Ebenenart ablegen"), ActionKind::Mutation))
            .action_audience("dropLayerKind", semio_framework_plugin::CapabilityAudience::Agent)
            .action_with(raster_internal_action("deleteLayer", LocalizedLabel::native("Delete Layer", "Ebene löschen"), ActionKind::Mutation))
            .action_with(raster_internal_action("duplicateLayer", LocalizedLabel::native("Duplicate Layer", "Ebene duplizieren"), ActionKind::Mutation))
            .action_with(raster_internal_action("patchLayer", LocalizedLabel::native("Patch Layer", "Ebene aktualisieren"), ActionKind::Mutation))
            .action_with(raster_internal_action("patchLayers", LocalizedLabel::native("Patch Layers", "Ebenen aktualisieren"), ActionKind::Mutation))
            .action_with(raster_internal_action("moveLayer", LocalizedLabel::native("Move Layer", "Ebene verschieben"), ActionKind::Mutation))
            // 🕹️ The framework-owned "layers" interaction domain (ticket
            // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — the layer tree's selection;
            // auto-injects interactionSelect/interactionHover/clearSelection/selectAll/
            // setSelectionMode/setInteractionGranularity, replacing the deleted bespoke
            // setSelection/setHover/selectAll actions below.
            .interaction(InteractionDefinition {
                id: RASTER_INTERACTION_DOMAIN.into(),
                label: LocalizedLabel::native("Layers", "Ebenen"),
                granularities: vec![GranularityDefinition { id: RASTER_INTERACTION_GRANULARITY.into(), label: LocalizedLabel::native("Layer", "Ebene"), icon_id: "image".into() }],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(composite::RASTER_PLAY_WINDOW_COMPOSITE, vec![InteractionRef::new(RASTER_INTERACTION_DOMAIN)])
            // 👁️ Ephemeral view state — live brush controls, navigator viewport, camera.
            .action_with(raster_internal_action("setBrushSize", LocalizedLabel::native("Set Brush Size", "Pinselgröße festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setBrushOpacity", LocalizedLabel::native("Set Brush Opacity", "Pinseldeckkraft festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setBrushColor", LocalizedLabel::native("Set Foreground Color", "Vordergrundfarbe festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setBrushHardness", LocalizedLabel::native("Set Brush Hardness", "Pinselhärte festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setPaintTarget",LocalizedLabel::native("Set Paint Target","Bearbeitungsziel festlegen"),ActionKind::View))
            .action_with(raster_internal_action("setPixelSelection",LocalizedLabel::native("Set Pixel Selection","Pixelauswahl festlegen"),ActionKind::View))
            .action_with(raster_internal_action("setMaskValue",LocalizedLabel::native("Set Mask Value","Maskenwert festlegen"),ActionKind::View))
            .action_with(raster_internal_action("setFillTolerance", LocalizedLabel::native("Set Fill Tolerance", "Fülltoleranz festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setCompositeViewport", LocalizedLabel::native("Set Composite Viewport", "Komposit-Ansichtsfenster festlegen"), ActionKind::View))
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera") })
            .action_with(raster_internal_action("setCameraZoom", LocalizedLabel::native("Set Camera Zoom", "Kamerazoom festlegen"), ActionKind::View))
            // 📝️ Staged palette-form arguments for the two palette operations.
            .action_args("addLayer", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Layer Kind", "Ebenenart"), vec![
                    ActionArgOption::new("pixel", LocalizedLabel::native("Pixel", "Pixel")),
                    ActionArgOption::new("group", LocalizedLabel::native("Group", "Gruppe")),
                    ActionArgOption::new("adjustment", LocalizedLabel::native("Adjustment", "Anpassung")),
                ]).required().default_value(&"pixel"),
            ])
            // 💬️ Agent-facing descriptions (ticket 26/09/18 slice M5a) — EN first, DE second.
            .action_describe("addLayer", LocalizedLabel::native("Adds a new raster layer to the image — a pixel layer, a group, or an adjustment layer.", "Fügt dem Bild eine neue Rasterebene hinzu — Pixelebene, Gruppe oder Anpassungsebene."))
            .action_use_when("addLayer", vec!["add a pixel layer".into(), "add an adjustment layer".into(), "new layer in the image".into()])
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole image with one of the plugin's declared playground examples.", "Ersetzt das gesamte Bild durch eines der deklarierten Beispiele des Plugins."))
            .action_describe("setLayerVisible", LocalizedLabel::native("Shows or hides one raster layer explicitly.", "Blendet eine Rasterebene gezielt ein oder aus."))
            .action_describe("toggleLayerVisible", LocalizedLabel::native("Flips one raster layer between visible and hidden.", "Schaltet eine Rasterebene zwischen sichtbar und ausgeblendet um."))
            .action_describe("dropLayerKind", LocalizedLabel::native("Creates a raster layer of the given kind at a drop target in the layer tree.", "Erzeugt eine Rasterebene der angegebenen Art an einer Ablagestelle im Ebenenbaum."))
            .action_describe("deleteLayer", LocalizedLabel::native("Removes one raster layer from the image by id.", "Entfernt eine Rasterebene anhand ihrer Id aus dem Bild."))
            .action_describe("duplicateLayer", LocalizedLabel::native("Copies one raster layer and inserts the copy above the original.", "Kopiert eine Rasterebene und fügt die Kopie über dem Original ein."))
            .action_describe("patchLayer", LocalizedLabel::native("Sets one named property of one raster layer — its name, opacity, blend mode or visibility.", "Setzt eine benannte Eigenschaft einer Rasterebene — Name, Deckkraft, Mischmodus oder Sichtbarkeit."))
            .action_use_when("patchLayer", vec!["rename a layer".into(), "change the layer opacity".into(), "set the blend mode".into()])
            .action_describe("patchLayers", LocalizedLabel::native("Sets the same named property on several raster layers at once.", "Setzt dieselbe benannte Eigenschaft auf mehreren Rasterebenen gleichzeitig."))
            .action_describe("moveLayer", LocalizedLabel::native("Reorders one raster layer within the layer stack.", "Ordnet eine Rasterebene im Ebenenstapel um."))
            .action_describe("setBrushSize", LocalizedLabel::native("Sets the painting brush diameter for this session.", "Legt den Pinseldurchmesser für diese Sitzung fest."))
            .action_describe("setFillTolerance", LocalizedLabel::native("Sets how far a pixel's colour may differ from the clicked pixel for the bucket to fill it, for this session.", "Legt fest, wie weit die Farbe eines Pixels vom angeklickten Pixel abweichen darf, damit der Farbeimer ihn füllt, für diese Sitzung."))
            .action_describe("setBrushOpacity", LocalizedLabel::native("Sets the painting brush opacity for this session.", "Legt die Pinseldeckkraft für diese Sitzung fest."))
            // ⚠️ Discards content no later verb reconstructs — the gateway asks a human first.
            .action_destructive("deleteLayer")
            .action_destructive("setActiveExample")
            // 🧵️ Phase-8 dispositions. Every id declared above is `Migrated`: each one is backed by the
            // exact-owner `RasterRetainedCommandJobFactory` proof in `🧵️RetainedCommands`, so UI dispatch
            // (which rejects anything that is not `Migrated`) and the release-blocking
            // `validate_interactive_job_classification` gate both admit it. The kind is untouched — the six
            // `ActionKind::View` rows above stay View/session-only (they publish into the CONFIG lane, see
            // `RASTER_PUBLICATION_CONTRACTS`), exactly as the camera ticket 26/07/31 requires.
            //
            // 🧰️ `setActiveUtility` — the 16th `RasterCommand` row — is NOT listed here on purpose: it is
            // framework-injected by `.utility(..)` inside `build_definition`, after this builder chain has
            // run, and this owning declaration already classifies it `Migrated`.
            // Calling `.action_interactive_job("setActiveUtility", ..)` here would silently match nothing.
            .action_interactive_job("addLayer", InteractiveJobClassification::Migrated)
            .action_interactive_job("dropLayerKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLayerVisible", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleLayerVisible", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteLayer", InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateLayer", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchLayer", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchLayers", InteractiveJobClassification::Migrated)
            .action_interactive_job("moveLayer", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushSize", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushOpacity", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushColor", InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushHardness", InteractiveJobClassification::Migrated)
            .action_interactive_job("setPaintTarget",InteractiveJobClassification::Migrated)
            .action_interactive_job("setPixelSelection",InteractiveJobClassification::Migrated)
            .action_interactive_job("setMaskValue",InteractiveJobClassification::Migrated)
            .action_interactive_job("setFillTolerance", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCompositeViewport", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCameraZoom", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            // 🧰️ Composite-window utilities — one exclusive set, active utility host-owned (never a document operation).
            .utility(raster_utility("selectMarquee", LocalizedLabel::native("Marquee Select", "Rahmenauswahl"), "square-dashed", "Select", UtilityCategory::Selection))
            .utility(raster_utility("paintBrush", LocalizedLabel::native("Brush", "Pinsel"), "paintbrush", "Paint", UtilityCategory::Utilities))
            .utility(raster_utility("paintEraser", LocalizedLabel::native("Eraser", "Radiergummi"), "eraser", "Paint", UtilityCategory::Utilities))
            .utility(raster_utility("paintBucket", LocalizedLabel::native("Bucket", "Farbeimer"), "paint-bucket", "Paint", UtilityCategory::Utilities))
            .window_kind_utilities(composite::RASTER_PLAY_WINDOW_COMPOSITE, vec![
                "selectMarquee".into(), "paintBrush".into(), "paintEraser".into(), "paintBucket".into(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
#[cfg(test)]
#[path = "🧪️tests/🧪️fill-tool-transactions/🦀️.rs"]
mod fill_tool_transactions;
#[cfg(test)]
#[path = "🧪️tests/🧪️stroke-stream-transactions/🦀️.rs"]
mod stroke_stream_transactions;

//#endregion 🧪️Tests
