//! 🖥️ Raster editor — ArtifactEditor impl, render, manifest (constitutional: ui/general). B1: `RasterPlayApp`
//! is a unit struct — every former `RasterConfig` (`ui`-crate `RefCell`) field (selection, hover, brush
//! size/opacity, navigator composite-viewport size, the session-only free camera) now lives in
//! `crate::editor::raster::config::RasterConfig`, written via `RasterConfigMutation`s. Every action
//! dispatches through the single typed `RasterCommand` channel via `app_commands!` — mirrors
//! `shooting_ui`'s B1 pilot.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::editor::raster::modes::edit;
use crate::editor::raster::modes::edit::windows::{composite, navigator};
use crate::editor::raster::presence::{RasterPresence, RasterPresenceMutation};
use crate::editor::raster::terminology::raster_play_labels;
use crate::op::RasterMutation;
use crate::{RasterLayerNode, RasterSnapshot, RASTER_DOCUMENT_SCHEMA};
use dsl::os_pack::json::Value;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, JobFault, JobPayloadStream, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDescriptor, ActionFactory, ActionKind, AppDefinition, AppOperationContext, ArtifactEditor, ArtifactKindSpec, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactReservedJob,
    ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest, ArtifactToolCompletion, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect,
    DraftView, Editor, EditorApp, Emit, EphemeralEmit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType,
    MergeMode, NoDraft, NoDraftMutation, OsMediaCapability, SelectionMethod, SelectionMode, SelectionSpec, UtilityCategory, UtilityDefinition, WindowMeasure,
};
use std::collections::HashMap;
use store::ArtifactPack;
use store::EngineHandles;

//#region 🔖️Constants
pub const RASTER_PLAY_CONTROLLER_ID: &str = "raster-play";
/// 🌳️ Prefix for every layer-tree row id — shared by the document/masks panels and the `moveLayer`
/// command (which needs to decode a `target_row_id` back into a layer/group id). App-wide tree-encoding
/// concern, not artifact data, so it lives here rather than in any single panel.
pub const RASTER_TREE_PREFIX: &str = "raster-play-layers";
/// 🕹️ The single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM interaction domain this app declares
/// (granularity `layer`, `HierarchyProvider::Flat`, method Pick) — the layer tree binds it and the
/// composite window references it.
pub const RASTER_INTERACTION_DOMAIN: &str = "layers";
pub const RASTER_INTERACTION_GRANULARITY: &str = "layer";
//#endregion 🔖️Constants

//#region 🔖️Document
/// 🌳️ Encodes a layer as its tree-row id — shared by the document/masks panels (which render rows) and
/// `moveLayer` (which decodes a drop target back into an id). More than one consumer, but this is UI row
/// encoding, not artifact data, so it stays app-level rather than in `crate::schema`.
pub fn layer_row_id(layer: &RasterLayerNode) -> String {
    let segment = match layer {
        RasterLayerNode::Group { .. } => "group",
        RasterLayerNode::Adjustment { .. } => "adjustment",
        RasterLayerNode::Pixel { .. } => "layer",
    };
    format!("{RASTER_TREE_PREFIX}.{segment}.{}", crate::standards::v1::subsets::any::schema::layer_node_id(layer))
}

pub fn layer_id_from_tree_row_id(row_id: &str) -> Option<String> {
    row_id.strip_prefix(&format!("{RASTER_TREE_PREFIX}.")).and_then(|rest| rest.split('.').nth(1)).map(str::to_string)
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
    let mut fields = vec![("schema".to_string(), dsl::DslValue::String(document.schema.clone())), ("id".to_string(), dsl::DslValue::String(document.id.clone()))];
    if let Some(title) = &document.title {
        fields.push(("title".to_string(), dsl::DslValue::String(title.clone())));
    }
    fields.push(("layers".to_string(), dsl::DslValue::Array(document.layers.iter().map(dsl::ToValue::to_value).collect())));
    dsl::os_pack::json::from_dsl_value(&dsl::DslValue::Object(fields)).to_string()
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
    let object: dsl::os_pack::json::Object = resolved.into_iter().map(|(id, asset)| (id, dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(&asset)))).collect();
    dsl::os_pack::json::to_string(&Value::Object(object))
}

/// 🎞️ Builds the shared `Paint2dScene` payload for both the composite and navigator windows. Takes
/// `&RasterConfig` (an app-only view-state type), so per TEMPLATE.md §4's `DocumentHelpers` placement
/// rule this stays at app level even though it has two window consumers.
///
/// 🕹️ `selection_json`/`hovered_id` used to read `RasterConfig.selected_ids`/`.hovered_id` (deleted,
/// ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM)?; the `"layers"` domain's selection/
/// hover is framework-owned `InteractionState` now, and `ArtifactEditor::render` is not threaded an
/// `InteractionView` this wave (known SDK gap — see the ticket's `w3c-summary.md`) — left at neutral
/// defaults here. The real sync happens below `render`, at the paint surface/host layer
/// (`RasterHost::sync_interaction`, `🧰️framework/🔨️modules/🗺️surface/🎨️paint`), already migrated.
pub fn raster_scene(document: &RasterSnapshot, runtime: &RasterConfig, active_utility: &str, view_mode: &str) -> semio_framework_plugin::Paint2dScene {
    semio_framework_plugin::Paint2dScene {
        document_sync_json: document_sync_json(document),
        assets_json: assets_json_from_document(document),
        camera_json: dsl::os_pack::json::to_json_string(&runtime.camera),
        selection_json: "[]".into(),
        hovered_id: None,
        active_utility: active_utility.into(),
        brush_size: runtime.brush_size,
        brush_opacity: runtime.brush_opacity,
        view_mode: view_mode.into(),
        composite_viewport_json: runtime.composite_viewport.as_ref().map(dsl::os_pack::json::to_json_string),
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
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    semio_framework_plugin::plugin_app_close_prelude::Label::try_from(value.as_ref().to_string()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "raster UI label admission failed"))
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

    fn put(entries: &mut Vec<(String, dsl::DslValue)>, key: &str, value: dsl::DslValue) {
        entries.retain(|(existing, _)| existing != key);
        entries.push((key.to_string(), value));
    }

    /// 🔢️ The host's JSON round trip turns every integer into `Number::Float`; the exact-integer
    /// codecs refuse `Float(1.0)`, so whole finite floats are restored to `UInt`/`Int` (every `f64`
    /// field accepts any `Number` variant, so nothing else changes).
    fn integral(value: dsl::DslValue) -> dsl::DslValue {
        match value {
            dsl::DslValue::Number(dsl::Number::Float(float)) if float.is_finite() && float.fract() == 0.0 && float.abs() < 9.007_199_254_740_992e15 => {
                if float >= 0.0 { dsl::DslValue::Number(dsl::Number::UInt(float as u64)) } else { dsl::DslValue::Number(dsl::Number::Int(float as i64)) }
            }
            dsl::DslValue::Array(items) => dsl::DslValue::Array(items.into_iter().map(integral).collect()),
            dsl::DslValue::Object(entries) => dsl::DslValue::Object(entries.into_iter().map(|(key, value)| (key, integral(value))).collect()),
            other => other,
        }
    }

    /// 🔁️ Emits every key of `args` under BOTH spellings (`FromValue` ignores keys it does not know)
    /// after applying `aliases` (snake_case source → destination).
    fn fold(args: Option<&dsl::DslValue>, aliases: &[(&str, &str)]) -> dsl::DslValue {
        let mut entries: Vec<(String, dsl::DslValue)> = Vec::new();
        if let Some(dsl::DslValue::Object(object)) = args {
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
        dsl::DslValue::Object(entries)
    }

    fn decode<T: dsl::FromValue>(action: &str, value: dsl::DslValue) -> Result<T, Fault> {
        T::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("raster action '{action}' arguments do not decode: {error}")))
    }

    pub fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<RasterCommand, Fault> {
        // 🧱️ Tree rows and context menus address a layer by its bare `id`; the payloads spell `layer_id`.
        const LAYER: &[(&str, &str)] = &[("id", "layer_id")];
        let plain = || fold(args, &[]);
        let layer = || fold(args, LAYER);
        Ok(match action {
            "addLayer" => RasterCommand::AddLayer(decode(action, plain())?),
            "dropLayerKind" => RasterCommand::DropLayerKind(decode(action, plain())?),
            "setLayerVisible" => RasterCommand::SetLayerVisible(decode(action, layer())?),
            "toggleLayerVisible" => RasterCommand::ToggleLayerVisible(decode(action, layer())?),
            "deleteLayer" => RasterCommand::DeleteLayer(decode(action, layer())?),
            "duplicateLayer" => RasterCommand::DuplicateLayer(decode(action, layer())?),
            "patchLayer" => RasterCommand::PatchLayer(decode(action, layer())?),
            "patchLayers" => RasterCommand::PatchLayers(decode(action, fold(args, &[("ids", "layer_ids")]))?),
            "moveLayer" => RasterCommand::MoveLayer(decode(action, fold(args, &[("id", "layer_id"), ("target_id", "target_row_id"), ("position", "drop_position")]))?),
            "setBrushSize" => RasterCommand::SetBrushSize(decode(action, fold(args, &[("size", "value")]))?),
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
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::raster::commands::set_active_example;
use crate::editor::raster::commands::{add_layer, delete_layer, drop_layer_kind, duplicate_layer, move_layer, patch_layer, patch_layers, set_layer_visible, toggle_layer_visible};
use crate::editor::raster::commands::{set_brush_opacity, set_brush_size};
use crate::editor::raster::commands::{set_camera, set_camera_zoom, set_composite_viewport};
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
/// 🧵️ Every `RasterCommand` row, without exception — the retained route table, the manifest's
/// `Migrated` classification list and `RasterCommand::TOOL_JOB_IDS` are the SAME fifteen ids, which
/// is exactly what the framework's `validate_tool_job_rows` demands (`expected = TOOL_JOB_IDS ∩
/// migrated` must equal the proof set). Row order mirrors the `app_commands!` declaration order above.
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
];
const RASTER_RETAINED_PAYLOAD_SCHEMA: &str = "raster.tool-command.v1";
const RASTER_RETAINED_RAW_BYTES: usize = 65_536;
const RASTER_RETAINED_WORK_ITEMS: usize = 4_096;
/// 🛣️ Publication lanes per route, read off each handler's own `Emit` in `🎮️commands/*/🦀️.rs` — the
/// ten document verbs build `Emit { artifact_mutations, .. }`/`Emit::mutations(..)` over
/// `RasterMutation` and never touch the config, while the five session verbs build `Emit::config(..)`
/// over `RasterConfigMutation` and never touch the document. No raster handler emits both lanes, a
/// draft, a presence or a transient mutation, so no route declares more than one lane here.
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

//#region 📬️StorePreparation
/// 📬️ The document lane's one-item retained preparation. Without it every route declaring
/// `ArtifactToolPublicationLane::Artifact` is registered with an unsupported publication contract and
/// stays dispatch-dead, no matter how it is classified.
struct RasterStorePreparationFactory;

struct RasterStorePreparation {
    base: Option<store::SnapshotRead<RasterSnapshot>>,
    mutation: Option<RasterMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<RasterSnapshot, RasterMutation>>,
    /// 🧮 The clone-free stepwise apply (`RasterOneItemApply`), live from the first `advance` until
    /// the post snapshot is handed over, or closed through its own retirement on cancel/fault.
    apply: Option<crate::spr::RasterOneItemApply>,
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
    fn preflight(&self, _mutation: &RasterMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Raster Store preparation rejected its lane or description envelope".into());
        }
        // 🧮 `work_items` counts the forward AND its inverse (every raster inverse is exactly one
        // operation: `create-layer` ↔ `delete-layer` of the whole subtree, `reorder` ↔ `reorder`, …) —
        // the batch fold sizes its inverse capacity as `Σ work_items − admitted_items`, so declaring
        // `1` exhausted it on the very first fold ("batched fold exceeded its admitted fixed inverse
        // capacity") and the two-layer demo never landed (process3d declares the same `2`).
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
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
            description: request.description,
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
            let apply = self.apply.get_or_insert_with(crate::spr::RasterOneItemApply::new);
            match apply.advance(base.get(), mutation, authority.operation(), authority.generation(), RASTER_ONE_ITEM_APPLY_FUEL)? {
                Some(post) => post,
                None => return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint)),
            }
        };
        drop(self.apply.take());
        let mutation = self.mutation.take().ok_or_else(|| "Raster preparation lost its mutation owner".to_string())?;
        let inverse = mutation.inverse(base.get());
        let id = format!("raster-retained-{}", authority.next_sequence_number());
        let edit = protocol::Edit {
            id: id.clone(),
            actor: Some(authority.actor().to_string()),
            forwards: vec![mutation],
            inverse,
            mutation_meta: vec![protocol::MutationMeta {
                mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
                dependencies: Vec::new(),
                base_version: authority.base_applied_edit_count() as u64,
                author_id: Some(protocol::ActorId(authority.actor().to_string())),
                timestamp: authority.next_clock(),
                undo_policy: protocol::UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: None,
                origin: Default::default(),
            }],
            description: self.description.take(),
            coalesce_key: None,
            sequence_number: authority.next_sequence_number(),
            started_at: String::new(),
            finished_at: None,
        };
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

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
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
            self.mutation_retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&crate::spr::RasterMutationRetirementFactory, mutation));
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
        if self.prepared.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Raster preparation could not return its exact base root".into());
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
        self.closing && self.base.is_none() && self.mutation.is_none() && self.apply.is_none() && self.mutation_retirement.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}

/// 📬️ The config lane's twin of {@link RasterStorePreparationFactory} — raster's seven session verbs
/// (`setBrushSize`/`setBrushOpacity`/`setCompositeViewport`/`setCamera`/`setCameraZoom`/
/// publication contract outright when this factory is absent. `RasterConfig` is a whole-record config
/// (`store::impl_whole_record_config!`), so its `Diff` is the config value itself.
struct RasterConfigStorePreparationFactory;

struct RasterConfigStorePreparation {
    base: Option<store::SnapshotRead<RasterConfig>>,
    mutation: Option<RasterConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<RasterConfig, RasterConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<RasterConfig, RasterConfigMutation> for RasterConfigStorePreparationFactory {
    fn preflight(&self, _mutation: &RasterConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Raster config preparation rejected its lane or description envelope".into());
        }
        // 🧮 Forward + its one inverse (a whole-record config swap), for the same fold-capacity reason
        // the document lane's `preflight` records — at `1` every `setCompositeViewport`/`setCamera`
        // was refused "batched item candidate failed its exact fixed fold contract" (react boot
        // `raster-boot-5`, 2026-09-16).
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
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
            description: request.description,
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
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Raster config preparation lost its Store authority".to_string())?;
        let id = format!("raster-config-retained-{}", authority.next_sequence_number());
        let edit = protocol::Edit {
            id: id.clone(),
            actor: Some(authority.actor().to_string()),
            forwards: vec![mutation],
            inverse,
            mutation_meta: vec![protocol::MutationMeta {
                mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
                dependencies: Vec::new(),
                base_version: authority.base_applied_edit_count() as u64,
                author_id: Some(protocol::ActorId(authority.actor().to_string())),
                timestamp: authority.next_clock(),
                undo_policy: protocol::UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: None,
                origin: Default::default(),
            }],
            description: self.description.take(),
            coalesce_key: None,
            sequence_number: authority.next_sequence_number(),
            started_at: String::new(),
            finished_at: None,
        };
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

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Raster config preparation could not return its exact base root".into());
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
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
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
/// completion authority. The decode itself is `crate::io::raster_image_layer_and_asset` — the SAME
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
            MediaPayload::Binary { .. } => None,
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
        let (asset_id, asset, layer) = crate::io::raster_image_layer_and_asset(media_json);
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
            "setBrushSize", "setBrushOpacity", "setCompositeViewport", "setCamera", "setCameraZoom", "setActiveExample"
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(RasterRetainedCommandJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !RASTER_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id || raster_retained_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
            return Err(Fault::from("raster-retained-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(BoundedArtifactCommandWork::new(tool_id, raster_retained_reduce, raster_retained_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: None,
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
        Some(crate::spr::raster_envelope_decode_owner_bundle())
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::spr::raster_document_store_owners())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::spr::raster_document_store_initialization_job(envelope, operation, generation))
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

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
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
        crate::standards::v1::subsets::any::schema::empty_raster_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(raster_io())
    }

    /// 🎞️ `image:in`/`image:out` (see `crate::io::raster_image_layer_and_asset`,
    /// `raster_composite_media`) plus the inherited `document:out` default (the pack of
    /// `doc.snapshot`, replicated inline — overriding `export_media` shadows the trait's provided
    /// body for every port on this app, not just the new ones).
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
    /// anymore). Falls through to the inherited `document:in` default (`MediaError::NotImplemented`,
    /// since `whole_document_operation` is no longer overridden) for any other port.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, RasterSnapshot>) -> Result<Emit<RasterMutation, RasterConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "image:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json: png_base64, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "image:in only accepts a Structured (base64 PNG) payload".into()));
        };
        let (asset_id, asset, layer) = crate::io::raster_image_layer_and_asset(png_base64);
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
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
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

    fn window_measures(_doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>, _view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(composite::RASTER_PLAY_WINDOW_COMPOSITE.into(), composite::window_measures(cfg.snapshot))])
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let active_utility = view_state.active_utility_id.as_deref().unwrap_or("selectMarquee");
        let labels = raster_play_labels(view_state);
        // 🪟️ One `TreeWindows` per panel body: the host's open/scroll state for exactly the containers
        // that body owns, plus the shared first-paint budget the panel spends in document order.
        let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
        let node = match body_key {
            composite::RASTER_PLAY_BODY_COMPOSITE => composite::render(document, config, active_utility)?,
            navigator::RASTER_PLAY_BODY_NAVIGATOR => navigator::render(document, config, active_utility)?,
            crate::editor::raster::panels::document::RASTER_PLAY_BODY_LAYERS => crate::editor::raster::panels::document::render(document, config, labels, &windows)?,
            crate::editor::raster::panels::masks::RASTER_PLAY_BODY_MASKS => crate::editor::raster::panels::masks::render(document, config, labels, &windows)?,
            crate::editor::raster::panels::catalogue::RASTER_PLAY_BODY_CATALOGUE => crate::editor::raster::panels::catalogue::render(labels, &windows)?,
            crate::editor::raster::panels::inspection::RASTER_PLAY_BODY_PROPERTIES => crate::editor::raster::panels::inspection::render(document, config, labels)?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "raster unknown-body label admission failed"))?,
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
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

/// 🖼️ Composites the current raster document to a PNG `Media` payload for the `image:out` port —
/// `crate::io::raster_document_json_to_svg` renders the document's real layer
/// stack (not a placeholder title card) via the `s.stdio.semio/v1/drawing` bridge; the vector→pixels
/// render step still has no stdio bridge (real pixel compositing is wgpu/canvas-host-side, out of
/// this pure headless compute node's reach — see that function's own doc), so its raw renderer
/// output is canonicalized through the real `s.stdio.semio/v1/image` ↔ png round trip inside
/// `🚪️io/🦀️.rs` before leaving this port.
pub fn raster_composite_media(document: &RasterSnapshot) -> Result<Media, MediaError> {
    let (svg, width, height) = crate::io::raster_document_json_to_svg(document).map_err(|error| MediaError::Payload("image:out".into(), error))?;
    let rendered = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height).map_err(|error| MediaError::Payload("image:out".into(), error))?;
    let raw_bytes = base64_codec::base64_standard_decode(rendered.as_bytes()).map_err(|error| MediaError::Payload("image:out".into(), error.to_string()))?;
    let canonical = crate::io::canonicalize_png_bytes(&raw_bytes).map_err(|error| MediaError::Payload("image:out".into(), error))?;
    let png_base64 = base64_codec::base64_standard_encode(canonical);
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
/// `paintEraser` erases, `selectMarquee` selects) because the scene's active utility feeds `RasterHost`.
fn raster_utility(id: &str, label: impl Into<LocalizedLabel>, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding/utility declarations (which have no dedicated `_def` passthrough) are
/// written out inline.
///
/// 📚️ CLOSED (ticket 26/09/05/RASTER-PLUGIN-END-TO-END, W2) — the old "SDK GAP #4" note here claimed
/// `.editor::<E>(def: AppDefinition)` discards `App.examples` with no replacement, so raster's
/// `📚️examples/🎬️demo` facet went unregistered. The replacement is `PluginBuilder::
/// editor_with_examples`, already in production on `🌀️procedural`'s two editors: the plugin root's
/// `examples()` now stamps the demo carrier onto `PluginManifest.examples`, which is what the react
/// shell's `NavbarExampleSelect` reads. Examples are a PLUGIN-root registration, not a builder-chain
/// one; nothing about them belongs in this function.
pub fn create_raster_app() -> AppDefinition {
    Editor::builder(crate::RASTER_DIALECT).document(["semio", "raster"])
            .artifact_kind(crate::artifact_kind())
            // 🖼️ `2d.image` — the interchange kind `image:out` produces (WORKFLOWS-END-TO-END-TYPED-PORTS
            // Wave 2 port recipe); `shooting`'s `photos:out` already declares the identical shape — a
            // harmless duplicate registration (registry dedupes by id).
            .artifact_kind(ArtifactKindSpec {
                id: "2d.image".into(),
                name: "2D Image".into(),
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
            .mutation("addLayer", LocalizedLabel::native("Add Layer", "Ebene hinzufügen"))
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
                hierarchy: HierarchyProvider::Flat,
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
            .action_interactive_job("setCompositeViewport", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCameraZoom", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            // 🧰️ Composite-window utilities — one exclusive set, active utility host-owned (never a document operation).
            .utility(raster_utility("selectMarquee", LocalizedLabel::native("Marquee Select", "Rahmenauswahl"), "square-dashed", "Select", UtilityCategory::Selection))
            .utility(raster_utility("paintBrush", LocalizedLabel::native("Brush", "Pinsel"), "paintbrush", "Paint", UtilityCategory::Utilities))
            .utility(raster_utility("paintEraser", LocalizedLabel::native("Eraser", "Radiergummi"), "eraser", "Paint", UtilityCategory::Utilities))
            .window_kind_utilities(composite::RASTER_PLAY_WINDOW_COMPOSITE, vec![
                "selectMarquee".into(), "paintBrush".into(), "paintEraser".into(),
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

//#endregion 🧪️Tests
