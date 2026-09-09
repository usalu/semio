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
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDescriptor, ActionFactory, ActionKind, AppDefinition, AppOperationContext, ArtifactEditor, ArtifactKindSpec, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry,
    ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label,
    LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode, NoDraft, NoDraftMutation, OsMediaCapability, SelectionMethod, SelectionMode, SelectionSpec, UtilityCategory, UtilityDefinition, WindowMeasure,
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

/// 📡️ Document JSON for the WASM compositor, omitting embedded assets/utility/brush — mirrors
/// premigration `rasterDocumentToSyncJson`. Takes `&RasterConfig` nowhere directly (assets live on the
/// document), but stays app-level next to {@link raster_scene}, its only caller.
fn document_sync_json(document: &RasterSnapshot) -> String {
    let value = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(document));
    let value = match value {
        Value::Object(object) => {
            let filtered: dsl::os_pack::json::Object = object.iter().filter(|(key, _)| *key != "assets" && *key != "brushSize" && *key != "brushOpacity").map(|(key, value)| (key.to_string(), value.clone())).collect();
            Value::Object(filtered)
        }
        other => other,
    };
    value.to_string()
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

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes.try_push(node).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
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

    const DIALECT: Dialect = crate::RASTER_DIALECT;
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

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
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

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::raster::config::schema::app_schema_descriptor())
    }

    /// 📄️ Boots on the bundled `📚️examples/🎬️demo` Semio-logo carrier (the same `.dsl.semio` text
    /// `setActiveExample` loads), so every window renders real content instead of the all-`Default`
    /// scaffold. `empty_raster_document()` stays the tests' blank slate — mirrors block2d's
    /// `default_block2d_snapshot`.
    fn initial_snapshot() -> RasterSnapshot {
        crate::standards::v1::subsets::any::schema::default_raster_document()
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
        let (asset_id, asset, layer) = crate::io::raster_image_layer_and_asset(png_base64);
        Ok(Emit::mutations(vec![
            RasterMutation::AddLayerAsset(crate::mutations::add_layer_asset::mutation::AddLayerAsset { asset_id, asset }),
            RasterMutation::CreateLayer(crate::mutations::create_layer::mutation::CreateLayer { parent_id: None, index: doc.snapshot.layers.len(), layer: Box::new(layer) }),
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
        let node = match body_key {
            composite::RASTER_PLAY_BODY_COMPOSITE => composite::render(document, config, active_utility)?,
            navigator::RASTER_PLAY_BODY_NAVIGATOR => navigator::render(document, config, active_utility)?,
            crate::editor::raster::panels::document::RASTER_PLAY_BODY_LAYERS => crate::editor::raster::panels::document::render(document, config, labels)?,
            crate::editor::raster::panels::masks::RASTER_PLAY_BODY_MASKS => crate::editor::raster::panels::masks::render(document, config, labels)?,
            crate::editor::raster::panels::catalogue::RASTER_PLAY_BODY_CATALOGUE => crate::editor::raster::panels::catalogue::render(labels)?,
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
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
