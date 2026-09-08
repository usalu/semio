//! 🖥️ Raster editor — ArtifactEditor impl, render, manifest (constitutional: ui/general). B1: `RasterPlayApp`
//! is a unit struct — every former `RasterConfig` (`ui`-crate `RefCell`) field (selection, hover, brush
//! size/opacity, navigator composite-viewport size, the session-only free camera) now lives in
//! `crate::editor::raster::config::RasterConfig`, written via `RasterConfigMutation`s. Every action
//! dispatches through the single typed `RasterCommand` channel via `app_commands!` — mirrors
//! `shooting_ui`'s B1 pilot.

use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot, RASTER_DOCUMENT_SCHEMA};
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::editor::raster::modes::edit;
use crate::editor::raster::modes::edit::windows::{composite, navigator};
use crate::editor::raster::presence::{RasterPresence, RasterPresenceMutation};
use crate::editor::raster::terminology::raster_play_labels;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDescriptor, ActionFactory, ActionKind, AppDefinition, AppOperationContext, ArtifactEditor, ArtifactKindSpec, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry,
    ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label,
    LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode, NoDraft, NoDraftMutation, OsMediaCapability, SelectionMethod, SelectionMode, SelectionSpec, UtilityCategory, UtilityDefinition, WindowMeasure,
};
use dsl::os_pack::json::Value;
use std::collections::HashMap;
use store::ArtifactPack;
use store::EngineHandles;

//#region 🔖️Constants
pub const RASTER_PLAY_CONTROLLER_ID: &str = "raster-play";
/// 🌳️ Prefix for every layer-tree row id — shared by the document/masks panels and the `moveLayer`
/// command (which needs to decode a `target_row_id` back into a layer/group id). App-wide tree-encoding
/// concern, not artifact data, so it lives here rather than in any single panel.
pub const RASTER_TREE_PREFIX: &str = "raster-play-layers";
//#endregion 🔖️Constants

//#region 🔖️Document
/// 🌳️ Encodes a layer as its tree-row id — shared by the document/masks panels (which render rows) and
/// `moveLayer` (which decodes a drop target back into an id). More than one consumer, but this is UI row
/// encoding, not artifact data, so it stays app-level rather than in `crate::artifacts::raster::schema`.
pub fn layer_row_id(layer: &RasterLayerNode) -> String {
    let segment = match layer {
        RasterLayerNode::Group { .. } => "group",
        RasterLayerNode::Adjustment { .. } => "adjustment",
        RasterLayerNode::Pixel { .. } => "layer",
    };
    format!("{RASTER_TREE_PREFIX}.{segment}.{}", crate::artifacts::raster::schema::layer_node_id(layer))
}

pub fn layer_id_from_tree_row_id(row_id: &str) -> Option<String> {
    row_id.strip_prefix(&format!("{RASTER_TREE_PREFIX}.")).and_then(|rest| rest.split('.').nth(1)).map(str::to_string)
}

pub fn mask_row_id(target_id: &str) -> String {
    format!("{RASTER_TREE_PREFIX}.mask.{target_id}")
}

/// 📡️ Document JSON for the WASM compositor, omitting embedded assets/utility/brush — mirrors
/// premigration `rasterDocumentToSyncJson`. Takes `&RasterConfig` nowhere directly (assets live on the
/// document), but stays app-level next to {@link raster_scene}, its only caller.
fn document_sync_json(document: &RasterSnapshot) -> String {
    let value = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(document));
    let value = match value {
        Value::Object(object) => {
            let filtered: dsl::os_pack::json::Object =
                object.iter().filter(|(key, _)| *key != "assets" && *key != "brushSize" && *key != "brushOpacity").map(|(key, value)| (key.to_string(), value.clone())).collect();
            Value::Object(filtered)
        }
        other => other,
    };
    value.to_string()
}

/// 🧩️ Resolves every asset handle on `document.assets` back to its real `RasterImageAsset` bytes
/// through the working-scene cache accessor (`crate::artifacts::raster::raster_asset`, ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` — `document.assets` now stores composed
/// `s.stdio.semio.image` CHILD handles, not embedded bytes) — the ONE call site the WASM compositor's
/// real pixel bytes funnel through. A handle whose content is not (or no longer) cached is honestly
/// omitted rather than serialized as an empty/garbage blob (documented staleness gap, matches every
/// other exemplar in this ticket).
fn assets_json_from_document(document: &RasterSnapshot) -> String {
    let resolved: std::collections::BTreeMap<String, crate::artifacts::raster::RasterImageAsset> =
        document.assets.keys().filter_map(|asset_id| crate::artifacts::raster::raster_asset(&document.assets, asset_id).map(|asset| (asset_id.clone(), asset))).collect();
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
        composite_viewport_json: runtime.composite_viewport.as_ref().map(|viewport| dsl::os_pack::json::to_json_string(viewport)),
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
    semio_framework_plugin::plugin_app_close_prelude::Label::try_from(value.as_ref().to_string())
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "raster UI label admission failed"))
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref())
        .map(semio_framework_plugin::UiValue::Text)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
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
    let mut builder = semio_framework_plugin::UiListBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder
            .push(value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder
            .push(key.to_owned(), value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes
            .try_push(node)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
}

//#endregion 🔖️Document

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
        "setActiveUtility" as "active-utility" => set_active_utility::SetActiveUtility,
        "setLocale" as "locale" => set_locale::SetLocale,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::raster::commands::set_active_example;
use crate::editor::raster::commands::set_active_utility;
use crate::editor::raster::commands::set_locale;
use crate::editor::raster::commands::{add_layer, delete_layer, drop_layer_kind, duplicate_layer, move_layer, patch_layer, patch_layers, set_layer_visible, toggle_layer_visible};
use crate::editor::raster::commands::{set_brush_opacity, set_brush_size};
use crate::editor::raster::commands::{set_camera, set_camera_zoom, set_composite_viewport};
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
/// 🧵️ Every `RasterCommand` row, without exception — the retained route table, the manifest's
/// `Migrated` classification list and `RasterCommand::TOOL_JOB_IDS` are the SAME seventeen ids, which
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
    "setActiveUtility",
    "setLocale",
    "setActiveExample",
];
const RASTER_RETAINED_PAYLOAD_SCHEMA: &str = "raster.tool-command.v1";
const RASTER_RETAINED_RAW_BYTES: usize = 65_536;
const RASTER_RETAINED_WORK_ITEMS: usize = 4_096;
/// 🛣️ Publication lanes per route, read off each handler's own `Emit` in `🎮️commands/*/🦀️.rs` — the
/// ten document verbs build `Emit { artifact_mutations, .. }`/`Emit::mutations(..)` over
/// `RasterMutation` and never touch the config, while the seven session verbs build `Emit::config(..)`
/// over `RasterConfigMutation` and never touch the document. No raster handler emits both lanes, a
/// draft, a presence or a transient mutation, so no route declares more than one lane here.
///
/// 🎬️ `setActiveExample` is a document verb, not a session one: raster has no whole-document replace
/// mutation, so `🎮️commands/🎬️set-active-example` spells loading an example as an ordered batch of
/// real `RasterMutation`s (delete every root layer, re-point the asset pool, plant the example forest)
/// — the Artifact lane, exactly like every other layer verb.
///
/// 🎥️ `setCamera`/`setCameraZoom`/`setCompositeViewport`/`setActiveUtility`/`setLocale` stay session-only
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
    ArtifactToolPublicationContract { tool_id: "setActiveUtility", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[ArtifactToolPublicationLane::Config] },
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
    let items = crate::artifacts::raster::schema::flatten_raster_layers(&snapshot.layers).len().checked_add(snapshot.assets.len())?.checked_add(1)?;
    (items <= RASTER_RETAINED_WORK_ITEMS).then_some(1)
}

fn raster_retained_reduce(
    command: &RasterCommand,
    snapshot: &RasterSnapshot,
    config: &RasterConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<RasterMutation, RasterConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config })
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
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<RasterSnapshot, RasterMutation> for RasterStorePreparationFactory {
    fn preflight(&self, _mutation: &RasterMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Raster Store preparation rejected its lane or description envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<RasterSnapshot, RasterMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<RasterSnapshot, RasterMutation>>, store::ArtifactStoreOneItemPreparationRequest<RasterSnapshot, RasterMutation>> {
        let item_count = crate::artifacts::raster::schema::flatten_raster_layers(&request.base.get().layers).len().saturating_add(request.base.get().assets.len());
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
        let mutation = self.mutation.take().ok_or_else(|| "Raster preparation lost its mutation owner".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Raster preparation lost its Store authority".to_string())?;
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
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
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
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}

/// 📬️ The config lane's twin of {@link RasterStorePreparationFactory} — raster's seven session verbs
/// (`setBrushSize`/`setBrushOpacity`/`setCompositeViewport`/`setCamera`/`setCameraZoom`/
/// `setActiveUtility`/`setLocale`) publish into the config store, and the runtime rejects a `Config`
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
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
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

    const DIALECT: Dialect = crate::artifacts::raster::RASTER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = RASTER_DOCUMENT_SCHEMA;

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
        document_schema: "raster.document",
        factory: "RasterRetainedCommandJobFactory",
        factory_type: RasterRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "addLayer", "dropLayerKind", "setLayerVisible", "toggleLayerVisible", "deleteLayer", "duplicateLayer", "patchLayer", "patchLayers", "moveLayer",
            "setBrushSize", "setBrushOpacity", "setCompositeViewport", "setCamera", "setCameraZoom", "setActiveUtility", "setLocale",
            "setActiveExample"
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
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            operation_context,
            request.completion,
            RasterCommand::command_id,
            RASTER_RETAINED_RAW_BYTES,
            RASTER_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::raster::spr::raster_envelope_decode_owner_bundle())
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::raster::spr::raster_document_store_owners())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::artifacts::raster::spr::raster_document_store_initialization_job(envelope, operation, generation))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::raster::config::schema::app_schema_descriptor())
    }

    /// 📄️ Boots on the bundled `📚️examples/🎬️demo` Semio-logo carrier (the same `.dsl.semio` text
    /// `setActiveExample` loads), so every window renders real content instead of the all-`Default`
    /// scaffold. `empty_raster_document()` stays the tests' blank slate — mirrors block2d's
    /// `default_block2d_snapshot`.
    fn initial_snapshot() -> RasterSnapshot {
        crate::artifacts::raster::schema::default_raster_document()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(raster_io())
    }

    /// 🎞️ `image:in`/`image:out` (see `crate::artifacts::raster::io::raster_image_layer_and_asset`,
    /// `raster_composite_media`) plus the inherited `document:out` default (the pack of
    /// `doc.snapshot`, replicated inline — overriding `export_media` shadows the trait's provided
    /// body for every port on this app, not just the new ones).
    fn export_media(port: &str, doc: &ArtifactView<'_, RasterSnapshot>) -> Result<Media, MediaError> {
        match port {
            "image:out" => raster_composite_media(doc.snapshot),
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
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
        let (asset_id, asset, layer) = crate::artifacts::raster::io::raster_image_layer_and_asset(png_base64);
        Ok(Emit::mutations(vec![
            RasterMutation::AddLayerAsset(crate::artifacts::raster::mutations::add_layer_asset::mutation::AddLayerAsset { asset_id, asset }),
            RasterMutation::CreateLayer(crate::artifacts::raster::mutations::create_layer::mutation::CreateLayer { parent_id: None, index: doc.snapshot.layers.len(), layer: Box::new(layer) }),
        ]))
    }

    fn command_id(command: &RasterCommand) -> &'static str {
        command.command_id()
    }

    fn handle(
        command: &RasterCommand,
        doc: &ArtifactView<'_, RasterSnapshot>,
        cfg: &ConfigView<'_, RasterConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<RasterMutation, RasterConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn window_measures(_doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(composite::RASTER_PLAY_WINDOW_COMPOSITE.into(), composite::window_measures(cfg.snapshot))])
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let labels = raster_play_labels(config);
        let node = match body_key {
            composite::RASTER_PLAY_BODY_COMPOSITE => composite::render(document, config)?,
            navigator::RASTER_PLAY_BODY_NAVIGATOR => navigator::render(document, config)?,
            crate::editor::raster::panels::document::RASTER_PLAY_BODY_LAYERS => crate::editor::raster::panels::document::render(document, config, labels)?,
            crate::editor::raster::panels::masks::RASTER_PLAY_BODY_MASKS => crate::editor::raster::panels::masks::render(document, config, labels)?,
            crate::editor::raster::panels::catalogue::RASTER_PLAY_BODY_CATALOGUE => crate::editor::raster::panels::catalogue::render(labels)?,
            crate::editor::raster::panels::inspection::RASTER_PLAY_BODY_PROPERTIES => crate::editor::raster::panels::inspection::render(document, config, labels)?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}")))
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "raster unknown-body label admission failed"))?,
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}
//#endregion 🔖️RasterPlayApp

//#region 🔖️Io
/// 🔌️ Relocated verbatim from `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
/// rule 4: anything returning `AppIo` or referencing an app type lives in `🎛️apps/<app>/`). This app's
/// typed media I/O surface (`AppDefinition.io`) — mirrors the `2d.raster` `ArtifactKindSpec` literal
/// `crate::artifacts::raster::artifact_kind` already declares, plus the app-specific `image:in`/
/// `image:out` ports (see below).
pub fn raster_io() -> semio_framework::AppIo {
    semio_framework::AppIo {
        document_schema: RASTER_DOCUMENT_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
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
/// `crate::artifacts::raster::io::raster_document_json_to_svg` renders the document's real layer
/// stack (not a placeholder title card) via the `s.stdio.semio/v1/drawing` bridge; the vector→pixels
/// render step still has no stdio bridge (real pixel compositing is wgpu/canvas-host-side, out of
/// this pure headless compute node's reach — see that function's own doc), so its raw renderer
/// output is canonicalized through the real `s.stdio.semio/v1/image` ↔ png round trip inside
/// `🚪️io/🦀️.rs` before leaving this port.
pub fn raster_composite_media(document: &RasterSnapshot) -> Result<Media, MediaError> {
    let (svg, width, height) = crate::artifacts::raster::io::raster_document_json_to_svg(document).map_err(|error| MediaError::Payload("image:out".into(), error))?;
    let rendered = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height).map_err(|error| MediaError::Payload("image:out".into(), error))?;
    let raw_bytes = base64_codec::base64_standard_decode(rendered.as_bytes()).map_err(|error| MediaError::Payload("image:out".into(), error.to_string()))?;
    let canonical = crate::artifacts::raster::io::canonicalize_png_bytes(&raw_bytes).map_err(|error| MediaError::Payload("image:out".into(), error))?;
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
    Editor::builder(crate::artifacts::raster::RASTER_DIALECT).document(["semio", "raster"])
            .artifact_kind(crate::artifacts::raster::artifact_kind())
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
                    export_stdio_kinds: vec!["stdio.png"],
        import_stdio_kinds: vec!["stdio.png"],
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
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            // 🔧️ Internal content operations — layer-tree / catalogue-drop / inspector bound.
            .action_with(raster_internal_action("setLayerVisible", LocalizedLabel::native("Set Layer Visible", "Ebenensichtbarkeit festlegen"), ActionKind::Mutation))
            .action_with(raster_internal_action("toggleLayerVisible", LocalizedLabel::native("Toggle Layer Visible", "Ebenensichtbarkeit umschalten"), ActionKind::Mutation))
            .action_with(raster_internal_action("dropLayerKind", LocalizedLabel::native("Drop Layer Kind", "Ebenenart ablegen"), ActionKind::Mutation))
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
                id: "layers".into(),
                label: LocalizedLabel::native("Layers", "Ebenen"),
                granularities: vec![GranularityDefinition { id: "layer".into(), label: LocalizedLabel::native("Layer", "Ebene"), icon_id: "image".into() }],
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
            .window_kind_interactions(composite::RASTER_PLAY_WINDOW_COMPOSITE, vec![InteractionRef::new("layers")])
            // 👁️ Ephemeral view state — live brush controls, navigator viewport, camera.
            .action_with(raster_internal_action("setBrushSize", LocalizedLabel::native("Set Brush Size", "Pinselgröße festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setBrushOpacity", LocalizedLabel::native("Set Brush Opacity", "Pinseldeckkraft festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setCompositeViewport", LocalizedLabel::native("Set Composite Viewport", "Komposit-Ansichtsfenster festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setCameraZoom", LocalizedLabel::native("Set Camera Zoom", "Kamerazoom festlegen"), ActionKind::View))
            .action_with(raster_internal_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"), ActionKind::View))
            // 📝️ Staged palette-form arguments for the two palette operations.
            .action_args("addLayer", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Layer Kind", "Ebenenart"), vec![
                    ActionArgOption::new("pixel", LocalizedLabel::native("Pixel", "Pixel")),
                    ActionArgOption::new("group", LocalizedLabel::native("Group", "Gruppe")),
                    ActionArgOption::new("adjustment", LocalizedLabel::native("Adjustment", "Anpassung")),
                ]).required().default_value("pixel"),
            ])
            // 🧵️ Phase-8 dispositions. Every id declared above is `Migrated`: each one is backed by the
            // exact-owner `RasterRetainedCommandJobFactory` proof in `🧵️RetainedCommands`, so UI dispatch
            // (which rejects anything that is not `Migrated`) and the release-blocking
            // `validate_interactive_job_classification` gate both admit it. The kind is untouched — the six
            // `ActionKind::View` rows above stay View/session-only (they publish into the CONFIG lane, see
            // `RASTER_PUBLICATION_CONTRACTS`), exactly as the camera ticket 26/07/31 requires.
            //
            // 🧰️ `setActiveUtility` — the 16th `RasterCommand` row — is NOT listed here on purpose: it is
            // framework-injected by `.utility(..)` inside `build_definition`, after this builder chain has
            // run, and `ActionDefinition::resumable_framework_catalog` already classifies it `Migrated`.
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
            .action_interactive_job("setLocale", InteractiveJobClassification::Migrated)
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
pub(crate) mod testkit {
    //! 🧪️ Shared harness for every `editor::raster` node's tests — mirrors TEMPLATE.md §7.
    use super::*;
    use semio_framework_plugin::{testkit as framework_testkit, App, InvocationResult, VcsArtifactApp, ViewModel};

    pub type RasterApp = VcsArtifactApp<EditorApp<RasterPlayApp>>;

    use semio_framework_plugin::PluginApp;

    /// 🚧️ SDK GAP (`📓️w2-cad-report.md` "SDK gaps found" #3): `testkit::new_app_with_registry`'s
    /// signature is still `fn(manifest: fn() -> App)`, unchanged for this ticket; `create_raster_app`
    /// now returns `AppDefinition`. This tiny local wrapper adapts one to the other.
    fn raster_app_manifest_for_testkit() -> App {
        App { definition: create_raster_app(), examples: Vec::new() }
    }

    pub async fn app() -> RasterApp {
        framework_testkit::new_app::<EditorApp<RasterPlayApp>>().await
    }

    pub async fn app_with_registry() -> RasterApp {
        framework_testkit::new_app_with_registry::<EditorApp<RasterPlayApp>>(raster_app_manifest_for_testkit).await
    }

    pub async fn dispatch(app: &mut RasterApp, command: RasterCommand) -> InvocationResult {
        app.dispatch_typed(command, &framework_testkit::meta("local")).await.expect("dispatch")
    }

    pub async fn render(app: &mut RasterApp, body_key: &str) -> String {
        let tree = app.render(body_key, None, &ViewModel::default()).await.expect("render");
        framework_testkit::project_and_retire_fixture_tree(tree).expect("rendered fixture observation and retirement")
    }

    pub async fn main_window_measures(app: &mut RasterApp) -> Vec<WindowMeasure> {
        app.window_measures().await.remove(composite::RASTER_PLAY_WINDOW_COMPOSITE).unwrap_or_default()
    }

    pub async fn semio_app() -> RasterApp {
        let mut app = framework_testkit::new_app::<EditorApp<RasterPlayApp>>().await;
        let document = crate::artifacts::raster::schema::semio_example_document();
        let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster", document, None);
        let files = store::print_document_pack(&envelope).await.expect("print document pack");
        app.load_document_pack(&files).await.expect("load semio");
        app
    }
}

#[cfg(test)]
mod tests {
    use super::testkit::*;
    use super::*;
    use crate::artifacts::raster::schema::{empty_raster_document, layer_name, layer_visible};
    use crate::editor::raster::panels::{catalogue, document, inspection, masks};
    use semio_framework_plugin::{testkit, PluginApp, SET_ACTIVE_UTILITY_ACTION_ID};
    use store::MemoryBackbone;

    //#region 🔖️RetainedEnvelopeIngress
    /// 📨️ One live document-envelope wire, built from the artifact's own empty output shell — the only
    /// snapshot `ArtifactPack::encode_pack` admits, since a populated Raster document reaches output
    /// exclusively through the retained page authority. The envelope the wire describes is retired here
    /// through the same owner bundle the app's decode hook installs, one bounded grant per turn.
    fn raster_envelope_wire() -> Vec<u8> {
        use store::ArtifactPack;

        let snapshot = crate::artifacts::raster::schema::empty_raster_snapshot();
        let snapshot_pack = snapshot.encode_pack();
        let snapshot_hex = snapshot_pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let wire = dsl::json::to_string(&dsl::json::object([
            ("schema".to_string(), Value::String(RASTER_DOCUMENT_SCHEMA.to_string())),
            ("id".to_string(), Value::String("raster-live-load".to_string())),
            (
                "vcs".to_string(),
                dsl::json::object([
                    ("initialSnapshot".to_string(), Value::String(snapshot_hex)),
                    ("edits".to_string(), dsl::json::array([])),
                    ("changes".to_string(), dsl::json::array([])),
                    ("checkpoints".to_string(), dsl::json::array([])),
                    ("alternatives".to_string(), dsl::json::array([])),
                ]),
            ),
            ("editMessages".to_string(), dsl::json::array([])),
            ("conflicts".to_string(), dsl::json::array([])),
        ]))
        .into_bytes();
        let envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster-live-load", snapshot, None);
        let mut retirement = crate::artifacts::raster::spr::raster_envelope_decode_owner_bundle().retire_envelope(envelope);
        for _ in 0..100_000 {
            match retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Raster fixture envelope retirement") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    drop(retirement);
                    return wire;
                }
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
                }
                store::SnapshotRetirementStep::Blocked => panic!("unshared Raster fixture envelope retirement blocked"),
            }
        }
        panic!("Raster fixture envelope retirement did not reach terminal")
    }

    /// 🎟️ Reserves page/byte credits first, then feeds the wire as fixed-size pages and seals — the
    /// caller never holds a growable buffer and never sees the store the decode will publish into.
    fn admit_raster_envelope(app: &mut RasterApp, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
        let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
        let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Raster live envelope ingress credits");
        for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
            let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
            bytes[..chunk.len()].copy_from_slice(chunk);
            let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Raster live envelope page");
            app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Raster live envelope page admission failed: {}: {}", fault.code.0, fault.message));
        }
        assert!(app.seal_artifact_envelope_ingress(handle).expect("Raster live envelope seal/submit"));
        handle
    }

    /// 🔄️ Pumps the load one bounded maintenance turn at a time and polls after each — never inline,
    /// never unbounded.
    fn drive_raster_live_load(app: &mut RasterApp, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
        for _ in 0..100_000 {
            app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one Raster live maintenance turn");
            let poll = app.advance_artifact_envelope_load(handle).expect("Raster live load advancement");
            if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
                return poll;
            }
            std::thread::yield_now();
        }
        panic!("Raster live envelope load did not reach terminal")
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed() {
        let mut app = app().await;
        let base_generation = app.artifact_generation_now();
        let handle = admit_raster_envelope(&mut app, &raster_envelope_wire());
        assert_eq!(handle.generation, base_generation);
        assert_eq!(drive_raster_live_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
        assert_eq!(app.artifact_generation_now().0, base_generation.0 + 1);
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("first exact Raster load acknowledgement"));
        assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate Raster load acknowledgement is a no-op"));
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_live_envelope_cancel_closes_retained_pages_without_publication() {
        let mut app = app().await;
        let base_generation = app.artifact_generation_now();
        let wire = raster_envelope_wire();
        let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
        let handle = app.begin_artifact_envelope_ingress(pages, wire.len()).expect("cancelled Raster ingress credits");
        let first = &wire[..wire.len().min(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)];
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..first.len()].copy_from_slice(first);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, first.len()).expect("cancelled Raster first page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("cancelled Raster page admission failed: {}: {}", fault.code.0, fault.message));
        app.cancel_artifact_envelope_load(handle).expect("cancel exact Raster ingress");
        assert_eq!(drive_raster_live_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(app.artifact_generation_now(), base_generation);
    }
    //#endregion 🔖️RetainedEnvelopeIngress

    /// 🌱️ Relocated verbatim from `⚙️engine`'s own test module (rule 4: `raster_io`/`raster_composite_media`
    /// now live in this file's own `🔖️Io` region).
    #[semio_framework_async_macros::async_test]
    async fn raster_io_declares_image_in_and_image_out() {
        let io = raster_io();
        assert_eq!(io.document_schema, RASTER_DOCUMENT_SCHEMA);
        assert_eq!(io.artifact.id, "2d.raster");
        assert!(io.ports.iter().any(|p| p.id == "image:in"));
        let out_port = raster_image_out_port();
        assert_eq!(out_port.kind_id.as_deref(), Some("2d.image"));
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_composite_media_exports_structured_2d_image_payload() {
        let document = empty_raster_document();
        let media = raster_composite_media(&document).expect("export image:out");
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "2d.image");
        assert!(!json.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn window_measures_expose_brush_and_eraser_option_groups() {
        let mut app = app().await;
        let measures = main_window_measures(&mut app).await;
        assert_eq!(measures.len(), 2);
        assert!(measures.iter().any(|m| matches!(m, WindowMeasure::Group { id, .. } if id == "raster-utility-options-paintBrush")));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_raster_scene() {
        let mut app = app().await;
        let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE).await;
        assert!(json.contains("raster"));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_navigator_scene() {
        let mut app = app().await;
        let json = render(&mut app, navigator::RASTER_PLAY_BODY_NAVIGATOR).await;
        assert!(json.contains("\"componentKind\":\"paint-2d\""));
        assert!(json.contains("\"viewMode\":\"navigator\""));
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_semio_example_document() {
        let document = crate::artifacts::raster::schema::semio_example_document();
        assert!(!document.layers.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_document_background_layer_has_identity_scale() {
        let document = empty_raster_document();
        let json = document_sync_json(&document);
        assert!(json.contains(r#""scaleX":1.0"#), "expected identity scale in {json}");
        assert!(json.contains(r#""scaleY":1.0"#), "expected identity scale in {json}");
        assert!(!json.contains(r#""scaleX":0.0"#), "layer must not collapse to zero size");
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_layers_tree() {
        let mut app = semio_app().await;
        let json = render(&mut app, document::RASTER_PLAY_BODY_LAYERS).await;
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Backdrop"));
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_labels_resolve_native_english_by_default() {
        let mut app = app().await;
        let layers_json = render(&mut app, document::RASTER_PLAY_BODY_LAYERS).await;
        assert!(layers_json.contains("Add Pixel"));
        assert!(layers_json.contains("Add Group"));
        let masks_json = render(&mut app, masks::RASTER_PLAY_BODY_MASKS).await;
        assert!(masks_json.contains("Masks"));
        assert!(masks_json.contains("No masks"));
        let catalogue_json = render(&mut app, catalogue::RASTER_PLAY_BODY_CATALOGUE).await;
        assert!(catalogue_json.contains("Layer kinds"));
        assert!(catalogue_json.contains("raster-catalogue.pixel"));
        assert!(catalogue_json.contains("raster-catalogue.group"));
        assert!(catalogue_json.contains("raster-catalogue.adjustment"));
        let properties_json = render(&mut app, inspection::RASTER_PLAY_BODY_PROPERTIES).await;
        assert!(properties_json.contains("raster-play-inspector.schema"));
        assert!(properties_json.contains(RASTER_DOCUMENT_SCHEMA));
        assert!(properties_json.contains("raster-play-inspector.brush"));
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_labels_resolve_german_locale() {
        let mut app = app().await;
        dispatch(&mut app, RasterCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() })).await;
        let layers_json = render(&mut app, document::RASTER_PLAY_BODY_LAYERS).await;
        assert!(layers_json.contains("Pixel hinzufügen"));
        assert!(layers_json.contains("Gruppe hinzufügen"));
        let masks_json = render(&mut app, masks::RASTER_PLAY_BODY_MASKS).await;
        assert!(masks_json.contains("Masken"));
        assert!(masks_json.contains("Keine Masken"));
        let catalogue_json = render(&mut app, catalogue::RASTER_PLAY_BODY_CATALOGUE).await;
        assert!(catalogue_json.contains("Ebenenarten"));
    }

    #[semio_framework_async_macros::async_test]
    async fn composite_scene_syncs_document_and_assets() {
        let mut app = semio_app().await;
        let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE).await;
        assert!(json.contains("\"componentKind\":\"paint-2d\""));
        assert!(json.contains("\"viewMode\":\"composite\""));
        assert!(!json.contains("\"assetsJson\":\"{}\""), "semio fixture has embedded assets");
        let document = crate::artifacts::raster::schema::semio_example_document();
        let sync_json = document_sync_json(&document);
        assert!(!sync_json.contains("\"assets\""), "sync json must omit assets");
        assert!(sync_json.contains("\"params\""), "adjustment params must survive document→sync roundtrip for the paint host");
        let sync_value: Value = dsl::os_pack::json::parse(&sync_json).expect("sync json");
        let layers = sync_value.get("layers").and_then(Value::as_array).expect("layers");
        assert!(layers.iter().any(|layer| layer.get("kind").and_then(Value::as_str) == Some("adjustment") && layer.get("params").is_some()));
        assert!(document.assets.contains_key("semio-emblem"));
    }

    #[semio_framework_async_macros::async_test]
    async fn semio_example_preserves_adjustment_params() {
        let document = crate::artifacts::raster::schema::semio_fixture_snapshot();
        let RasterLayerNode::Adjustment { params, adjustment_kind, .. } = document.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Adjustment { id, .. } if id == "brighten")).expect("brighten adjustment") else {
            panic!("expected adjustment");
        };
        assert_eq!(adjustment_kind, "brightnessContrast");
        assert!(params.contains_key("brightness"), "fixture brightness must roundtrip");
        assert!(params.contains_key("contrast"), "fixture contrast must roundtrip");
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: layer hover/selection dispatch
    /// through the framework-injected `interactionHover`/`interactionSelect` verbs against the
    /// `"layers"` domain now (`semio-framework-plugin`'s own suite covers that generic machinery);
    /// this app's contribution is declaring the domain and binding the tree to it.
    #[semio_framework_async_macros::async_test]
    async fn document_tree_binds_the_layers_interaction_domain() {
        let mut app = semio_app().await;
        let json = render(&mut app, document::RASTER_PLAY_BODY_LAYERS).await;
        assert!(json.contains("\"interactionDomain\":\"layers\""), "layer tree must bind the framework-owned layers domain: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_composite_viewport_feeds_navigator_scene() {
        let mut app = app().await;
        dispatch(&mut app, RasterCommand::SetCompositeViewport(set_composite_viewport::SetCompositeViewport { width: 640.0, height: 480.0 })).await;
        let json = render(&mut app, navigator::RASTER_PLAY_BODY_NAVIGATOR).await;
        assert!(json.contains("compositeViewportJson"));
        assert!(json.contains(r#"\"width\":640.0"#));
        assert!(json.contains(r#"\"height\":480.0"#));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_camera_mutates_runtime_and_emits_no_operations() {
        let mut app = app().await;
        let before = app.snapshot().expect("snapshot");
        let result = dispatch(&mut app, RasterCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::raster::RasterCamera { x: 4.0, y: 5.0, zoom: 2.0 } })).await;
        assert!(result.mutations.is_empty(), "camera is a view action and emits no operations");
        assert_eq!(app.snapshot().expect("snapshot"), before, "camera never mutates the document");
        let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE).await;
        assert!(json.contains(r#"\"zoom\":2.0"#), "composite scene camera reflects runtime state: {json}");
        assert!(json.contains(r#"\"x\":4.0"#), "composite scene camera reflects runtime state: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_camera_zoom_updates_zoom_and_keeps_pan_via_runtime() {
        let mut app = app().await;
        dispatch(&mut app, RasterCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::raster::RasterCamera { x: 4.0, y: 5.0, zoom: 1.0 } })).await;
        let result = dispatch(&mut app, RasterCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { zoom: 3.0 })).await;
        assert!(result.mutations.is_empty(), "camera zoom is a view action and emits no operations");
        let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE).await;
        assert!(json.contains(r#"\"zoom\":3.0"#), "zoom updated: {json}");
        assert!(json.contains(r#"\"x\":4.0"#), "pan preserved across zoom-only update: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn add_layer_action_appends_and_undo_removes() {
        let mut app = app().await;
        let before = app.snapshot().expect("snapshot").layers.len();
        dispatch(&mut app, RasterCommand::AddLayer(add_layer::AddLayer { kind: "group".into() })).await;
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(projection.layers.len(), before + 1);
        assert!(matches!(projection.layers.last().unwrap(), RasterLayerNode::Group { .. }));
        app.handle_action("undo", None, &testkit::meta("local")).await.expect("undo");
        assert_eq!(app.snapshot().expect("snapshot").layers.len(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_layer_renames_and_toggles_visibility_round_trip() {
        let mut app = app().await;
        let layer_id = crate::artifacts::raster::schema::layer_node_id(&app.snapshot().expect("snapshot").layers[0]).to_string();
        dispatch(&mut app, RasterCommand::PatchLayer(patch_layer::PatchLayer { layer_id: layer_id.clone(), field: "name".into(), value: "Renamed".into() })).await;
        assert_eq!(layer_name(&app.snapshot().expect("snapshot").layers[0]), "Renamed");
        dispatch(&mut app, RasterCommand::ToggleLayerVisible(toggle_layer_visible::ToggleLayerVisible { layer_id })).await;
        assert!(!layer_visible(&app.snapshot().expect("snapshot").layers[0]));
        app.handle_action("undo", None, &testkit::meta("local")).await.expect("undo toggle");
        assert!(layer_visible(&app.snapshot().expect("snapshot").layers[0]));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_layer_into_group() {
        let mut app = app().await;
        dispatch(&mut app, RasterCommand::AddLayer(add_layer::AddLayer { kind: "group".into() })).await;
        let (group_id, pixel_id) = {
            let projection = app.snapshot().expect("snapshot");
            let group = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Group { .. })).unwrap();
            let pixel = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Pixel { .. })).unwrap();
            (crate::artifacts::raster::schema::layer_node_id(group).to_string(), crate::artifacts::raster::schema::layer_node_id(pixel).to_string())
        };
        let target_row = format!("{RASTER_TREE_PREFIX}.group.{group_id}");
        dispatch(&mut app, RasterCommand::MoveLayer(move_layer::MoveLayer { layer_id: pixel_id.clone(), target_row_id: target_row, drop_position: "after".into() })).await;
        let projection = app.snapshot().expect("snapshot");
        let RasterLayerNode::Group { children, .. } = projection.layers.iter().find(|layer| crate::artifacts::raster::schema::layer_node_id(layer) == group_id).unwrap() else {
            panic!("expected group");
        };
        assert_eq!(children.len(), 1);
        assert_eq!(crate::artifacts::raster::schema::layer_node_id(&children[0]), pixel_id);
    }

    /// 🧪️ The definitional merge proof: A adds a layer while B renames the background layer — disjoint
    /// tree edits on one backbone that must both survive on both instances.
    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_disjoint_layer_edits_via_backbone() {
        let mut instance_a = app().await;
        let mut instance_b = app().await;
        // Seed both from an identical base projection (a background layer with a fixed id) so B's
        // rename targets the same layer A holds — per-instance `initial_snapshot` mints fresh ids.
        let mut base = crate::artifacts::raster::schema::empty_raster_snapshot();
        base.layers = vec![RasterLayerNode::Pixel {
            id: "bg".into(),
            name: "Background".into(),
            visible: true,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: crate::artifacts::raster::RasterTransform::default(),
            mask: None,
            width: Some(512),
            height: Some(512),
            image_key: None,
        }];
        let base_envelope = store::create_document_envelope::<RasterSnapshot, RasterMutation>(RASTER_DOCUMENT_SCHEMA, "raster", base, None);
        let base_files = store::print_document_pack(&base_envelope).await.expect("print document pack");
        instance_a.load_document_pack(&base_files).await.expect("load a");
        instance_b.load_document_pack(&base_files).await.expect("load b");
        let background_id = "bg".to_string();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://raster-convergence", "mem://raster-convergence").await;
        instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
        instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");

        dispatch(&mut instance_a, RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() })).await;
        dispatch(&mut instance_b, RasterCommand::PatchLayer(patch_layer::PatchLayer { layer_id: background_id, field: "name".into(), value: "Renamed By B".into() })).await;

        instance_a.handle_action("commitCheckpoint", None, &testkit::meta("actor-a")).await.expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &testkit::meta("actor-b")).await.expect("pump b");

        let projection_a = instance_a.snapshot().expect("projection a");
        let projection_b = instance_b.snapshot().expect("projection b");
        assert_eq!(projection_a.layers.len(), 2, "A keeps its added layer");
        assert_eq!(projection_b.layers.len(), 2, "B converges on A's added layer");
        assert_eq!(layer_name(&projection_a.layers[0]), "Renamed By B", "A converges on B's rename");
        assert_eq!(layer_name(&projection_b.layers[0]), "Renamed By B", "B keeps its rename");
    }

    #[semio_framework_async_macros::async_test]
    async fn ingest_operations_is_idempotent() {
        testkit::assert_ingest_idempotent::<EditorApp<RasterPlayApp>, usize>(RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() }), |app| app.snapshot().unwrap().layers.len()).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_utility_switch_emits_no_ops_and_persists_in_config() {
        let mut app = app_with_registry().await;
        let before = app.snapshot().expect("snapshot");
        // Switching utilities is the framework View action: no document operations, nothing to sync/undo.
        let result = dispatch(&mut app, RasterCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "paintBrush".into() })).await;
        assert!(result.mutations.is_empty(), "utility switching never emits document operations");
        assert_eq!(app.snapshot().expect("snapshot"), before, "utility switching does not mutate the document");
        // The composite scene reads the host-owned active utility from config, not view state.
        let json = render(&mut app, composite::RASTER_PLAY_BODY_COMPOSITE).await;
        assert!(json.contains("\"activeUtility\":\"paintBrush\""), "scene reflects host-owned active utility: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn utility_registry_declares_utilities_scoped_to_the_composite_window() {
        let definition = create_raster_app();
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["selectMarquee", "paintBrush", "paintEraser"]);
        // The marquee carries the Selection category; the paint utilities are Tools.
        let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
        assert_eq!(selects, ["selectMarquee"]);
        let composite = definition.window_kinds.iter().find(|window| window.id == composite::RASTER_PLAY_WINDOW_COMPOSITE).expect("composite window");
        assert_eq!(composite.utilities.len(), definition.utilities.len(), "every utility is scoped to the composite window kind");
        // The framework auto-injects the setActiveUtility View action once utilities are declared; no doc operation survives.
        assert!(composite.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID && matches!(action.kind, ActionKind::View)));
        assert!(!definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == "setActiveUtility" && !matches!(action.kind, ActionKind::View)));
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_io_declares_image_in_out_and_export_media_covers_all_ports() {
        let projection = empty_raster_document();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&projection, &history);
        let image_out = RasterPlayApp::export_media("image:out", &doc).expect("image:out");
        let MediaPayload::Structured { schema, json } = image_out.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "2d.image");
        assert!(!json.is_empty());
        assert!(RasterPlayApp::export_media("document:out", &doc).is_ok());
        assert!(matches!(RasterPlayApp::export_media("unknown:out", &doc), Err(MediaError::NotImplemented)));
    }

    #[semio_framework_async_macros::async_test]
    async fn raster_import_media_appends_layer_from_incoming_image() {
        let mut app = app().await;
        let before = app.snapshot().expect("snapshot").layers.len();
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: "aGVsbG8=".into() } };
        let result = app.import_media("image:in", media, &testkit::meta("local")).await.expect("import image:in");
        assert!(!result.mutations.is_empty(), "image:in import must emit a real document operation");
        assert_eq!(app.snapshot().expect("snapshot").layers.len(), before + 1);
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order — TEMPLATE.md §7's
    /// permanent wire guard, feeding the round-trip/keyword-uniqueness/leading-token laws below.
    fn every_command() -> Vec<RasterCommand> {
        vec![
            RasterCommand::AddLayer(add_layer::AddLayer { kind: "pixel".into() }),
            RasterCommand::DropLayerKind(drop_layer_kind::DropLayerKind { kind: "group".into() }),
            RasterCommand::SetLayerVisible(set_layer_visible::SetLayerVisible { layer_id: "l1".into(), visible: Some(true) }),
            RasterCommand::ToggleLayerVisible(toggle_layer_visible::ToggleLayerVisible { layer_id: "l1".into() }),
            RasterCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: "l1".into() }),
            RasterCommand::DuplicateLayer(duplicate_layer::DuplicateLayer { layer_id: "l1".into() }),
            RasterCommand::PatchLayer(patch_layer::PatchLayer { layer_id: "l1".into(), field: "opacity".into(), value: "0.4".into() }),
            RasterCommand::PatchLayers(patch_layers::PatchLayers { layer_ids: vec!["a".into(), "b".into()], field: "name".into(), value: "Renamed".into() }),
            RasterCommand::MoveLayer(move_layer::MoveLayer { layer_id: "l1".into(), target_row_id: "raster-play-layers".into(), drop_position: "after".into() }),
            RasterCommand::SetBrushSize(set_brush_size::SetBrushSize { value: 40.0 }),
            RasterCommand::SetBrushOpacity(set_brush_opacity::SetBrushOpacity { value: 0.5 }),
            RasterCommand::SetCompositeViewport(set_composite_viewport::SetCompositeViewport { width: 640.0, height: 480.0 }),
            RasterCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::raster::RasterCamera { x: 1.0, y: 2.0, zoom: 1.5 } }),
            RasterCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { zoom: 2.0 }),
            RasterCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "paintBrush".into() }),
            RasterCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            RasterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::artifacts::raster::examples::demo::ID.into() }),
        ]
    }

    /// ⚖️ LAW: raster's retained route table, its publication contracts, its bounded-first-step proofs,
    /// `RasterCommand::TOOL_JOB_IDS` and the catalog's `Migrated` classifications are the SAME
    /// seventeen ids — the exact join `validate_tool_job_rows` demands
    /// (`interactive-job.catalog-authority` / `interactive-job.catalog-incomplete`). Mirrors block2d's
    /// `retained_route_dispositions_are_exact_and_exhaustive`.
    #[semio_framework_async_macros::async_test]
    async fn retained_route_dispositions_are_exact_and_exhaustive() {
        use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};
        use std::collections::BTreeSet;
        assert_eq!(RASTER_RETAINED_TOOL_IDS.len(), 17);
        assert_eq!(<RasterPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 17);
        assert_eq!(RasterRetainedCommandJobFactory::PUBLICATION_CONTRACTS.len(), 17);
        assert_eq!(raster_retained_contract().shape, ToolExecutionShape::BoundedFirstStep);
        assert_eq!(raster_retained_contract().cancellation, ToolCancellationPolicy::PerOperation);

        let retained: BTreeSet<&str> = RASTER_RETAINED_TOOL_IDS.iter().copied().collect();
        assert_eq!(retained, every_command().iter().map(RasterCommand::command_id).collect::<BTreeSet<_>>(), "every RasterCommand row must be a retained route");
        assert_eq!(retained, RasterCommand::TOOL_JOB_IDS.iter().copied().collect::<BTreeSet<_>>(), "the retained table must equal the generated tool-job id set");

        // 🛣️ Lane discipline, read off the handlers: ten document verbs publish into the artifact lane,
        // seven session verbs into the config lane, and no route publishes into both.
        let artifact_lane: BTreeSet<&str> = ["addLayer", "dropLayerKind", "setLayerVisible", "toggleLayerVisible", "deleteLayer", "duplicateLayer", "patchLayer", "patchLayers", "moveLayer", "setActiveExample"].into_iter().collect();
        for tool_id in RASTER_RETAINED_TOOL_IDS {
            let contract = RasterRetainedCommandJobFactory::PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == *tool_id).unwrap_or_else(|| panic!("publication contract for {tool_id}"));
            let expected = if artifact_lane.contains(tool_id) { ArtifactToolPublicationLane::Artifact } else { ArtifactToolPublicationLane::Config };
            assert_eq!(contract.lanes, [expected].as_slice(), "{tool_id} publishes into exactly one lane");
        }
        assert!(<RasterPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some(), "the Artifact lane is rejected outright without a document one-item preparation factory");
        assert!(<RasterPlayApp as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_some(), "the Config lane is rejected outright without a config one-item preparation factory");

        // 🧵️ Every retained id must be UI-dispatchable — including the framework-injected
        // `setActiveUtility`, which `resumable_framework_catalog` classifies for us.
        let definition = create_raster_app();
        for tool_id in RASTER_RETAINED_TOOL_IDS {
            let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == *tool_id).unwrap_or_else(|| panic!("action {tool_id} declared"));
            assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id} must be UI-dispatchable");
        }
        // ⚖️ No declaration in raster's catalog may stay `Unclassified` — that is the release-blocking
        // gate `validate_interactive_job_classification` enforces. Every action lands on a window kind
        // (both app-declared and framework-injected ones), so this sweep sees the whole action surface.
        for action in definition.window_kinds.iter().flat_map(|window| window.actions.iter()) {
            assert_ne!(action.semantics.execution.interactive_job, InteractiveJobClassification::Unclassified, "action {} is unclassified", action.id);
        }
        for command in definition.commands.iter() {
            assert_ne!(command.semantics.execution.interactive_job, InteractiveJobClassification::Unclassified, "command {} is unclassified", command.id);
        }

        // ⚖️ The gate's own arithmetic, spelled out: `expected = TOOL_JOB_IDS ∩ migrated` must equal the
        // proof set. `RASTER_RETAINED_TOOL_IDS` names that proof set (its length is asserted equal to
        // `bounded_first_step_tool_proofs().len()` above, and the macro derives the proofs from the same
        // literal list), so a route that is proven but left unclassified — or classified but unproven —
        // fails here instead of at runtime with `interactive-job.catalog-authority`.
        let migrated_ids: BTreeSet<&str> = definition
            .window_kinds
            .iter()
            .flat_map(|window| window.actions.iter())
            .filter(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated)
            .map(|action| action.id.as_str())
            .collect();
        assert_eq!(
            RasterCommand::TOOL_JOB_IDS.iter().copied().filter(|tool_id| migrated_ids.contains(tool_id)).collect::<BTreeSet<_>>(),
            retained,
            "every bounded-first-step proof entry must be Migrated, and every Migrated tool-job row must be proven"
        );
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🎫️ Every `app_commands!` row's wire keyword must be distinct — the cross-cutting invariant the
    /// macro exists to hold.
    #[semio_framework_async_macros::async_test]
    async fn command_wire_keywords_are_unique_across_every_row() {
        let commands = every_command();
        assert_eq!(commands.len(), 17, "every RasterCommand row must be covered by every_command()");
        let mut keywords: Vec<String> = commands.iter().map(|command| protocol::OpText::print_op(command).split(' ').next().unwrap_or_default().to_string()).collect();
        keywords.sort();
        keywords.dedup();
        assert_eq!(keywords.len(), commands.len(), "every row's wire keyword must be distinct");
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — what a
    /// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
    /// keyword at all and no longer parses).
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_declared_wire_keyword() {
        let expectations: Vec<(&str, RasterCommand)> = every_command()
            .into_iter()
            .map(|command| {
                let keyword: &'static str = match &command {
                    RasterCommand::AddLayer(_) => "add-layer",
                    RasterCommand::DropLayerKind(_) => "drop-layer-kind",
                    RasterCommand::SetLayerVisible(_) => "set-layer-visible",
                    RasterCommand::ToggleLayerVisible(_) => "toggle-layer-visible",
                    RasterCommand::DeleteLayer(_) => "delete-layer",
                    RasterCommand::DuplicateLayer(_) => "duplicate-layer",
                    RasterCommand::PatchLayer(_) => "patch-layer",
                    RasterCommand::PatchLayers(_) => "patch-layers",
                    RasterCommand::MoveLayer(_) => "move-layer",
                    RasterCommand::SetBrushSize(_) => "brush-size",
                    RasterCommand::SetBrushOpacity(_) => "brush-opacity",
                    RasterCommand::SetCompositeViewport(_) => "composite-viewport",
                    RasterCommand::SetCamera(_) => "camera",
                    RasterCommand::SetCameraZoom(_) => "camera-zoom",
                    RasterCommand::SetActiveUtility(_) => "active-utility",
                    RasterCommand::SetLocale(_) => "locale",
                    RasterCommand::SetActiveExample(_) => "set-active-example",
                };
                (keyword, command)
            })
            .collect();
        for (expected_keyword, command) in expectations {
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for {command:?}: {printed:?}");
        }
    }

    /// ⚖️ The rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to exact
    /// bytes so an ACCIDENTAL row reorder is caught. Baseline rebased once, deliberately, by the
    /// `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL` ticket: dropping the two leading `setSnapshot`/
    /// `setActiveExample` rows (whole-document replace is no longer expressible as a mutation)
    /// shifted every later row's binary ordinal down by two — `set-layer-visible` 4→2 (`0104`→`0102`).
    /// `setActiveExample` returned in `26/09/05/RASTER-PLUGIN-END-TO-END` as an ordered mutation batch
    /// rather than a snapshot swap, APPENDED as the last row, so no earlier ordinal moved.
    /// Rebased again by `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`: the `set-selection`/
    /// `set-hover`/`select-all` rows (the only other `Option`-carrying case, `set-hover`) are deleted
    /// outright — layer selection/hover is the framework-owned `"layers"` interaction domain now.
    /// `set-layer-visible`'s ordinal is unaffected (it sits before the deleted rows). Greenfield repo,
    /// no persisted wire data to migrate. Any FURTHER drift here is a real format break, not a
    /// fixture mismatch.
    #[semio_framework_async_macros::async_test]
    async fn optional_field_rows_keep_their_declared_wire_bytes() {
        let cases: [(RasterCommand, &str, &str); 1] = [(RasterCommand::SetLayerVisible(set_layer_visible::SetLayerVisible { layer_id: "l1".into(), visible: None }), "set-layer-visible set-layer-visible layer-id=l1", "010201026c3101000600")];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text, "printed text drifted for {command:?}");
            let bytes = protocol::OpBinary::encode_op(&command).expect("encode");
            assert_eq!(bytes.iter().map(|b| format!("{b:02x}")).collect::<String>(), hex, "binary bytes drifted for {command:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_across_every_row() {
        let mut seen = std::collections::HashSet::new();
        for command in every_command() {
            assert!(seen.insert(command.command_id().to_string()), "duplicate command_id {}", command.command_id());
        }
    }
}
//#endregion 🧪️Tests
