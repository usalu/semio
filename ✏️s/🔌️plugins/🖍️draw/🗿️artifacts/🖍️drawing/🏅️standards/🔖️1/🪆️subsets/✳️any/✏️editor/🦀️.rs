//! 🖥️ Drawing editor surface — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum
//! and the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window
//! render in `🎭️modes/✏️edit/🪟️windows/🖼️canvas`, panel trees in `📌️panels/*`, labels in
//! `🦀️terminology.rs`, view state in `🦀️config.rs`.
//! This file is a routing table: `handle` → `DrawingCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use crate::editor::drawing::commands::canvas_pointer_down::{DrawingGesturePreview, DrawingSession};
use crate::editor::drawing::commands::{
    add_layer, canvas_commit_draft, canvas_double_click, canvas_escape, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, combine_boolean, commit_document, delete_layer, drop_layer_kind, duplicate_layer, engagement_input,
    engagement_submit, move_layer, patch_layer, patch_layers, set_active_example, set_camera, set_camera_zoom, set_fixture_json, set_selected_opacity, set_snapshot, toggle_layer_visible,
};
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use crate::editor::drawing::modes::edit;
use crate::editor::drawing::modes::edit::windows::canvas as canvas_window;
use crate::editor::drawing::panels::{catalogue as catalogue_panel, layers as layers_panel, properties as properties_panel};
use crate::editor::drawing::presence::{DrawingPresence, DrawingPresenceMutation};
use crate::editor::drawing::terminology::DrawingPlayLabels;
use crate::op::DrawingMutation;
use crate::{DrawingSnapshot, DRAWING_DOCUMENT_SCHEMA};
use semio_framework_job::FixedOperationOwner;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ActionDescriptor, ActionKind, ArtifactEditor, ArtifactView, ConfigView, DraftView, Editor, Emit, Fault, FaultCode, FaultOrigin, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label, LocalizedLabel,
    Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, UtilityCategory, UtilityDefinition, WindowEngagement, WindowEngagementInput,
    WindowEngagementStatus,
};
use store::ArtifactPack;
use store::EngineHandles;

pub use canvas_window::{DRAWING_PLAY_BODY_COMPOSITE, DRAWING_PLAY_WINDOW_CANVAS};
pub use catalogue_panel::DRAWING_PLAY_BODY_CATALOGUE;
pub use layers_panel::{DRAWING_LAYER_KIND_DRAG_MIME, DRAWING_PLAY_BODY_LAYERS};
pub use properties_panel::DRAWING_PLAY_BODY_PROPERTIES;

//#region 🔖️Constants
pub const DRAWING_PLAY_CONTROLLER_ID: &str = "drawing-play";
/// 🧰️ The utility the canvas returns to after committing a shape/draft/trace (first UtilityRef default).
pub const DRAWING_DEFAULT_UTILITY: &str = "selectDirect";
/// 🕹️ The single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM interaction domain this app declares
/// (granularity `stroke`, `HierarchyProvider::Flat`, methods Pick/Rectangle/Lasso).
pub const DRAWING_INTERACTION_DOMAIN: &str = "strokes";
pub const DRAWING_INTERACTION_GRANULARITY: &str = "stroke";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn drawing_play_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(DRAWING_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🎛️ Builds one manifest-side engagement action without crossing into the retained UI wire action type.
fn drawing_manifest_action(action: &str) -> ActionDescriptor {
    ActionDescriptor { controller_id: DRAWING_PLAY_CONTROLLER_ID.into(), action: action.into(), args: None }
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

/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector-bound vocabulary
/// that is dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn drawing_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> semio_framework_plugin::ActionDefinition {
    semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog(id, label, kind) }
}

/// 🧰️ One canvas utility declaration (id/label/icon reused verbatim from the retired `utilities()` impl).
fn drawing_utility(id: &str, label: impl Into<LocalizedLabel>, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `DrawingPlayApp::Command` — the SOLE dispatch surface for drawing's own behavior, covering every
    /// action `create_drawing_app` declares. Field shapes mirror each action's real `args` object.
    /// **Row order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum DrawingCommand for DrawingSnapshot, DrawingMutation, DrawingConfig, DrawingConfigMutation, ctx = DrawingSession {
        "setSnapshot" as "set-snapshot" => set_snapshot::SetSnapshot,
        "commitDocument" as "commit-document" => commit_document::CommitDocument,
        "setFixtureJson" as "fixture-json" => set_fixture_json::SetFixtureJson,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "setSelectedOpacity" as "selected-opacity" => set_selected_opacity::SetSelectedOpacity,
        "engagementSubmit" as "engagement-submit" => engagement_submit::EngagementSubmit,
        "addLayer" as "add-layer" => add_layer::AddLayer,
        "dropLayerKind" as "drop-layer-kind" => drop_layer_kind::DropLayerKind,
        "moveLayer" as "move-layer" => move_layer::MoveLayer,
        "deleteLayer" as "delete-layer" => delete_layer::DeleteLayer,
        "duplicateLayer" as "duplicate-layer" => duplicate_layer::DuplicateLayer,
        "toggleLayerVisible" as "toggle-layer-visible" => toggle_layer_visible::ToggleLayerVisible,
        "combineBoolean" as "combine-boolean" => combine_boolean::CombineBoolean,
        "patchLayer" as "patch-layer" => patch_layer::PatchLayer,
        "patchLayers" as "patch-layers" => patch_layers::PatchLayers,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setCameraZoom" as "camera-zoom" => set_camera_zoom::SetCameraZoom,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "canvasDoubleClick" as "canvas-double-click" => canvas_double_click::CanvasDoubleClick,
        "canvasCommitDraft" as "canvas-commit-draft" => canvas_commit_draft::CanvasCommitDraft,
        "canvasEscape" as "canvas-escape" => canvas_escape::CanvasEscape,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.

//#endregion 🔖️Commands

//#region 🧵️GestureOperationJobs
const DRAWING_GESTURE_TOOL_IDS: &[&str] = &["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "canvasCommitDraft", "canvasEscape"];
const DRAWING_GESTURE_RAW_BYTES: usize = 8_192;
const DRAWING_GESTURE_RETAINED_BYTES: usize = 32_768;

/// 🛣️ One publication lane row per gesture route, read off each route's real `Emit` construction, not
/// off its `ActionKind`: every gesture that reaches a commit does so through
/// `commit_with_utility_reset`/`DrawingDraftQuery::advance` (artifact lane), and the two routes that
/// additionally write `DrawingConfigMutation::SetTracePointerProgress` — `canvasPointerDown` through
/// `advance_trace_pointer`, `canvasEscape` through its own trace cancellation — carry the config lane
/// as well. Under-declaring a lane that is actually emitted faults at publication time.
const DRAWING_GESTURE_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasDoubleClick", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasCommitDraft", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasEscape", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
];

struct DrawingGestureOperationOwner {
    session: Option<DrawingSession>,
    closing: bool,
}

impl DrawingGestureOperationOwner {
    fn new(active_utility_id: &str) -> Self {
        Self { session: Some(DrawingSession::with_active_utility(active_utility_id)), closing: false }
    }
}

impl FixedOperationOwner for DrawingGestureOperationOwner {
    fn retained_bytes(&self) -> usize {
        DRAWING_GESTURE_RETAINED_BYTES
    }

    fn cancel(&mut self) {
        self.closing = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 || maximum_bytes < DRAWING_GESTURE_RETAINED_BYTES {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if self.session.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: DRAWING_GESTURE_RETAINED_BYTES };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.session.is_none()
    }
}

struct DrawingInstanceOperationOwner {
    operations: semio_framework_job::FixedOperationRegistry<DrawingGestureOperationOwner, 64>,
    active: Option<(semio_framework_job::FixedOperationKey, [u8; 32])>,
    closing: bool,
}

impl DrawingInstanceOperationOwner {
    fn new() -> Self {
        Self { operations: semio_framework_job::FixedOperationRegistry::new(64 * DRAWING_GESTURE_RETAINED_BYTES), active: None, closing: false }
    }

    fn dispatch(&mut self, payload: &DrawingGestureOperationPayload) -> Result<Option<Emit<DrawingMutation, DrawingConfigMutation, NoDraftMutation>>, Fault> {
        let key = semio_framework_job::FixedOperationKey::new(semio_framework_job::OperationId(payload.operation_context.operation_id), semio_framework_job::Generation(payload.operation_context.generation));
        let base_revision = payload.operation_context.canonical_base_revision;
        let command = &payload.command;
        let snapshot = payload.snapshot.as_ref();
        let config = payload.config.as_ref();
        let active_utility_id = payload.active_utility_id.as_str();
        let history = payload.history.as_ref();
        let operation = payload.operation_context.clone();
        if self.closing {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.closing"), "the Drawing gesture operation owner is closing"));
        }
        if let Some((active, observed_revision)) = self.active {
            let same_utility = self.operations.get(active).and_then(|owner| owner.session.as_ref()).is_some_and(|session| session.active_utility_id == active_utility_id);
            if observed_revision != base_revision || !same_utility {
                self.operations.cancel(active);
                self.active = None;
            }
        }
        let live_key = self.active.map_or(key, |(active, _)| active);
        if self.operations.get(live_key).is_none() {
            self.operations.admit(live_key, DrawingGestureOperationOwner::new(active_utility_id)).map_err(|mut rejected| {
                rejected.owner.cancel();
                rejected.owner.begin_close();
                let _ = rejected.owner.close_step(1, DRAWING_GESTURE_RETAINED_BYTES);
                Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.saturated"), "the fixed Drawing gesture operation authority is saturated")
            })?;
            self.active = Some((live_key, base_revision));
        }
        let retained = self.operations.get_mut(live_key).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.owner"), "the exact Drawing gesture owner changed before its bounded reducer step"))?;
        let session = retained.session.as_mut().ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.owner"), "the Drawing gesture session is already closing"))?;
        if session.gesture.context.points_overflowed {
            self.operations.cancel(live_key);
            self.active = None;
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.point-capacity"), "the fixed Drawing gesture point capacity was exceeded"));
        }
        if let Some(query) = session.draft_query.as_mut() {
            if query.command_id != command.command_id() {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.draft-owner"), "a retained Drawing draft query rejects a different command owner"));
            }
            let Some(emit) = query.advance(snapshot) else { return Ok(None) };
            session.draft_query = None;
            if session.gesture.matches("idle") && session.trace_pointer.is_none() {
                self.operations.cancel(live_key);
                self.active = None;
            }
            return Ok(Some(emit));
        }
        if let Some(query) = session.point_query.as_mut() {
            if query.command_id != command.command_id() {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.query-owner"), "a retained Drawing point query rejects a different command owner"));
            }
            if !query.traversal_complete {
                if !query.cursor.advance(snapshot) {
                    return Ok(None);
                }
                if query.cursor.overflowed {
                    session.point_query = None;
                    self.operations.cancel(live_key);
                    self.active = None;
                    return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.query-capacity"), "the fixed Drawing query result capacity was exceeded"));
                }
                query.traversal_complete = true;
                return Ok(None);
            }
            let targets = match query.publication_step() {
                canvas_pointer_down::DrawingQueryPublication::Pending => return Ok(None),
                canvas_pointer_down::DrawingQueryPublication::Complete(targets) => targets,
                canvas_pointer_down::DrawingQueryPublication::Fault => {
                    session.point_query = None;
                    self.operations.cancel(live_key);
                    self.active = None;
                    return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.query-output-capacity"), "the fixed Drawing interaction output capacity was exceeded"));
                }
            };
            let query = session.point_query.take().expect("the exact published query remains retained");
            let effect = if query.hover { canvas_pointer_down::interaction_hover_effect_from_targets(targets) } else { canvas_pointer_down::interaction_select_effect_from_targets(targets, &query.merge) };
            let mut emit = Emit::default();
            emit.effects.push(effect);
            if session.gesture.matches("idle") && session.trace_pointer.is_none() {
                self.operations.cancel(live_key);
                self.active = None;
            }
            return Ok(Some(emit));
        }
        if let DrawingCommand::CanvasPointerMove(payload) = command {
            if session.gesture.matches("idle") {
                let (world_x, world_y) = canvas_pointer_down::canvas_point_to_world(&config.camera, payload.x, payload.y, payload.width, payload.height);
                let tolerance = canvas_pointer_down::DRAWING_PICK_TOLERANCE_PX / config.camera.zoom.max(1e-6);
                session.point_query = Some(canvas_pointer_down::DrawingPointQuery::new(
                    command.command_id(),
                    canvas_pointer_down::TracePointerJob::new_query(snapshot, [world_x, world_y], tolerance, session.active_utility_id == "selectDirect"),
                    true,
                    "replace".into(),
                    false,
                ));
                return Ok(None);
            }
        }
        let retained_emit = match command {
            DrawingCommand::CanvasPointerUp(payload) => {
                let (world_x, world_y) = canvas_pointer_down::canvas_point_to_world(&config.camera, payload.x, payload.y, payload.width, payload.height);
                Some(session.step_gesture_retained(
                    command.command_id(),
                    canvas_pointer_down::drawing_gesture::Event::PointerUp { utility: session.active_utility_id.clone(), world: [world_x, world_y], shift: payload.shift, ctrl: payload.ctrl, meta: payload.meta },
                    snapshot,
                    config,
                ))
            }
            DrawingCommand::CanvasDoubleClick(_) | DrawingCommand::CanvasCommitDraft(_) => Some(session.step_gesture_retained(command.command_id(), canvas_pointer_down::drawing_gesture::Event::CommitDraft, snapshot, config)),
            _ => None,
        };
        if let Some(retained_emit) = retained_emit {
            let Some(emit) = retained_emit else { return Ok(None) };
            if session.gesture.matches("idle") && session.trace_pointer.is_none() {
                self.operations.cancel(live_key);
                self.active = None;
            }
            return Ok(Some(emit));
        }
        let doc = ArtifactView::with_operation(snapshot, history, operation);
        let cfg = ConfigView { snapshot: config, window: None };
        let emit = match command {
            DrawingCommand::CanvasPointerDown(payload) => canvas_pointer_down::handle(payload, &doc, &cfg, session),
            DrawingCommand::CanvasPointerMove(payload) => canvas_pointer_move::handle(payload, &doc, &cfg, session),
            DrawingCommand::CanvasPointerUp(_) | DrawingCommand::CanvasDoubleClick(_) | DrawingCommand::CanvasCommitDraft(_) => unreachable!("retained Drawing gesture commands returned above"),
            DrawingCommand::CanvasEscape(payload) => canvas_escape::handle(payload, &doc, &cfg, session),
            _ => Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.command"), "the retained Drawing gesture owner rejects non-gesture commands")),
        }?;
        if session.gesture.context.points_overflowed {
            self.operations.cancel(live_key);
            self.active = None;
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.point-capacity"), "the fixed Drawing gesture point capacity was exceeded"));
        }
        if session.gesture.matches("idle") && session.trace_pointer.is_none() {
            self.operations.cancel(live_key);
            self.active = None;
        }
        Ok(Some(emit))
    }

    fn preview_projection(&mut self, canonical_base_revision: [u8; 32], active_utility: &str) -> Option<DrawingGesturePreview> {
        let (key, observed_revision) = self.active?;
        if observed_revision != canonical_base_revision {
            self.operations.cancel(key);
            self.active = None;
            return None;
        }
        let session = self.operations.get_mut(key).and_then(|owner| owner.session.as_mut())?;
        if session.active_utility_id != active_utility {
            self.operations.cancel(key);
            self.active = None;
            return None;
        }
        if session.gesture.matches("idle") && session.trace_pointer.is_none() && session.point_query.is_none() && session.draft_query.is_none() {
            self.operations.cancel(key);
            self.active = None;
            return None;
        }
        Some(session.preview())
    }
}

impl semio_framework_plugin::ArtifactInstanceOperationOwner for DrawingInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        Ok(match self.operations.close_step(maximum_items, maximum_bytes) {
            semio_framework_job::InteractiveJobCloseStep::Blocked => semio_framework_plugin::PluginCloseStep::Blocked { reason: "Drawing gesture close owner awaits its exact grant" },
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes },
            semio_framework_job::InteractiveJobCloseStep::Complete => semio_framework_plugin::PluginCloseStep::Complete,
        })
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        self.closing = true;
        if !self.operations.is_empty() {
            self.operations.begin_close_step();
        }
        self.maintenance_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.operations.is_empty()
    }
}

struct DrawingGestureOperationPayload {
    command: DrawingCommand,
    snapshot: std::sync::Arc<DrawingSnapshot>,
    config: std::sync::Arc<DrawingConfig>,
    history: std::sync::Arc<semio_framework_plugin::HistoryView>,
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    operation_context: semio_framework_plugin::AppOperationContext,
    active_utility_id: String,
    completion: semio_framework_plugin::ArtifactToolCompletion<semio_framework_plugin::EditorApp<DrawingPlayApp>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawingRetainedDecodePhase {
    Open,
    VerbOpen,
    Verb,
    Comma,
    Value,
    Close,
    Complete,
    Fault,
}

struct DrawingRetainedCommandDecoder {
    expected_verb: &'static [u8],
    phase: DrawingRetainedDecodePhase,
    verb_cursor: usize,
    nested_depth: usize,
    string: bool,
    escaped: bool,
    scalar: bool,
    value_complete: bool,
}

impl DrawingRetainedCommandDecoder {
    fn new(expected_verb: &'static str) -> Self {
        Self { expected_verb: expected_verb.as_bytes(), phase: DrawingRetainedDecodePhase::Open, verb_cursor: 0, nested_depth: 0, string: false, escaped: false, scalar: false, value_complete: false }
    }

    fn feed(&mut self, byte: u8) {
        if matches!(self.phase, DrawingRetainedDecodePhase::Fault | DrawingRetainedDecodePhase::Complete) {
            if !byte.is_ascii_whitespace() {
                self.phase = DrawingRetainedDecodePhase::Fault;
            }
            return;
        }
        match self.phase {
            DrawingRetainedDecodePhase::Open => {
                if byte.is_ascii_whitespace() {
                    return;
                }
                self.phase = if byte == b'[' { DrawingRetainedDecodePhase::VerbOpen } else { DrawingRetainedDecodePhase::Fault };
            }
            DrawingRetainedDecodePhase::VerbOpen => {
                if byte.is_ascii_whitespace() {
                    return;
                }
                self.phase = if byte == b'"' { DrawingRetainedDecodePhase::Verb } else { DrawingRetainedDecodePhase::Fault };
            }
            DrawingRetainedDecodePhase::Verb => {
                if self.verb_cursor == self.expected_verb.len() {
                    self.phase = if byte == b'"' { DrawingRetainedDecodePhase::Comma } else { DrawingRetainedDecodePhase::Fault };
                } else if self.expected_verb.get(self.verb_cursor) == Some(&byte) {
                    self.verb_cursor += 1;
                } else {
                    self.phase = DrawingRetainedDecodePhase::Fault;
                }
            }
            DrawingRetainedDecodePhase::Comma => {
                if byte.is_ascii_whitespace() {
                    return;
                }
                self.phase = if byte == b',' { DrawingRetainedDecodePhase::Value } else { DrawingRetainedDecodePhase::Fault };
            }
            DrawingRetainedDecodePhase::Value => self.feed_value(byte),
            DrawingRetainedDecodePhase::Close => {
                if byte.is_ascii_whitespace() {
                    return;
                }
                self.phase = if byte == b']' { DrawingRetainedDecodePhase::Complete } else { DrawingRetainedDecodePhase::Fault };
            }
            DrawingRetainedDecodePhase::Complete | DrawingRetainedDecodePhase::Fault => {}
        }
    }

    fn feed_value(&mut self, byte: u8) {
        if self.value_complete {
            if byte.is_ascii_whitespace() {
                return;
            }
            self.phase = if byte == b']' { DrawingRetainedDecodePhase::Complete } else { DrawingRetainedDecodePhase::Fault };
            return;
        }
        if self.string {
            if self.escaped {
                self.escaped = false;
            } else if byte == b'\\' {
                self.escaped = true;
            } else if byte == b'"' {
                self.string = false;
                if self.nested_depth == 0 {
                    self.value_complete = true;
                }
            }
            return;
        }
        if byte.is_ascii_whitespace() && !self.scalar && self.nested_depth == 0 {
            return;
        }
        match byte {
            b'"' => self.string = true,
            b'{' | b'[' => self.nested_depth += 1,
            b'}' | b']' if self.nested_depth != 0 => {
                self.nested_depth -= 1;
                if self.nested_depth == 0 {
                    self.value_complete = true;
                }
            }
            b']' if self.scalar => self.phase = DrawingRetainedDecodePhase::Complete,
            b']' => self.phase = DrawingRetainedDecodePhase::Fault,
            byte if byte.is_ascii_whitespace() && self.scalar => {
                self.scalar = false;
                self.value_complete = true;
            }
            _ => self.scalar = true,
        }
    }

    fn finish(&mut self) -> bool {
        if self.phase == DrawingRetainedDecodePhase::Value && self.scalar {
            self.value_complete = true;
            self.phase = DrawingRetainedDecodePhase::Close;
        }
        self.phase == DrawingRetainedDecodePhase::Complete && self.verb_cursor == self.expected_verb.len() && self.nested_depth == 0 && !self.string && !self.escaped
    }
}

struct DrawingGestureOperationJob {
    payload: Option<DrawingGestureOperationPayload>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<semio_framework_plugin::EditorApp<DrawingPlayApp>>>,
    raw_input: Option<semio_framework::action_bus::RetainedToolWireInput>,
    raw_page_cursor: usize,
    raw_byte_cursor: usize,
    decoder: Option<DrawingRetainedCommandDecoder>,
    raw_validated: bool,
    completed: bool,
    closing: bool,
}

impl semio_framework_job::InteractiveJob for DrawingGestureOperationJob {
    fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if context.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if context.should_yield() || context.fuel_remaining() == 0 {
            return semio_framework_job::StepOutcome::Yield;
        }
        if !self.raw_validated {
            let Some(input) = self.raw_input.as_ref() else { return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }) };
            if let Some(page) = input.page(self.raw_page_cursor) {
                if let Some(byte) = page.get(self.raw_byte_cursor) {
                    let Some(decoder) = self.decoder.as_mut() else {
                        return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
                    };
                    decoder.feed(*byte);
                    if decoder.phase == DrawingRetainedDecodePhase::Fault {
                        return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
                    }
                    self.raw_byte_cursor += 1;
                    context.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                self.raw_page_cursor += 1;
                self.raw_byte_cursor = 0;
                return semio_framework_job::StepOutcome::Yield;
            }
            let exact = self.decoder.as_mut().is_some_and(DrawingRetainedCommandDecoder::finish);
            if !exact {
                return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
            }
            self.raw_validated = true;
            context.consume_fuel(1);
            return semio_framework_job::StepOutcome::Yield;
        }
        if self.pending_completion_rejection.is_some() {
            return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
        }
        if !self.completed {
            let Some(payload) = self.payload.as_ref() else { return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }) };
            let emit = payload.instance_owner.with_mut::<DrawingInstanceOperationOwner, _>(|owner| owner.dispatch(payload));
            let emit = match emit {
                Ok(Some(emit)) => Ok(emit),
                Ok(None) => {
                    context.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                Err(error) => Err(error),
            };
            if let Err(rejected) = payload.completion.complete(emit, semio_framework_plugin::EphemeralEmit::default()) {
                self.pending_completion_rejection = Some(rejected);
                return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
            }
            self.completed = true;
            context.consume_fuel(1);
        }
        semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        })
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(input) = self.raw_input.as_mut() {
            input.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            if let Ok(emit) = rejected.emit.as_mut() {
                if let Some(step) = emit.close_child_one(maximum_items, maximum_bytes) {
                    return match step {
                        semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                        semio_framework_plugin::PluginCloseStep::Blocked { .. } | semio_framework_plugin::PluginCloseStep::AwaitingInput { .. } => semio_framework_job::InteractiveJobCloseStep::Blocked,
                        semio_framework_plugin::PluginCloseStep::Complete => unreachable!("child close helper consumes completed children"),
                    };
                }
            }
            if maximum_items == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.pending_completion_rejection = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if let Some(input) = self.raw_input.as_mut() {
            let step = input.close_step(1, maximum_bytes);
            if input.terminal_is_empty() {
                self.raw_input = None;
            }
            return match step {
                semio_framework_job::InteractiveJobCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                other => other,
            };
        }
        if self.payload.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.decoder.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.pending_completion_rejection.is_none() && self.payload.is_none() && self.raw_input.is_none() && self.decoder.is_none()
    }
}

struct DrawingGestureOperationJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl DrawingGestureOperationJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: DRAWING_GESTURE_TOOL_IDS.iter().map(|tool| semio_framework::ToolFactoryKey::new(controller_id, *tool)).collect() }
    }
}

impl semio_framework::ToolJobFactory for DrawingGestureOperationJobFactory {
    type Payload = DrawingGestureOperationPayload;
    type Job = DrawingGestureOperationJob;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        DRAWING_DOCUMENT_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(DRAWING_GESTURE_RAW_BYTES, 32, 1, 16_384, 7_500, 1, 1)
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(DrawingGestureOperationJob { payload: Some(payload), pending_completion_rejection: None, raw_input: None, raw_page_cursor: 0, raw_byte_cursor: 0, decoder: None, raw_validated: true, completed: false, closing: false })
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if checkpoint.is_some() || input.declared_bytes() > DRAWING_GESTURE_RAW_BYTES {
            return Err((semio_framework::ToolJobFactoryError::new("Drawing gesture retained ingress rejects a checkpoint or oversized wire owner"), input, checkpoint));
        }
        let mut job = match self.create_job(operation, payload) {
            Ok(job) => job,
            Err(error) => return Err((error, input, None)),
        };
        let Some(expected_verb) = job.payload.as_ref().map(|payload| payload.command.command_id()) else {
            return Err((semio_framework::ToolJobFactoryError::new("Drawing gesture retained decoder has no exact typed command owner"), input, None));
        };
        job.raw_input = Some(input);
        job.decoder = Some(DrawingRetainedCommandDecoder::new(expected_verb));
        job.raw_validated = false;
        Ok(job)
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for DrawingGestureOperationJobFactory {
    type Owner = semio_framework_plugin::EditorApp<DrawingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = DRAWING_GESTURE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = DRAWING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = DRAWING_GESTURE_PUBLICATION_CONTRACTS;
}

#[cfg(test)]
#[path = "🧪️tests/🔬️gesture-operation-owner/🦀️.rs"]
mod gesture_operation_owner_tests;
//#endregion 🧵️GestureOperationJobs

//#region 🧵️BoundedCommands
/// 🧵️ Every non-gesture `DrawingCommand` row, each reducing in one bounded first step through
/// [`DrawingBoundedCommandJobFactory`]. Together with [`DRAWING_GESTURE_TOOL_IDS`] this covers the
/// command enum exactly (pinned by `retained_route_dispositions_are_exact_and_exhaustive`): an id
/// missing from both lists is unreachable from the client, because `validate_tool_job_rows` demands
/// one proof row per `Migrated` generated id and `validate_ui_dispatch_classification` rejects
/// anything that is not `Migrated`.
const DRAWING_BOUNDED_TOOL_IDS: &[&str] = &[
    "setSnapshot",
    "commitDocument",
    "setFixtureJson",
    "setActiveExample",
    "setSelectedOpacity",
    "engagementSubmit",
    "addLayer",
    "dropLayerKind",
    "moveLayer",
    "deleteLayer",
    "duplicateLayer",
    "toggleLayerVisible",
    "combineBoolean",
    "patchLayer",
    "patchLayers",
    "setCamera",
    "setCameraZoom",
    "engagementInput",
];
const DRAWING_BOUNDED_PAYLOAD_SCHEMA: &str = "drawing.tool-command.v1";
const DRAWING_BOUNDED_RAW_BYTES: usize = 65_536;
const DRAWING_BOUNDED_WORK_ITEMS: usize = 4_096;

/// 🛣️ One publication lane row per bounded route, read off each handler's real `Emit` construction
/// (`🎮️commands/*/🦀️.rs`). The four whole-document routes emit nothing but an `Effect::LoadDocument`
/// — effects are not a store lane, so those are honestly `HostOnly`; the layer routes emit
/// `DrawingMutation`s; the view routes emit `DrawingConfigMutation`s.
const DRAWING_BOUNDED_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSnapshot", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "commitDocument", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setFixtureJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSelectedOpacity", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "dropLayerKind", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "duplicateLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleLayerVisible", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "combineBoolean", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchLayers", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCameraZoom", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
];

fn drawing_bounded_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::bounded_first_step(DRAWING_BOUNDED_RAW_BYTES, DRAWING_BOUNDED_WORK_ITEMS, 1, 262_144, 7_500)
}

/// 📏️ One work item per bounded dispatch, admitted only while the live document still fits the
/// declared envelope — a gesture id or an oversized document answers `None`, which fails the route
/// closed instead of publishing an unbounded step.
fn drawing_bounded_extent(command: &DrawingCommand, snapshot: &DrawingSnapshot, _interaction: &::protocol::InteractionState) -> Option<usize> {
    if !DRAWING_BOUNDED_TOOL_IDS.contains(&command.command_id()) {
        return None;
    }
    let items = [snapshot.layers.len(), snapshot.assets.len()].into_iter().try_fold(1usize, |total, count| total.checked_add(count))?;
    (items <= DRAWING_BOUNDED_WORK_ITEMS).then_some(1)
}

/// 🔁️ The bounded reducer runs the SAME `DrawingCommand::dispatch` the ordinary `ArtifactEditor::handle`
/// route runs, seeded with the same framework-owned selection — migration is wiring, never a rewrite
/// of a command body.
#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
fn drawing_bounded_reduce(
    command: &DrawingCommand,
    snapshot: &DrawingSnapshot,
    config: &DrawingConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &::protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<DrawingPlayApp>>>,
    operation: &semio_framework_plugin::AppOperationContext,
) -> Result<Emit<DrawingMutation, DrawingConfigMutation, NoDraftMutation>, Fault> {
    if !DRAWING_BOUNDED_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.bounded.route"), "the bounded Drawing command owner rejects a gesture command"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: None };
    let active_utility = context.and_then(|context| context.view_state.as_ref()).and_then(|view| view.active_utility_id.as_deref()).unwrap_or(DRAWING_DEFAULT_UTILITY);
    let mut session = DrawingSession::with_active_utility(active_utility);
    session.interaction.ids = interaction.selection.get(DRAWING_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default();
    command.dispatch(&doc, &cfg, &mut session)
}

struct DrawingBoundedCommandJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl DrawingBoundedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: DRAWING_BOUNDED_TOOL_IDS.iter().map(|tool| semio_framework::ToolFactoryKey::new(controller_id, *tool)).collect() }
    }
}

impl semio_framework::ToolJobFactory for DrawingBoundedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<DrawingPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<DrawingPlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        DRAWING_BOUNDED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        drawing_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if checkpoint.is_some() || input.declared_bytes() > DRAWING_BOUNDED_RAW_BYTES {
            return Err((semio_framework::ToolJobFactoryError::new("bounded Drawing command ingress rejects a checkpoint or oversized wire owner"), input, checkpoint));
        }
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for DrawingBoundedCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<DrawingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = DRAWING_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = DRAWING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = DRAWING_BOUNDED_PUBLICATION_CONTRACTS;
}

/// 🏗️ The bounded half of `DrawingPlayApp::build_tool_job` — one `BoundedArtifactCommandWork` per
/// non-gesture route, refused outright when the id, the typed command and the live extent disagree.
fn drawing_bounded_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<DrawingPlayApp>>) -> Result<semio_framework::ToolOperationSpec, Fault> {
    if request.command.command_id() != request.tool_id || drawing_bounded_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.bounded.tool-mismatch"), "bounded Drawing command does not match its exact registered tool or exceeds its declared extent"));
    }
    let tool_id = request.command.command_id();
    let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<DrawingPlayApp>>> =
        Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, drawing_bounded_reduce, drawing_bounded_extent));
    let operation_context = semio_framework_plugin::AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id.clone(),
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
    };
    let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new(
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
        DrawingCommand::command_id,
        DRAWING_BOUNDED_RAW_BYTES,
        DRAWING_BOUNDED_WORK_ITEMS,
        work,
    )?;
    Ok(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation))
}
//#endregion 🧵️BoundedCommands

//#region 📬️StorePreparation
/// 🧬️ Builds the single described edit both preparation lanes publish — the two differ only in their
/// mutation type and id prefix.
fn drawing_prepared_edit<M>(prefix: &str, forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> ::protocol::Edit<M> {
    let id = format!("{prefix}-{}", authority.next_sequence_number());
    ::protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![::protocol::MutationMeta {
            mutation_id: Some(::protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(::protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: ::protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

struct DrawingArtifactStorePreparationFactory;

struct DrawingArtifactStorePreparation {
    base: Option<store::SnapshotRead<DrawingSnapshot>>,
    mutation: Option<DrawingMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<DrawingSnapshot, DrawingMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<DrawingSnapshot, DrawingMutation> for DrawingArtifactStorePreparationFactory {
    fn preflight(&self, _mutation: &DrawingMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("drawing-artifact-lane-or-description-envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<DrawingSnapshot, DrawingMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<DrawingSnapshot, DrawingMutation>>, store::ArtifactStoreOneItemPreparationRequest<DrawingSnapshot, DrawingMutation>> {
        let items = request.base.get().layers.len().saturating_add(request.base.get().assets.len());
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || items > DRAWING_BOUNDED_WORK_ITEMS
        {
            return Err(request);
        }
        Ok(Box::new(DrawingArtifactStorePreparation {
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

impl store::ArtifactStoreOneItemPreparation<DrawingSnapshot, DrawingMutation> for DrawingArtifactStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use ::protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "drawing-artifact-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "drawing-artifact-mutation-owner-missing".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = ::protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "drawing-artifact-authority-missing".to_string())?;
        let edit = drawing_prepared_edit("drawing-artifact-bounded", mutation, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<DrawingSnapshot, DrawingMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<DrawingSnapshot, DrawingMutation>> {
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
                return Err("drawing-artifact-base-retirement-rejected".into());
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

struct DrawingConfigStorePreparationFactory;

struct DrawingConfigStorePreparation {
    base: Option<store::SnapshotRead<DrawingConfig>>,
    mutation: Option<DrawingConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<DrawingConfig, DrawingConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<DrawingConfig, DrawingConfigMutation> for DrawingConfigStorePreparationFactory {
    fn preflight(&self, _mutation: &DrawingConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("drawing-config-lane-or-description-envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<DrawingConfig, DrawingConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<DrawingConfig, DrawingConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<DrawingConfig, DrawingConfigMutation>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(DrawingConfigStorePreparation {
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

impl store::ArtifactStoreOneItemPreparation<DrawingConfig, DrawingConfigMutation> for DrawingConfigStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use ::protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "drawing-config-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "drawing-config-mutation-owner-missing".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = ::protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "drawing-config-authority-missing".to_string())?;
        let edit = drawing_prepared_edit("drawing-config-bounded", mutation, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<DrawingConfig, DrawingConfigMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<DrawingConfig, DrawingConfigMutation>> {
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
                return Err("drawing-config-base-retirement-rejected".into());
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

//#region 🧾️ProofCatalogs
/// 🧾️ One `bounded_first_step_tool_proofs!` invocation per owned factory — the macro emits the whole
/// `bounded_first_step_tool_proofs()` body for a single factory, so an app with two factories declares
/// two catalogs and concatenates them in its `ArtifactEditor` item (process3d/flow precedent).
struct DrawingGestureProofs;
impl DrawingGestureProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        document_schema: "drawing.document",
        factory: "DrawingGestureOperationJobFactory",
        factory_type: DrawingGestureOperationJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(8_192, 32, 1, 16_384, 7_500, 1, 1),
        tools: ["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "canvasCommitDraft", "canvasEscape"]
    }
}

struct DrawingBoundedProofs;
impl DrawingBoundedProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        document_schema: "drawing.document",
        factory: "DrawingBoundedCommandJobFactory",
        factory_type: DrawingBoundedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "setSnapshot", "commitDocument", "setFixtureJson", "setActiveExample", "setSelectedOpacity", "engagementSubmit",
            "addLayer", "dropLayerKind", "moveLayer", "deleteLayer", "duplicateLayer", "toggleLayerVisible", "combineBoolean",
            "patchLayer", "patchLayers", "setCamera", "setCameraZoom", "engagementInput",         ]
    }
}
//#endregion 🧾️ProofCatalogs

//#region 🔖️DrawingPlayApp
pub(crate) fn drawing_document_revision(doc: &ArtifactView<'_, DrawingSnapshot>, _config: &DrawingConfig) -> String {
    doc.operation_optional().map_or_else(|| "0".repeat(64), |operation| operation.canonical_base_revision_hex())
}

/// 🧪️ Unit struct apart from `session`: every former `DrawingInteractionState`/`ViewModel`-derived field
/// lives in [`DrawingConfig`], written through [`DrawingConfigMutation`]s. `session` holds the one piece of
/// state that is neither document nor view-config — the live gesture statechart — threaded into every
/// command handler as the `app_commands!` dispatch context.
pub struct DrawingPlayApp {
    arena_boot_fault: Option<&'static str>,
}

impl DrawingPlayApp {
    pub fn arena_boot_fault(&self) -> Option<&'static str> {
        self.arena_boot_fault.or_else(crate::spr::drawing_mutation_arena_pool_fault)
    }
}

fn render_drawing_body(
    body_key: &str,
    document: &DrawingSnapshot,
    config: &DrawingConfig,
    preview: &DrawingGesturePreview,
    view_state: &semio_framework_plugin::ViewModel,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let labels = semio_framework_plugin::resolve_labels::<DrawingPlayLabels>(view_state);
    let active_utility = view_state.active_utility_id.as_deref().unwrap_or(DRAWING_DEFAULT_UTILITY);
    let root = match body_key {
        DRAWING_PLAY_BODY_COMPOSITE => canvas_window::render(document, config, preview, active_utility),
        DRAWING_PLAY_BODY_LAYERS => layers_panel::render(document, labels),
        DRAWING_PLAY_BODY_CATALOGUE => catalogue_panel::render(document, labels),
        DRAWING_PLAY_BODY_PROPERTIES => properties_panel::render(document, active_utility),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("drawing.body.label", "the fixed Drawing unknown-body label exceeds its UI bound")),
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(root))
}

impl Default for DrawingPlayApp {
    fn default() -> Self {
        let arena_boot_fault = match crate::spr::request_drawing_mutation_arena_pool() {
            crate::spr::DrawingMutationArenaPoolAvailability::Fault(error) => Some(error),
            crate::spr::DrawingMutationArenaPoolAvailability::Ready | crate::spr::DrawingMutationArenaPoolAvailability::NotReady | crate::spr::DrawingMutationArenaPoolAvailability::Contended => None,
        };
        Self { arena_boot_fault }
    }
}

impl ArtifactEditor for DrawingPlayApp {
    type Snapshot = DrawingSnapshot;
    type Mutation = DrawingMutation;
    type Config = DrawingConfig;
    type ConfigMutation = DrawingConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = DrawingPresence;
    type PresenceMutation = DrawingPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = DrawingCommand;

    const DIALECT: semio_framework::Dialect = crate::DRAWING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DRAWING_DOCUMENT_SCHEMA;

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::spr::drawing_envelope_decode_owner_bundle())
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::spr::drawing_document_store_owners())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::spr::drawing_document_store_initialization_job(envelope, operation, generation))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(DrawingArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(DrawingConfigStorePreparationFactory))
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        DrawingGestureProofs::bounded_first_step_tool_proofs().into_iter().chain(DrawingBoundedProofs::bounded_first_step_tool_proofs()).collect()
    }

    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(DrawingInstanceOperationOwner::new())
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(DrawingGestureOperationJobFactory::new(&controller))?;
        registry.register(DrawingBoundedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if DRAWING_BOUNDED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return drawing_bounded_tool_job(request).map(Some);
        }
        if !DRAWING_GESTURE_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.tool-mismatch"), "Drawing gesture command does not match its exact registered tool"));
        }
        let operation_context = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = DrawingGestureOperationPayload {
            command: *request.command,
            snapshot: request.snapshot,
            config: request.config,
            history: request.history,
            instance_owner: request.instance_operation_owner,
            operation_context,
            active_utility_id: request.context.view_state.as_ref().and_then(|view| view.active_utility_id.clone()).unwrap_or_else(|| DRAWING_DEFAULT_UTILITY.into()),
            completion: request.completion,
        };
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::drawing::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> DrawingSnapshot {
        crate::schema::default_drawing_document("empty", None)
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(drawing_io())
    }

    /// 🎞️ `vector:out` (see `drawing_vector_media`) plus the inherited `document:out` default (the pack
    /// of `doc.snapshot`, replicated inline — overriding `export_media` shadows the trait's provided
    /// body for every port on this app, not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, DrawingSnapshot>) -> Result<Media, MediaError> {
        match port {
            "vector:out" => drawing_vector_media(doc.snapshot),
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    // 🖼️ No override: whole-document replacement has no `Mutation` vehicle any more (banned
    // vocabulary — see `🧬️mutations/🦀️.rs`'s module doc). The default `None` disables the
    // generic `import_media("document:in")` port for drawing; explicit whole-document load/replace
    // stays reachable through the `set_snapshot`/`commit_document`/`set_fixture_json`/
    // `set_active_example` commands, which now emit `Effect::LoadDocument` (the sanctioned
    // non-history reset path) instead.

    /// 🏷️ `app_commands!`'s generated `command_id()`.
    fn command_id(command: &DrawingCommand) -> &'static str {
        command.command_id()
    }

    fn handle(
        command: &DrawingCommand,
        doc: &ArtifactView<'_, DrawingSnapshot>,
        cfg: &ConfigView<'_, DrawingConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<DrawingMutation, DrawingConfigMutation, Self::DraftMutation>, Fault> {
        if DRAWING_GESTURE_TOOL_IDS.contains(&command.command_id()) {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.retained-route"), "Drawing gesture commands are reachable only through their exact retained factory owner"));
        }
        let mut session = DrawingSession::with_active_utility(view_state.and_then(|view| view.active_utility_id.as_deref()).unwrap_or(DRAWING_DEFAULT_UTILITY));
        session.interaction.ids = interaction.selection(DRAWING_INTERACTION_DOMAIN).ids.clone();
        command.dispatch(doc, cfg, &mut session)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, DrawingConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_drawing_body(body_key, doc.snapshot, cfg.snapshot, &DrawingSession::default().preview(), view_state)
    }

    fn render_with_instance_operation_owner(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, DrawingSnapshot>,
        cfg: &ConfigView<'_, DrawingConfig>,
        view_state: &semio_framework_plugin::ViewModel,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let preview = match doc.render_operation() {
            Some(operation) => owner
                .with_mut::<DrawingInstanceOperationOwner, _>(|owner| Ok(owner.preview_projection(operation.canonical_base_revision, view_state.active_utility_id.as_deref().unwrap_or(DRAWING_DEFAULT_UTILITY))))
                .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("drawing.gesture.preview-owner", error.message))?
                .unwrap_or_default(),
            None => DrawingGesturePreview::default(),
        };
        render_drawing_body(body_key, doc.snapshot, cfg.snapshot, &preview, view_state)
    }
}
//#endregion 🔖️DrawingPlayApp

//#region 🔖️Io
/// 🌱️ Builds the single canonical non-history document-reset effect for Drawing.
pub(crate) fn drawing_reset_document_effect(scene: &DrawingSnapshot) -> semio_framework_plugin::Effect {
    let pack = <DrawingSnapshot as ArtifactPack>::encode_pack(scene);
    let envelope = store::create_document_envelope::<DrawingSnapshot, DrawingMutation>(DRAWING_DOCUMENT_SCHEMA, &scene.id, scene.clone(), None);
    let spr = semio_framework_plugin::resolve_ready(store::print_document_spr(&envelope)).expect("drawing document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}

/// 🔌️ Relocated verbatim from the `⚙️engine` directory (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 4: anything returning `AppIo` or
/// referencing an app type lives in `🎛️apps/<app>/`). This app's typed media I/O surface
/// (`AppDefinition.io`) — mirrors the `2d.drawing` `ArtifactKindSpec` literal `create_drawing_app`
/// already declares via `.artifact_kind(...)` (schema/media type/export+import formats copied
/// verbatim), plus the app-specific `vector:out` port (see `drawing_vector_out_port` below).
pub fn drawing_io() -> semio_framework::AppIo {
    semio_framework::AppIo {
        document_schema: DRAWING_DOCUMENT_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        ports: vec![drawing_vector_out_port()],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework::ArtifactPresentation { id: "2d.drawing".into(), name: "2D Drawing".into(), dimension: "2d".into(), component_kind: "drawing".into() },
    }
}

/// 🔌️ `vector:out` — the drawing document's current vector content, exported as SVG (workflow port
/// surface; WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe). Reuses the existing `2d.drawing`
/// kind (already declared by `create_drawing_app`'s `.artifact_kind(...)`) rather than minting a
/// duplicate — `kind_id` just pins this port to that same catalog entry. `Many`/optional: a
/// consumer (e.g. raster's Vector→Raster-converted `image:in`) may connect before the canvas has
/// any content, or fan out to several consumers at once.
pub fn drawing_vector_out_port() -> semio_framework::MediaPortSpec {
    semio_framework::MediaPortSpec {
        id: "vector:out".into(),
        label: "Vector".into(),
        direction: semio_framework::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        kind_id: Some("2d.drawing".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🖼️ Exports the current drawing document as an SVG `Media` payload for the `vector:out` port —
/// reuses `crate::io::drawing_document_to_svg` (the same semio/drawing↔svg bridge the
/// export-svg shell path uses), so there is exactly one SVG renderer.
pub fn drawing_vector_media(doc: &DrawingSnapshot) -> Result<Media, MediaError> {
    let (svg, _width, _height) = crate::io::drawing_document_to_svg(doc).map_err(|error| MediaError::Payload("vector:out".into(), error))?;
    Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "2d.drawing".into(), json: svg } })
}
//#endregion 🔖️Io

//#region 🔖️Manifest
pub fn create_drawing_app() -> semio_framework_plugin::AppDefinition {
    let engagement = WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("drawing-canvas-engagement".into()),
            value: Some(String::new()),
            placeholder: Some("Layer name".into()),
            on_change: Some(drawing_manifest_action("engagementInput")),
            on_submit: Some(drawing_manifest_action("engagementSubmit")),
            disabled: None,
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "drawing-layer-count".into(), text: "0 layers · 0 selected".into() }]),
        possible_engagements: None,
    };
    Editor::builder(crate::DRAWING_DIALECT).document(["semio", "drawing"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("drawing")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind_with_engagement(DRAWING_PLAY_WINDOW_CANVAS, LocalizedLabel::native("Canvas", "Leinwand"), DRAWING_PLAY_BODY_COMPOSITE, semio_framework_ui_contract::SurfaceKind::Canvas2d, engagement, "pen-tool")
            .panel_tab_def(layers_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(properties_panel::definition())
            // ✏️ Palette-visible content operations.
            .mutation("addLayer", LocalizedLabel::native("Add Layer", "Ebene hinzufügen"))
            .action_interactive_job("addLayer", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .mutation("combineBoolean", LocalizedLabel::native("Combine Boolean", "Boolean kombinieren"))
            .action_interactive_job("combineBoolean", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::Migrated)
            // 🔧️ Internal content operations — inspector/layer-panel/import-bound, not palette commands.
            .action_with(drawing_internal_action("setSnapshot", LocalizedLabel::native("Set Document", "Dokument festlegen"), ActionKind::Mutation))
            .action_interactive_job("setSnapshot", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("commitDocument", LocalizedLabel::native("Commit Document", "Dokument übernehmen"), ActionKind::Mutation))
            .action_interactive_job("commitDocument", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("setFixtureJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Mutation))
            .action_interactive_job("setFixtureJson", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("setSelectedOpacity", LocalizedLabel::native("Set Selected Opacity", "Deckkraft der Auswahl festlegen"), ActionKind::Mutation))
            .action_interactive_job("setSelectedOpacity", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Mutation))
            .action_interactive_job("engagementSubmit", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("dropLayerKind", LocalizedLabel::native("Drop Layer Kind", "Ebenenart ablegen"), ActionKind::Mutation))
            .action_interactive_job("dropLayerKind", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("moveLayer", LocalizedLabel::native("Move Layer", "Ebene verschieben"), ActionKind::Mutation))
            .action_interactive_job("moveLayer", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("deleteLayer", LocalizedLabel::native("Delete Layer", "Ebene löschen"), ActionKind::Mutation))
            .action_interactive_job("deleteLayer", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("duplicateLayer", LocalizedLabel::native("Duplicate Layer", "Ebene duplizieren"), ActionKind::Mutation))
            .action_interactive_job("duplicateLayer", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("toggleLayerVisible", LocalizedLabel::native("Toggle Layer Visible", "Ebenensichtbarkeit umschalten"), ActionKind::Mutation))
            .action_interactive_job("toggleLayerVisible", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("patchLayer", LocalizedLabel::native("Patch Layer", "Ebene aktualisieren"), ActionKind::Mutation))
            .action_interactive_job("patchLayer", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("patchLayers", LocalizedLabel::native("Patch Layers", "Ebenen aktualisieren"), ActionKind::Mutation))
            .action_interactive_job("patchLayers", semio_framework_plugin::InteractiveJobClassification::Migrated)
            // 🖱️ Internal pointer/gesture vocabulary — commit-time handlers emit operations, the rest are pure View.
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), ActionKind::Mutation, "mouse-pointer") })
            .action_interactive_job("canvasPointerDown", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), ActionKind::Mutation, "mouse-pointer") })
            .action_interactive_job("canvasPointerUp", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("canvasDoubleClick", LocalizedLabel::native("Canvas Double Click", "Leinwand-Doppelklick"), ActionKind::Mutation))
            .action_interactive_job("canvasDoubleClick", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("canvasCommitDraft", LocalizedLabel::native("Canvas Commit Draft", "Leinwand-Entwurf übernehmen"), ActionKind::Mutation))
            .action_interactive_job("canvasCommitDraft", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegen"), ActionKind::View, "mouse-pointer") })
            .action_interactive_job("canvasPointerMove", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("canvasEscape", LocalizedLabel::native("Canvas Escape", "Leinwand abbrechen"), ActionKind::View))
            .action_interactive_job("canvasEscape", semio_framework_plugin::InteractiveJobClassification::Migrated)
            // 👁️ Ephemeral view state — selection/hover are framework-owned now (see `.interaction(...)`
            // below): interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
            // setInteractionGranularity auto-inject, never declared here (ticket
            // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand") })
            .action_interactive_job("engagementInput", semio_framework_plugin::InteractiveJobClassification::Migrated)
            // 📷️ Camera — session-only runtime pose, never a document operation.
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera") })
            .action_interactive_job("setCamera", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(drawing_internal_action("setCameraZoom", LocalizedLabel::native("Set Camera Zoom", "Kamerazoom festlegen"), ActionKind::View))
            .action_interactive_job("setCameraZoom", semio_framework_plugin::InteractiveJobClassification::Migrated)
            // 🧰️ Canvas utilities — one exclusive set per window, active utility host-owned (never a document operation).
            .utility(drawing_utility("selectMarquee", LocalizedLabel::native("Marquee Select", "Rahmenauswahl"), "square-dashed", "Select", UtilityCategory::Selection))
            .utility(drawing_utility("selectLasso", LocalizedLabel::native("Lasso Select", "Lasso-Auswahl"), "lasso", "Select", UtilityCategory::Selection))
            .utility(drawing_utility("selectDirect", LocalizedLabel::native("Direct Select", "Direktauswahl"), "mouse-pointer-2", "Select", UtilityCategory::Selection))
            .utility(drawing_utility("pen", LocalizedLabel::native("Pen", "Stift"), "pen-tool", "Drawing", UtilityCategory::Utilities))
            .utility(drawing_utility("shapeRect", LocalizedLabel::native("Rectangle", "Rechteck"), "rectangle-tool", "Drawing", UtilityCategory::Utilities))
            .utility(drawing_utility("shapeEllipse", LocalizedLabel::native("Ellipse", "Ellipse"), "circle", "Drawing", UtilityCategory::Utilities))
            .utility(drawing_utility("shapeLine", LocalizedLabel::native("Line", "Linie"), "minus", "Drawing", UtilityCategory::Utilities))
            .utility(drawing_utility("shapePolygon", LocalizedLabel::native("Polygon", "Polygon"), "hexagon", "Drawing", UtilityCategory::Utilities))
            .utility(drawing_utility("booleanCombine", LocalizedLabel::native("Boolean", "Boolean"), "combine", "Combine", UtilityCategory::Utilities))
            .utility(drawing_utility("trace", LocalizedLabel::native("Trace", "Nachzeichnen"), "scan-line", "Combine", UtilityCategory::Utilities))
            .utility(drawing_utility("transformMove", LocalizedLabel::native("Pan", "Verschieben"), "move", "View", UtilityCategory::Utilities))
            .window_kind_utilities(DRAWING_PLAY_WINDOW_CANVAS, vec![
                "selectMarquee".into(), "selectLasso".into(), "selectDirect".into(),
                "pen".into(), "shapeRect".into(), "shapeEllipse".into(), "shapeLine".into(), "shapePolygon".into(),
                "booleanCombine".into(), "trace".into(), "transformMove".into(),
            ])
            // 🕹️ The framework-owned "strokes" interaction domain (ticket
            // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — covers both the layers panel tree
            // (`.interaction_domain("strokes")?`) and the canvas's pick/marquee/lasso layer selection;
            // auto-injects interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
            // setInteractionGranularity, replacing every deleted bespoke setSelection/setHover/
            // clearSelection/selectAll action.
            .interaction(InteractionDefinition {
                id: DRAWING_INTERACTION_DOMAIN.into(),
                label: LocalizedLabel::native("Strokes", "Striche"),
                granularities: vec![GranularityDefinition { id: DRAWING_INTERACTION_GRANULARITY.into(), label: LocalizedLabel::native("Stroke", "Strich"), icon_id: "pen-tool".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(DRAWING_PLAY_WINDOW_CANVAS, vec![InteractionRef::new(DRAWING_INTERACTION_DOMAIN)])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("escape", "canvasEscape")
            .keybinding("enter", "canvasCommitDraft")
            .default_layout(edit::layout())
            // 📚️ Examples are declared once, on the subset (`🪆️subsets/✳️any/🦀️.rs`'s `examples()`,
            // reached by the shell through `SubsetDeclaration.examples`), never a second time on this
            // builder — `setActiveExample` resolves its `example_id` against that same slice, so the
            // switcher's ids and the command's ids cannot drift apart.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region.
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
