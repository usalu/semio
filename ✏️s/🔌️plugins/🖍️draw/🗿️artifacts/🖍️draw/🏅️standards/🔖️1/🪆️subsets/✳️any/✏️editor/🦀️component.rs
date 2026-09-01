//! 🖥️ Draw editor surface — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum
//! and the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window
//! render in `🎭️modes/✏️edit/🪟️windows/🖼️canvas`, panel trees in `📌️panels/*`, labels in
//! `🦀️terminology.rs`, view state in `🦀️config.rs`.
//! This file is a routing table: `handle` → `DrawCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawSnapshot, DRAW_DOCUMENT_SCHEMA};
use crate::editor::draw::commands::canvas_pointer_down::{DrawGesturePreview, DrawSession};
use crate::editor::draw::commands::{
    add_layer, canvas_commit_draft, canvas_double_click, canvas_escape, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, combine_boolean, commit_document, delete_layer, drop_layer_kind, duplicate_layer, engagement_input,
    engagement_submit, move_layer, patch_layer, patch_layers, set_active_example, set_active_utility, set_camera, set_camera_zoom, set_fixture_json, set_locale, set_selected_opacity, set_snapshot, toggle_layer_visible,
};
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use crate::editor::draw::modes::edit;
use crate::editor::draw::modes::edit::windows::canvas as canvas_window;
use crate::editor::draw::panels::{catalogue as catalogue_panel, layers as layers_panel, properties as properties_panel};
use crate::editor::draw::presence::{DrawPresence, DrawPresenceMutation};
use crate::editor::draw::terminology::DrawPlayLabels;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ActionDescriptor, ActionKind, ArtifactEditor, ArtifactView, ConfigView, DraftView, Editor, Emit, Fault, FaultCode, FaultOrigin, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label, LocalizedLabel,
    Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, UtilityCategory, UtilityDefinition, WindowEngagement, WindowEngagementInput,
    WindowEngagementStatus,
};
use serde_json::Value;
use semio_framework_job::FixedOperationOwner;
use store::ArtifactPack;
use store::EngineHandles;

pub use canvas_window::{DRAW_PLAY_BODY_COMPOSITE, DRAW_PLAY_WINDOW_CANVAS};
pub use catalogue_panel::DRAW_PLAY_BODY_CATALOGUE;
pub use layers_panel::{DRAW_LAYER_KIND_DRAG_MIME, DRAW_PLAY_BODY_LAYERS};
pub use properties_panel::DRAW_PLAY_BODY_PROPERTIES;

//#region 🔖️Constants
pub const DRAW_PLAY_CONTROLLER_ID: &str = "draw-play";
/// 🧰️ The utility the canvas returns to after committing a shape/draft/trace (first UtilityRef default).
pub const DRAW_DEFAULT_UTILITY: &str = "selectDirect";
pub const DRAW_PLAY_EXAMPLE_DEFAULT_ID: &str = "semio";
/// 🕹️ The single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM interaction domain this app declares
/// (granularity `stroke`, `HierarchyProvider::Flat`, methods Pick/Rectangle/Lasso).
pub const DRAW_INTERACTION_DOMAIN: &str = "strokes";
pub const DRAW_INTERACTION_GRANULARITY: &str = "stroke";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn draw_play_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(DRAW_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🎛️ Builds one manifest-side engagement action without crossing into the retained UI wire action type.
fn draw_manifest_action(action: &str) -> ActionDescriptor {
    ActionDescriptor { controller_id: DRAW_PLAY_CONTROLLER_ID.into(), action: action.into(), args: None }
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


/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector-bound vocabulary
/// that is dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn draw_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> semio_framework_plugin::ActionDefinition {
    semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog(id, label, kind) }
}

/// 🧰️ One canvas utility declaration (id/label/icon reused verbatim from the retired `utilities()` impl).
fn draw_utility(id: &str, label: impl Into<LocalizedLabel>, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `DrawPlayApp::Command` — the SOLE dispatch surface for draw's own behavior, covering every
    /// action `create_draw_app` declares. Field shapes mirror each action's real `args` object.
    /// **Row order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum DrawCommand for DrawSnapshot, DrawMutation, DrawConfig, DrawConfigMutation, ctx = DrawSession {
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
        "setActiveUtility" as "active-utility" => set_active_utility::SetActiveUtility,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setCameraZoom" as "camera-zoom" => set_camera_zoom::SetCameraZoom,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "setLocale" as "locale" => set_locale::SetLocale,
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
const DRAW_GESTURE_TOOL_IDS: &[&str] = &["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "canvasCommitDraft", "canvasEscape"];
const DRAW_GESTURE_RAW_BYTES: usize = 8_192;
const DRAW_GESTURE_RETAINED_BYTES: usize = 32_768;

struct DrawGestureOperationOwner {
    session: Option<DrawSession>,
    closing: bool,
}

impl DrawGestureOperationOwner {
    fn new() -> Self {
        Self { session: Some(DrawSession::default()), closing: false }
    }
}

impl semio_framework_job::FixedOperationOwner for DrawGestureOperationOwner {
    fn retained_bytes(&self) -> usize {
        DRAW_GESTURE_RETAINED_BYTES
    }

    fn cancel(&mut self) {
        self.closing = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 || maximum_bytes < DRAW_GESTURE_RETAINED_BYTES {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if self.session.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: DRAW_GESTURE_RETAINED_BYTES };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.session.is_none()
    }
}

struct DrawInstanceOperationOwner {
    operations: semio_framework_job::FixedOperationRegistry<DrawGestureOperationOwner, 64>,
    active: Option<(semio_framework_job::FixedOperationKey, [u8; 32])>,
    closing: bool,
}

impl DrawInstanceOperationOwner {
    fn new() -> Self {
        Self { operations: semio_framework_job::FixedOperationRegistry::new(64 * DRAW_GESTURE_RETAINED_BYTES), active: None, closing: false }
    }

    fn dispatch(
        &mut self,
        key: semio_framework_job::FixedOperationKey,
        base_revision: [u8; 32],
        command: &DrawCommand,
        snapshot: &DrawSnapshot,
        config: &DrawConfig,
        history: &semio_framework_plugin::HistoryView,
        operation: semio_framework_plugin::AppOperationContext,
    ) -> Result<Option<Emit<DrawMutation, DrawConfigMutation, NoDraftMutation>>, Fault> {
        if self.closing {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("draw.gesture.closing"), "the Draw gesture operation owner is closing"));
        }
        if let Some((active, observed_revision)) = self.active {
            if observed_revision != base_revision {
                self.operations.cancel(active);
                self.active = None;
            }
        }
        let live_key = self.active.map_or(key, |(active, _)| active);
        if self.operations.get(live_key).is_none() {
            self.operations.admit(live_key, DrawGestureOperationOwner::new()).map_err(|mut rejected| {
                rejected.owner.cancel();
                rejected.owner.begin_close();
                let _ = rejected.owner.close_step(1, DRAW_GESTURE_RETAINED_BYTES);
                Fault::new(FaultOrigin::App, FaultCode::new("draw.gesture.saturated"), "the fixed Draw gesture operation authority is saturated")
            })?;
            self.active = Some((live_key, base_revision));
        }
        let retained = self
            .operations
            .get_mut(live_key)
            .ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("draw.gesture.owner"), "the exact Draw gesture owner changed before its bounded reducer step"))?;
        let session = retained.session.as_mut().ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("draw.gesture.owner"), "the Draw gesture session is already closing"))?;
        if session.gesture.context.points_overflowed {
            self.operations.cancel(live_key);
            self.active = None;
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("draw.gesture.point-capacity"), "the fixed Draw gesture point capacity was exceeded"));
        }
        if let Some(query) = session.draft_query.as_mut() {
            if query.command_id != command.command_id() {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("draw.gesture.draft-owner"), "a retained Draw draft query rejects a different command owner"));
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
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("draw.gesture.query-owner"), "a retained Draw point query rejects a different command owner"));
            }
            if !query.traversal_complete {
                if !query.cursor.advance(snapshot) {
                    return Ok(None);
                }
                if query.cursor.overflowed {
                    session.point_query = None;
                    self.operations.cancel(live_key);
                    self.active = None;
                    return Err(Fault::new(FaultOrigin::App, FaultCode::new("draw.gesture.query-capacity"), "the fixed Draw query result capacity was exceeded"));
                }
                query.traversal_complete = true;
                return Ok(None);
            }
            let targets = match query.publication_step() {
                canvas_pointer_down::DrawQueryPublication::Pending => return Ok(None),
                canvas_pointer_down::DrawQueryPublication::Complete(targets) => targets,
                canvas_pointer_down::DrawQueryPublication::Fault => {
                    session.point_query = None;
                    self.operations.cancel(live_key);
                    self.active = None;
                    return Err(Fault::new(FaultOrigin::App, FaultCode::new("draw.gesture.query-output-capacity"), "the fixed Draw interaction output capacity was exceeded"));
                }
            };
            let query = session.point_query.take().expect("the exact published query remains retained");
            let effect = if query.hover {
                canvas_pointer_down::interaction_hover_effect_from_targets(targets)
            } else {
                canvas_pointer_down::interaction_select_effect_from_targets(targets, &query.merge)
            };
            let mut emit = Emit::default();
            emit.effects.push(effect);
            if session.gesture.matches("idle") && session.trace_pointer.is_none() {
                self.operations.cancel(live_key);
                self.active = None;
            }
            return Ok(Some(emit));
        }
        if let DrawCommand::CanvasPointerMove(payload) = command {
            if session.gesture.matches("idle") {
                let (world_x, world_y) = canvas_pointer_down::canvas_point_to_world(&config.camera, payload.x, payload.y, payload.width, payload.height);
                let tolerance = canvas_pointer_down::DRAW_PICK_TOLERANCE_PX / config.camera.zoom.max(1e-6);
                session.point_query = Some(canvas_pointer_down::DrawPointQuery::new(
                    command.command_id(),
                    canvas_pointer_down::TracePointerJob::new_query(snapshot, [world_x, world_y], tolerance, config.active_utility_id == "selectDirect"),
                    true,
                    "replace".into(),
                    false,
                ));
                return Ok(None);
            }
        }
        let retained_emit = match command {
            DrawCommand::CanvasPointerUp(payload) => {
                let (world_x, world_y) = canvas_pointer_down::canvas_point_to_world(&config.camera, payload.x, payload.y, payload.width, payload.height);
                Some(session.step_gesture_retained(
                    command.command_id(),
                    canvas_pointer_down::draw_gesture::Event::PointerUp {
                        utility: config.active_utility_id.clone(),
                        world: [world_x, world_y],
                        shift: payload.shift,
                        ctrl: payload.ctrl,
                        meta: payload.meta,
                    },
                    snapshot,
                    config,
                ))
            }
            DrawCommand::CanvasDoubleClick(_) | DrawCommand::CanvasCommitDraft(_) => Some(session.step_gesture_retained(command.command_id(), canvas_pointer_down::draw_gesture::Event::CommitDraft, snapshot, config)),
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
        let cfg = ConfigView { snapshot: config };
        let emit = match command {
            DrawCommand::CanvasPointerDown(payload) => canvas_pointer_down::handle(payload, &doc, &cfg, session),
            DrawCommand::CanvasPointerMove(payload) => canvas_pointer_move::handle(payload, &doc, &cfg, session),
            DrawCommand::CanvasPointerUp(_) | DrawCommand::CanvasDoubleClick(_) | DrawCommand::CanvasCommitDraft(_) => unreachable!("retained Draw gesture commands returned above"),
            DrawCommand::CanvasEscape(payload) => canvas_escape::handle(payload, &doc, &cfg, session),
            _ => Err(Fault::new(FaultOrigin::App, FaultCode::new("draw.gesture.command"), "the retained Draw gesture owner rejects non-gesture commands")),
        }?;
        if session.gesture.context.points_overflowed {
            self.operations.cancel(live_key);
            self.active = None;
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("draw.gesture.point-capacity"), "the fixed Draw gesture point capacity was exceeded"));
        }
        if session.gesture.matches("idle") && session.trace_pointer.is_none() {
            self.operations.cancel(live_key);
            self.active = None;
        }
        Ok(Some(emit))
    }

    fn preview_projection(&mut self, canonical_base_revision: [u8; 32], active_utility: &str) -> Option<DrawGesturePreview> {
        let (key, observed_revision) = self.active?;
        if observed_revision != canonical_base_revision {
            self.operations.cancel(key);
            self.active = None;
            return None;
        }
        let session = self.operations.get_mut(key).and_then(|owner| owner.session.as_mut())?;
        if active_utility != "trace" {
            session.trace_pointer = None;
        }
        if session.gesture.matches("idle") && session.trace_pointer.is_none() && session.point_query.is_none() && session.draft_query.is_none() {
            self.operations.cancel(key);
            self.active = None;
            return None;
        }
        Some(session.preview())
    }
}

impl semio_framework_plugin::ArtifactInstanceOperationOwner for DrawInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        Ok(match self.operations.close_step(maximum_items, maximum_bytes) {
            semio_framework_job::InteractiveJobCloseStep::Blocked => semio_framework_plugin::PluginCloseStep::Blocked { reason: "Draw gesture close owner awaits its exact grant" },
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

struct DrawGestureOperationPayload {
    command: DrawCommand,
    snapshot: std::sync::Arc<DrawSnapshot>,
    config: std::sync::Arc<DrawConfig>,
    history: std::sync::Arc<semio_framework_plugin::HistoryView>,
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    operation_context: semio_framework_plugin::AppOperationContext,
    completion: semio_framework_plugin::ArtifactToolCompletion<semio_framework_plugin::EditorApp<DrawPlayApp>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DrawRetainedDecodePhase {
    Open,
    VerbOpen,
    Verb,
    Comma,
    Value,
    Close,
    Complete,
    Fault,
}

struct DrawRetainedCommandDecoder {
    expected_verb: &'static [u8],
    phase: DrawRetainedDecodePhase,
    verb_cursor: usize,
    nested_depth: usize,
    string: bool,
    escaped: bool,
    scalar: bool,
    value_complete: bool,
}

impl DrawRetainedCommandDecoder {
    fn new(expected_verb: &'static str) -> Self {
        Self { expected_verb: expected_verb.as_bytes(), phase: DrawRetainedDecodePhase::Open, verb_cursor: 0, nested_depth: 0, string: false, escaped: false, scalar: false, value_complete: false }
    }

    fn feed(&mut self, byte: u8) {
        if matches!(self.phase, DrawRetainedDecodePhase::Fault | DrawRetainedDecodePhase::Complete) {
            if !byte.is_ascii_whitespace() {
                self.phase = DrawRetainedDecodePhase::Fault;
            }
            return;
        }
        match self.phase {
            DrawRetainedDecodePhase::Open => {
                if byte.is_ascii_whitespace() {
                    return;
                }
                self.phase = if byte == b'[' { DrawRetainedDecodePhase::VerbOpen } else { DrawRetainedDecodePhase::Fault };
            }
            DrawRetainedDecodePhase::VerbOpen => {
                if byte.is_ascii_whitespace() {
                    return;
                }
                self.phase = if byte == b'"' { DrawRetainedDecodePhase::Verb } else { DrawRetainedDecodePhase::Fault };
            }
            DrawRetainedDecodePhase::Verb => {
                if self.verb_cursor == self.expected_verb.len() {
                    self.phase = if byte == b'"' { DrawRetainedDecodePhase::Comma } else { DrawRetainedDecodePhase::Fault };
                } else if self.expected_verb.get(self.verb_cursor) == Some(&byte) {
                    self.verb_cursor += 1;
                } else {
                    self.phase = DrawRetainedDecodePhase::Fault;
                }
            }
            DrawRetainedDecodePhase::Comma => {
                if byte.is_ascii_whitespace() {
                    return;
                }
                self.phase = if byte == b',' { DrawRetainedDecodePhase::Value } else { DrawRetainedDecodePhase::Fault };
            }
            DrawRetainedDecodePhase::Value => self.feed_value(byte),
            DrawRetainedDecodePhase::Close => {
                if byte.is_ascii_whitespace() {
                    return;
                }
                self.phase = if byte == b']' { DrawRetainedDecodePhase::Complete } else { DrawRetainedDecodePhase::Fault };
            }
            DrawRetainedDecodePhase::Complete | DrawRetainedDecodePhase::Fault => {}
        }
    }

    fn feed_value(&mut self, byte: u8) {
        if self.value_complete {
            if byte.is_ascii_whitespace() {
                return;
            }
            self.phase = if byte == b']' { DrawRetainedDecodePhase::Complete } else { DrawRetainedDecodePhase::Fault };
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
            b']' if self.scalar => self.phase = DrawRetainedDecodePhase::Complete,
            b']' => self.phase = DrawRetainedDecodePhase::Fault,
            byte if byte.is_ascii_whitespace() && self.scalar => {
                self.scalar = false;
                self.value_complete = true;
            }
            _ => self.scalar = true,
        }
    }

    fn finish(&mut self) -> bool {
        if self.phase == DrawRetainedDecodePhase::Value && self.scalar {
            self.value_complete = true;
            self.phase = DrawRetainedDecodePhase::Close;
        }
        self.phase == DrawRetainedDecodePhase::Complete && self.verb_cursor == self.expected_verb.len() && self.nested_depth == 0 && !self.string && !self.escaped
    }
}

struct DrawGestureOperationJob {
    payload: Option<DrawGestureOperationPayload>,
    raw_input: Option<semio_framework::action_bus::RetainedToolWireInput>,
    raw_page_cursor: usize,
    raw_byte_cursor: usize,
    decoder: Option<DrawRetainedCommandDecoder>,
    raw_validated: bool,
    completed: bool,
    closing: bool,
}

impl semio_framework_job::InteractiveJob for DrawGestureOperationJob {
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
                    let Some(decoder) = self.decoder.as_mut() else { return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }) };
                    decoder.feed(*byte);
                    if decoder.phase == DrawRetainedDecodePhase::Fault {
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
            let exact = self.decoder.as_mut().is_some_and(DrawRetainedCommandDecoder::finish);
            if !exact {
                return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
            }
            self.raw_validated = true;
            context.consume_fuel(1);
            return semio_framework_job::StepOutcome::Yield;
        }
        if !self.completed {
            let Some(payload) = self.payload.as_ref() else { return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }) };
            let key = semio_framework_job::FixedOperationKey::new(semio_framework_job::OperationId(payload.operation_context.operation_id), semio_framework_job::Generation(payload.operation_context.generation));
            let emit = payload.instance_owner.with_mut::<DrawInstanceOperationOwner, _>(|owner| {
                owner.dispatch(key, payload.operation_context.canonical_base_revision, &payload.command, &payload.snapshot, &payload.config, &payload.history, payload.operation_context.clone())
            });
            let emit = match emit {
                Ok(Some(emit)) => Ok(emit),
                Ok(None) => {
                    context.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
                Err(error) => Err(error),
            };
            if payload.completion.complete(emit, semio_framework_plugin::EphemeralEmit::default()).is_err() {
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
        if !self.closing || maximum_items == 0 {
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
        self.closing && self.payload.is_none() && self.raw_input.is_none() && self.decoder.is_none()
    }
}

struct DrawGestureOperationJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl DrawGestureOperationJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: DRAW_GESTURE_TOOL_IDS.iter().map(|tool| semio_framework::ToolFactoryKey::new(controller_id, *tool)).collect() }
    }
}

impl semio_framework::ToolJobFactory for DrawGestureOperationJobFactory {
    type Payload = DrawGestureOperationPayload;
    type Job = DrawGestureOperationJob;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        DRAW_DOCUMENT_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(DRAW_GESTURE_RAW_BYTES, 32, 1, 16_384, 7_500, 1, 1)
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(DrawGestureOperationJob { payload: Some(payload), raw_input: None, raw_page_cursor: 0, raw_byte_cursor: 0, decoder: None, raw_validated: true, completed: false, closing: false })
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if checkpoint.is_some() || input.declared_bytes() > DRAW_GESTURE_RAW_BYTES {
            return Err((semio_framework::ToolJobFactoryError::new("Draw gesture retained ingress rejects a checkpoint or oversized wire owner"), input, checkpoint));
        }
        let mut job = match self.create_job(operation, payload) {
            Ok(job) => job,
            Err(error) => return Err((error, input, None)),
        };
        let Some(expected_verb) = job.payload.as_ref().map(|payload| payload.command.command_id()) else {
            return Err((semio_framework::ToolJobFactoryError::new("Draw gesture retained decoder has no exact typed command owner"), input, None));
        };
        job.raw_input = Some(input);
        job.decoder = Some(DrawRetainedCommandDecoder::new(expected_verb));
        job.raw_validated = false;
        Ok(job)
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for DrawGestureOperationJobFactory {
    type Owner = semio_framework_plugin::EditorApp<DrawPlayApp>;
    const TOOL_IDS: &'static [&'static str] = DRAW_GESTURE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = DRAW_DOCUMENT_SCHEMA;
}

#[cfg(test)]
mod gesture_operation_owner_tests {
    use super::*;

    fn decode_retained(expected: &'static str, wire: &[u8]) -> bool {
        let mut decoder = DrawRetainedCommandDecoder::new(expected);
        for byte in wire {
            decoder.feed(*byte);
        }
        decoder.finish()
    }

    fn key(operation: u64, generation: u64) -> semio_framework_job::FixedOperationKey {
        semio_framework_job::FixedOperationKey::new(semio_framework_job::OperationId(operation), semio_framework_job::Generation(generation))
    }

    fn drain(registry: &mut semio_framework_job::FixedOperationRegistry<DrawGestureOperationOwner, 64>) {
        for operation in 0..64 {
            registry.cancel(key(operation, 0));
        }
        for _ in 0..256 {
            if registry.is_empty() {
                return;
            }
            let _ = registry.close_step(1, DRAW_GESTURE_RETAINED_BYTES);
        }
        assert!(registry.is_empty());
    }

    #[test]
    fn draw_gesture_maximum_plus_one_returns_the_exact_owner() {
        let mut registry = semio_framework_job::FixedOperationRegistry::<DrawGestureOperationOwner, 64>::new(64 * DRAW_GESTURE_RETAINED_BYTES);
        for operation in 0..64 {
            if registry.admit(key(operation, 0), DrawGestureOperationOwner::new()).is_err() {
                panic!("every distinct fixed slot must admit through the declared maximum");
            }
        }
        let rejected = match registry.admit(key(64, 0), DrawGestureOperationOwner::new()) {
            Ok(()) => panic!("maximum plus one must return its exact owner"),
            Err(rejected) => rejected,
        };
        assert_eq!(rejected.key, key(64, 0));
        assert!(rejected.owner.session.is_some());
        let mut owner = rejected.owner;
        owner.cancel();
        owner.begin_close();
        assert_eq!(owner.close_step(1, DRAW_GESTURE_RETAINED_BYTES), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: DRAW_GESTURE_RETAINED_BYTES });
        assert!(owner.terminal_is_empty());
        drain(&mut registry);
    }

    #[test]
    fn draw_gesture_stale_generation_and_aba_are_exact() {
        let mut registry = semio_framework_job::FixedOperationRegistry::<DrawGestureOperationOwner, 64>::new(DRAW_GESTURE_RETAINED_BYTES);
        assert!(registry.admit(key(7, 1), DrawGestureOperationOwner::new()).is_ok());
        for _ in 0..64 {
            if registry.cancel_stale_step(semio_framework_job::OperationId(7), semio_framework_job::Generation(2)) {
                break;
            }
        }
        for _ in 0..128 {
            if registry.is_empty() {
                break;
            }
            let _ = registry.close_step(1, DRAW_GESTURE_RETAINED_BYTES);
        }
        assert!(registry.is_empty());
        assert!(registry.admit(key(7, 2), DrawGestureOperationOwner::new()).is_ok(), "the new generation owns the retired slot");
        registry.cancel(key(7, 2));
        for _ in 0..128 {
            if registry.is_empty() {
                break;
            }
            let _ = registry.close_step(1, DRAW_GESTURE_RETAINED_BYTES);
        }
        assert!(registry.is_empty());
    }

    #[test]
    fn draw_gesture_interrupted_and_repeated_close_is_terminal_empty() {
        let mut owner = DrawGestureOperationOwner::new();
        owner.cancel();
        owner.begin_close();
        assert_eq!(owner.close_step(0, DRAW_GESTURE_RETAINED_BYTES), semio_framework_job::InteractiveJobCloseStep::Blocked);
        assert_eq!(owner.close_step(1, DRAW_GESTURE_RETAINED_BYTES - 1), semio_framework_job::InteractiveJobCloseStep::Blocked);
        assert_eq!(owner.close_step(1, DRAW_GESTURE_RETAINED_BYTES), semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: DRAW_GESTURE_RETAINED_BYTES });
        assert_eq!(owner.close_step(1, DRAW_GESTURE_RETAINED_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete);
        assert!(owner.terminal_is_empty());
    }

    #[test]
    fn draw_retained_decoder_is_incremental_exact_and_fail_closed() {
        assert!(decode_retained("canvasPointerMove", br#"["canvasPointerMove",{"x":1,"y":2}]"#));
        assert!(decode_retained("canvasEscape", br#"["canvasEscape",null]"#));
        assert!(!decode_retained("canvasPointerMove", br#"["canvasPointerUp",{"x":1}]"#));
        assert!(!decode_retained("canvasPointerMove", br#"["canvasPointerMove",{"x":1}"#));
        assert!(!decode_retained("canvasPointerMove", br#"["canvasPointerMove",{"x":1}]x"#));
    }

    #[test]
    fn draw_preview_rejects_a_stale_revision_and_cancels_the_owner() {
        let mut owner = DrawInstanceOperationOwner::new();
        let operation = key(9, 4);
        assert!(owner.operations.admit(operation, DrawGestureOperationOwner::new()).is_ok());
        owner.active = Some((operation, [1; 32]));
        assert!(owner.preview_projection([2; 32], "selectDirect").is_none());
        assert!(owner.active.is_none());
        for _ in 0..128 {
            if owner.operations.is_empty() {
                break;
            }
            let _ = owner.operations.close_step(1, DRAW_GESTURE_RETAINED_BYTES);
        }
        assert!(owner.operations.is_empty());
    }
}
//#endregion 🧵️GestureOperationJobs

//#region 🔖️DrawPlayApp
pub(crate) fn draw_document_revision(doc: &ArtifactView<'_, DrawSnapshot>, _config: &DrawConfig) -> String {
    doc.operation_optional().map(|operation| operation.canonical_base_revision_hex()).unwrap_or_else(|| "0".repeat(64))
}

/// 🧪️ Unit struct apart from `session`: every former `DrawInteractionState`/`ViewModel`-derived field
/// lives in [`DrawConfig`], written through [`DrawConfigMutation`]s. `session` holds the one piece of
/// state that is neither document nor view-config — the live gesture statechart — threaded into every
/// command handler as the `app_commands!` dispatch context.
pub struct DrawPlayApp {
    arena_boot_fault: Option<&'static str>,
}

impl DrawPlayApp {
    pub fn arena_boot_fault(&self) -> Option<&'static str> {
        self.arena_boot_fault.or_else(crate::artifacts::draw::spr::draw_mutation_arena_pool_fault)
    }
}

fn render_draw_body(
    body_key: &str,
    document: &DrawSnapshot,
    config: &DrawConfig,
    preview: &DrawGesturePreview,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let labels = semio_framework_plugin::resolve_labels_for_locale::<DrawPlayLabels>(&config.locale);
    let active_utility = config.active_utility_id.as_str();
    let root = match body_key {
        DRAW_PLAY_BODY_COMPOSITE => canvas_window::render(document, config, preview, active_utility),
        DRAW_PLAY_BODY_LAYERS => layers_panel::render(document, labels),
        DRAW_PLAY_BODY_CATALOGUE => catalogue_panel::render(document, labels),
        DRAW_PLAY_BODY_PROPERTIES => properties_panel::render(document, active_utility),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}")))
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("draw.body.label", "the fixed Draw unknown-body label exceeds its UI bound")),
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(root))
}

impl Default for DrawPlayApp {
    fn default() -> Self {
        let arena_boot_fault = match crate::artifacts::draw::spr::request_draw_mutation_arena_pool() {
            crate::artifacts::draw::spr::DrawMutationArenaPoolAvailability::Fault(error) => Some(error),
            crate::artifacts::draw::spr::DrawMutationArenaPoolAvailability::Ready | crate::artifacts::draw::spr::DrawMutationArenaPoolAvailability::NotReady | crate::artifacts::draw::spr::DrawMutationArenaPoolAvailability::Contended => None,
        };
        Self { arena_boot_fault }
    }
}

impl ArtifactEditor for DrawPlayApp {
    type Snapshot = DrawSnapshot;
    type Mutation = DrawMutation;
    type Config = DrawConfig;
    type ConfigMutation = DrawConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = DrawPresence;
    type PresenceMutation = DrawPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = DrawCommand;

    const DIALECT: semio_framework::Dialect = crate::artifacts::draw::DRAW_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DRAW_DOCUMENT_SCHEMA;

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::draw::spr::draw_envelope_decode_owner_bundle())
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::draw::spr::draw_document_store_owners())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::artifacts::draw::spr::draw_document_store_initialization_job(envelope, operation, generation))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs",
        controller: "s.draw.draw@1/*#editor",
        document_schema: "draw.document",
        factory: "DrawGestureOperationJobFactory",
        factory_type: DrawGestureOperationJobFactory,
        tools: {
            "canvasPointerDown" => semio_framework::ToolExecutionContract::resumable(8_192, 32, 1, 16_384, 7_500, 1, 1),
            "canvasPointerMove" => semio_framework::ToolExecutionContract::resumable(8_192, 32, 1, 16_384, 7_500, 1, 1),
            "canvasPointerUp" => semio_framework::ToolExecutionContract::resumable(8_192, 32, 1, 16_384, 7_500, 1, 1),
            "canvasDoubleClick" => semio_framework::ToolExecutionContract::resumable(8_192, 32, 1, 16_384, 7_500, 1, 1),
            "canvasCommitDraft" => semio_framework::ToolExecutionContract::resumable(8_192, 32, 1, 16_384, 7_500, 1, 1),
            "canvasEscape" => semio_framework::ToolExecutionContract::resumable(8_192, 32, 1, 16_384, 7_500, 1, 1),
        }
    }

    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(DrawInstanceOperationOwner::new())
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(DrawGestureOperationJobFactory::new(&controller))
    }

    async fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !DRAW_GESTURE_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("draw.gesture.tool-mismatch"), "Draw gesture command does not match its exact registered tool"));
        }
        let operation_context = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = DrawGestureOperationPayload {
            command: *request.command,
            snapshot: request.snapshot,
            config: request.config,
            history: request.history,
            instance_owner: request.instance_operation_owner,
            operation_context,
            completion: request.completion,
        };
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::draw::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> DrawSnapshot {
        crate::artifacts::draw::schema::default_draw_document("empty", None)
    }

    async fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(draw_io())
    }

    /// 🎞️ `vector:out` (see `draw_vector_media`) plus the inherited `document:out` default (the pack
    /// of `doc.snapshot`, replicated inline — overriding `export_media` shadows the trait's provided
    /// body for every port on this app, not just the new one).
    async fn export_media(port: &str, doc: &ArtifactView<'_, DrawSnapshot>) -> Result<Media, MediaError> {
        match port {
            "vector:out" => draw_vector_media(doc.snapshot),
            "document:out" => {
                let media_type = Self::io().await.map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    // 🖼️ No override: whole-document replacement has no `Mutation` vehicle any more (banned
    // vocabulary — see `🧬️mutations/🦀️component.rs`'s module doc). The default `None` disables the
    // generic `import_media("document:in")` port for draw; explicit whole-document load/replace
    // stays reachable through the `set_snapshot`/`commit_document`/`set_fixture_json`/
    // `set_active_example` commands, which now emit `Effect::LoadDocument` (the sanctioned
    // non-history reset path) instead.

    /// 🏷️ `app_commands!`'s generated `command_id()`.
    async fn command_id(command: &DrawCommand) -> &'static str {
        command.command_id()
    }

    async fn handle(
        command: &DrawCommand,
        doc: &ArtifactView<'_, DrawSnapshot>,
        cfg: &ConfigView<'_, DrawConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<DrawMutation, DrawConfigMutation, Self::DraftMutation>, Fault> {
        if DRAW_GESTURE_TOOL_IDS.contains(&command.command_id()) {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("draw.gesture.retained-route"), "Draw gesture commands are reachable only through their exact retained factory owner"));
        }
        let mut session = DrawSession::default();
        session.interaction.ids = interaction.selection(DRAW_INTERACTION_DOMAIN).ids.clone();
        command.dispatch(doc, cfg, &mut session)
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_draw_body(body_key, doc.snapshot, cfg.snapshot, &DrawSession::default().preview())
    }

    async fn render_with_instance_operation_owner(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, DrawSnapshot>,
        cfg: &ConfigView<'_, DrawConfig>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let preview = match doc.render_operation() {
            Some(operation) => owner
                .with_mut::<DrawInstanceOperationOwner, _>(|owner| Ok(owner.preview_projection(operation.canonical_base_revision, &cfg.snapshot.active_utility_id)))
                .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("draw.gesture.preview-owner", error.message))?
                .unwrap_or_default(),
            None => DrawGesturePreview::default(),
        };
        render_draw_body(body_key, doc.snapshot, cfg.snapshot, &preview)
    }
}
//#endregion 🔖️DrawPlayApp

//#region 🔖️Io
/// 🌱️ Builds the single canonical non-history document-reset effect for Draw.
pub(crate) fn draw_reset_document_effect(scene: &DrawSnapshot) -> semio_framework_plugin::Effect {
    let pack = <DrawSnapshot as ArtifactPack>::encode_pack(scene);
    let envelope = store::create_document_envelope::<DrawSnapshot, DrawMutation>(DRAW_DOCUMENT_SCHEMA, &scene.id, scene.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("draw document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}

/// 🔌️ Relocated verbatim from the `⚙️engine` directory (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 4: anything returning `AppIo` or
/// referencing an app type lives in `🎛️apps/<app>/`). This app's typed media I/O surface
/// (`AppDefinition.io`) — mirrors the `2d.drawing` `ArtifactKindSpec` literal `create_draw_app`
/// already declares via `.artifact_kind(...)` (schema/media type/export+import formats copied
/// verbatim), plus the app-specific `vector:out` port (see `draw_vector_out_port` below).
pub fn draw_io() -> semio_framework::AppIo {
    semio_framework::AppIo {
        document_schema: DRAW_DOCUMENT_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        ports: vec![draw_vector_out_port()],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework::ArtifactPresentation { id: "2d.drawing".into(), name: "2D Drawing".into(), dimension: "2d".into(), component_kind: "draw".into() },
    }
}

/// 🔌️ `vector:out` — the draw document's current vector content, exported as SVG (workflow port
/// surface; WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe). Reuses the existing `2d.drawing`
/// kind (already declared by `create_draw_app`'s `.artifact_kind(...)`) rather than minting a
/// duplicate — `kind_id` just pins this port to that same catalog entry. `Many`/optional: a
/// consumer (e.g. raster's Vector→Raster-converted `image:in`) may connect before the canvas has
/// any content, or fan out to several consumers at once.
pub fn draw_vector_out_port() -> semio_framework::MediaPortSpec {
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

/// 🖼️ Exports the current draw document as an SVG `Media` payload for the `vector:out` port —
/// reuses `crate::artifacts::draw::io::draw_document_to_svg` (the same semio/drawing↔svg bridge the
/// export-svg shell path uses), so there is exactly one SVG renderer.
pub fn draw_vector_media(doc: &DrawSnapshot) -> Result<Media, MediaError> {
    let (svg, _width, _height) = crate::artifacts::draw::io::draw_document_to_svg(doc).map_err(|error| MediaError::Payload("vector:out".into(), error))?;
    Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "2d.drawing".into(), json: svg } })
}
//#endregion 🔖️Io

//#region 🔖️Manifest
pub fn create_draw_app() -> semio_framework_plugin::AppDefinition {
    let engagement = WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("draw-canvas-engagement".into()),
            value: Some(String::new()),
            placeholder: Some("Layer name".into()),
            on_change: Some(draw_manifest_action("engagementInput")),
            on_submit: Some(draw_manifest_action("engagementSubmit")),
            disabled: None,
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "draw-layer-count".into(), text: "0 layers · 0 selected".into() }]),
        possible_engagements: None,
    };
    Editor::builder(crate::artifacts::draw::DRAW_DIALECT).document(["semio", "draw"])
            .artifact_kind(crate::artifacts::draw::artifact_kind())
            .icon_id("draw")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind_with_engagement(DRAW_PLAY_WINDOW_CANVAS, LocalizedLabel::native("Canvas", "Leinwand"), DRAW_PLAY_BODY_COMPOSITE, semio_framework_ui_contract::SurfaceKind::Canvas2d, engagement, "pen-tool")
            .panel_tab_def(layers_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(properties_panel::definition())
            // ✏️ Palette-visible content operations.
            .mutation("addLayer", LocalizedLabel::native("Add Layer", "Ebene hinzufügen"))
            .mutation("combineBoolean", LocalizedLabel::native("Combine Boolean", "Boolean kombinieren"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            // 🔧️ Internal content operations — inspector/layer-panel/import-bound, not palette commands.
            .action_with(draw_internal_action("setSnapshot", LocalizedLabel::native("Set Document", "Dokument festlegen"), ActionKind::Mutation))
            .action_with(draw_internal_action("commitDocument", LocalizedLabel::native("Commit Document", "Dokument übernehmen"), ActionKind::Mutation))
            .action_with(draw_internal_action("setFixtureJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Mutation))
            .action_with(draw_internal_action("setSelectedOpacity", LocalizedLabel::native("Set Selected Opacity", "Deckkraft der Auswahl festlegen"), ActionKind::Mutation))
            .action_with(draw_internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Mutation))
            .action_with(draw_internal_action("dropLayerKind", LocalizedLabel::native("Drop Layer Kind", "Ebenenart ablegen"), ActionKind::Mutation))
            .action_with(draw_internal_action("moveLayer", LocalizedLabel::native("Move Layer", "Ebene verschieben"), ActionKind::Mutation))
            .action_with(draw_internal_action("deleteLayer", LocalizedLabel::native("Delete Layer", "Ebene löschen"), ActionKind::Mutation))
            .action_with(draw_internal_action("duplicateLayer", LocalizedLabel::native("Duplicate Layer", "Ebene duplizieren"), ActionKind::Mutation))
            .action_with(draw_internal_action("toggleLayerVisible", LocalizedLabel::native("Toggle Layer Visible", "Ebenensichtbarkeit umschalten"), ActionKind::Mutation))
            .action_with(draw_internal_action("patchLayer", LocalizedLabel::native("Patch Layer", "Ebene aktualisieren"), ActionKind::Mutation))
            .action_with(draw_internal_action("patchLayers", LocalizedLabel::native("Patch Layers", "Ebenen aktualisieren"), ActionKind::Mutation))
            // 🖱️ Internal pointer/gesture vocabulary — commit-time handlers emit operations, the rest are pure View.
            .action_with(draw_internal_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), ActionKind::Mutation))
            .action_interactive_job("canvasPointerDown", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(draw_internal_action("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), ActionKind::Mutation))
            .action_interactive_job("canvasPointerUp", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(draw_internal_action("canvasDoubleClick", LocalizedLabel::native("Canvas Double Click", "Leinwand-Doppelklick"), ActionKind::Mutation))
            .action_interactive_job("canvasDoubleClick", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(draw_internal_action("canvasCommitDraft", LocalizedLabel::native("Canvas Commit Draft", "Leinwand-Entwurf übernehmen"), ActionKind::Mutation))
            .action_interactive_job("canvasCommitDraft", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(draw_internal_action("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegen"), ActionKind::View))
            .action_interactive_job("canvasPointerMove", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(draw_internal_action("canvasEscape", LocalizedLabel::native("Canvas Escape", "Leinwand abbrechen"), ActionKind::View))
            .action_interactive_job("canvasEscape", semio_framework_plugin::InteractiveJobClassification::Migrated)
            // 👁️ Ephemeral view state — selection/hover are framework-owned now (see `.interaction(...)`
            // below): interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
            // setInteractionGranularity auto-inject, never declared here (ticket
            // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
            .action_with(draw_internal_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View))
            .action_with(draw_internal_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"), ActionKind::View))
            // 📷️ Camera — session-only runtime pose, never a document operation.
            .action_with(draw_internal_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View))
            .action_with(draw_internal_action("setCameraZoom", LocalizedLabel::native("Set Camera Zoom", "Kamerazoom festlegen"), ActionKind::View))
            // 🧰️ Canvas utilities — one exclusive set per window, active utility host-owned (never a document operation).
            .utility(draw_utility("selectMarquee", LocalizedLabel::native("Marquee Select", "Rahmenauswahl"), "square-dashed", "Select", UtilityCategory::Selection))
            .utility(draw_utility("selectLasso", LocalizedLabel::native("Lasso Select", "Lasso-Auswahl"), "lasso", "Select", UtilityCategory::Selection))
            .utility(draw_utility("selectDirect", LocalizedLabel::native("Direct Select", "Direktauswahl"), "mouse-pointer-2", "Select", UtilityCategory::Selection))
            .utility(draw_utility("pen", LocalizedLabel::native("Pen", "Stift"), "pen-tool", "Draw", UtilityCategory::Utilities))
            .utility(draw_utility("shapeRect", LocalizedLabel::native("Rectangle", "Rechteck"), "rectangle-tool", "Draw", UtilityCategory::Utilities))
            .utility(draw_utility("shapeEllipse", LocalizedLabel::native("Ellipse", "Ellipse"), "circle", "Draw", UtilityCategory::Utilities))
            .utility(draw_utility("shapeLine", LocalizedLabel::native("Line", "Linie"), "minus", "Draw", UtilityCategory::Utilities))
            .utility(draw_utility("shapePolygon", LocalizedLabel::native("Polygon", "Polygon"), "hexagon", "Draw", UtilityCategory::Utilities))
            .utility(draw_utility("booleanCombine", LocalizedLabel::native("Boolean", "Boolean"), "combine", "Combine", UtilityCategory::Utilities))
            .utility(draw_utility("trace", LocalizedLabel::native("Trace", "Nachzeichnen"), "scan-line", "Combine", UtilityCategory::Utilities))
            .utility(draw_utility("transformMove", LocalizedLabel::native("Pan", "Verschieben"), "move", "View", UtilityCategory::Utilities))
            .window_kind_utilities(DRAW_PLAY_WINDOW_CANVAS, vec![
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
                id: DRAW_INTERACTION_DOMAIN.into(),
                label: LocalizedLabel::native("Strokes", "Striche"),
                granularities: vec![GranularityDefinition { id: DRAW_INTERACTION_GRANULARITY.into(), label: LocalizedLabel::native("Stroke", "Strich"), icon_id: "pen-tool".into() }],
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
            .window_kind_interactions(DRAW_PLAY_WINDOW_CANVAS, vec![InteractionRef::new(DRAW_INTERACTION_DOMAIN)])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("escape", "canvasEscape")
            .keybinding("enter", "canvasCommitDraft")
            .default_layout(edit::layout())
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a
            // bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the old
            // `DRAW_PLAY_EXAMPLE_DEFAULT_ID` app-level example registration and the no-op
            // `.workflow("draw", …)` call are dropped here (not silently: reported in this packet's
            // migration report). The subset's own `📚️examples/🎬️demo` facet
            // (`crate::artifacts::draw::examples::...`, real content, pre-existing) is the modern,
            // role-agnostic replacement surface for this.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{App, EditorApp, VcsArtifactApp};

    pub type DrawApp = VcsArtifactApp<EditorApp<DrawPlayApp>>;

    /// ✏️ `DrawPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<DrawPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<DrawPlayApp>` builds it.

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn draw_app() -> DrawApp {
        new_app::<EditorApp<DrawPlayApp>>()
    }

    /// 🧪️ Adapts `create_draw_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `new_app_with_registry` still expects — framework testkit gap, not
    /// modifiable here (`🧰️framework/**` is outside this packet's lease).
    async fn draw_app_manifest_for_testkit() -> App {
        App { definition: create_draw_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn draw_app_with_registry() -> DrawApp {
        new_app_with_registry::<EditorApp<DrawPlayApp>>(draw_app_manifest_for_testkit)
    }

    /// 🧰️ Sets the config's host-owned active utility to `utility`.
    pub async fn set_utility(app: &mut DrawApp, utility: &str) {
        app.dispatch_typed(DrawCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: utility.into() }), &meta("local")).expect("set active utility");
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::draw::schema::{default_draw_document, layer_id, semio_draw_example_json};
    use crate::artifacts::draw::DrawLayerNode;
    use semio_framework_plugin::kernel::Effect;
    use semio_framework_plugin::{testkit as fw_testkit, PluginApp, ViewModel, SET_ACTIVE_UTILITY_ACTION_ID};
    use testkit::{draw_app, draw_app_with_registry, set_utility, DrawApp};

    fn draw_envelope_wire() -> Vec<u8> {
        use store::ArtifactPack;

        let mut snapshot = default_draw_document("draw-retained-load", None);
        let mut group = crate::artifacts::draw::schema::create_draw_group_layer("Nested");
        if let DrawLayerNode::Group(value) = &mut group {
            value.children.push(crate::artifacts::draw::schema::create_draw_path_layer("Path", vec![crate::artifacts::draw::PathSegment::Move { to: [1.0, 2.0] }, crate::artifacts::draw::PathSegment::Line { to: [3.0, 4.0] }]));
        }
        let retained_target = match &group {
            DrawLayerNode::Group(value) => crate::artifacts::draw::schema::layer_id(&value.children[0]).to_string(),
            _ => unreachable!("retained Draw fixture group remains exact"),
        };
        snapshot.layers.push(group);
        snapshot.assets.insert("image-a".into(), crate::artifacts::draw::DrawImageAsset { mime: "image/png".into(), data: "AA==".into(), width: Some(1), height: Some(1) });
        let snapshot_pack = snapshot.encode_pack();
        let snapshot_hex = snapshot_pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let wire = serde_json::to_vec(&serde_json::json!({
            "schema": DRAW_DOCUMENT_SCHEMA,
            "id": "draw-retained-load",
            "vcs": {
                "initialSnapshot": snapshot_hex,
                "edits": [{
                    "id": "draw-retained-edit-final",
                    "actor": "draw-retained-actor",
                    "forwards": [crate::artifacts::draw::mutations::DrawMutation::RenameLayer(crate::artifacts::draw::mutations::RenameLayer { layer_id: retained_target.clone(), new_name: "Retained Path".into() })],
                    "inverse": [],
                    "sequenceNumber": 1,
                    "startedAt": "2026-08-23T00:00:00.000Z"
                }],
                "changes": [],
                "checkpoints": [],
                "alternatives": []
            },
            "editMessages": [],
            "conflicts": []
        }))
        .expect("schema-first Draw fixture envelope");
        let envelope = store::create_document_envelope(DRAW_DOCUMENT_SCHEMA, "draw-retained-load", snapshot, None);
        let mut retirement = crate::artifacts::draw::spr::draw_envelope_decode_owner_bundle().retire_envelope(envelope);
        for _ in 0..100_000 {
            match retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Draw fixture envelope retirement") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    drop(retirement);
                    return wire;
                }
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
                }
                store::SnapshotRetirementStep::Blocked => panic!("unshared Draw fixture envelope retirement blocked"),
            }
        }
        panic!("Draw fixture envelope retirement did not reach terminal")
    }

    fn admit_draw_envelope(app: &mut DrawApp, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
        let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
        let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Draw live envelope ingress credits");
        for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
            let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
            bytes[..chunk.len()].copy_from_slice(chunk);
            let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Draw envelope page");
            app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Draw envelope page admission failed: {fault}"));
        }
        assert!(app.seal_artifact_envelope_ingress(handle).expect("Draw envelope seal"));
        handle
    }

    fn drive_draw_load(app: &mut DrawApp, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
        for _ in 0..100_000 {
            app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one Draw maintenance turn");
            let poll = app.advance_artifact_envelope_load(handle).expect("Draw load advancement");
            if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
                return poll;
            }
            std::thread::yield_now();
        }
        panic!("Draw retained envelope load did not reach terminal")
    }

    #[semio_framework_async_macros::async_test]
    async fn draw_live_envelope_submit_recursive_clone_swap_displaced_store_and_exact_ack_succeed() {
        let mut app = draw_app();
        let base_generation = app.artifact_generation_now();
        let handle = admit_draw_envelope(&mut app, &draw_envelope_wire());
        assert_eq!(handle.generation, base_generation);
        assert_eq!(drive_draw_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
        assert_eq!(app.artifact_generation_now().0, base_generation.0 + 1);
        let projection = app.snapshot().expect("Draw retained mutation publication");
        let renamed = crate::artifacts::draw::schema::find_draw_layer(&projection, &crate::artifacts::draw::schema::create_draw_id("path", b"Path")).expect("retained Draw target");
        assert_eq!(crate::artifacts::draw::schema::layer_base(renamed).name, "Retained Path");
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("first Draw acknowledgement"));
        assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate Draw acknowledgement"));
    }

    #[semio_framework_async_macros::async_test]
    async fn draw_live_envelope_cancel_closes_retained_pages_without_publication() {
        let mut app = draw_app();
        let base_generation = app.artifact_generation_now();
        let wire = draw_envelope_wire();
        let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
        let handle = app.begin_artifact_envelope_ingress(pages, wire.len()).expect("cancelled Draw ingress credits");
        let first = &wire[..wire.len().min(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)];
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..first.len()].copy_from_slice(first);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, first.len()).expect("cancelled Draw first page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("cancelled Draw page admission failed: {fault}"));
        app.cancel_artifact_envelope_load(handle).expect("cancel Draw ingress");
        assert_eq!(drive_draw_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(app.artifact_generation_now(), base_generation);
    }

    #[semio_framework_async_macros::async_test]
    async fn draw_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle() {
        for turns in [0usize, 1, 2, 8] {
            let mut app = draw_app();
            let base_generation = app.artifact_generation_now();
            let base_id = app.snapshot().expect("Draw last-valid snapshot").id;
            let handle = admit_draw_envelope(&mut app, &draw_envelope_wire());
            for _ in 0..turns {
                app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("bounded Draw staged maintenance");
            }
            let stale = semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle { operation: handle.operation, generation: semio_framework_job::Generation(handle.generation.0 + 1) };
            assert!(app.advance_artifact_envelope_load(stale).is_err(), "stale staged handle cannot consume the exact operation owner");
            app.cancel_artifact_envelope_load(handle).expect("exact Draw staged cancellation");
            assert!(matches!(drive_draw_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled));
            assert_eq!(app.artifact_generation_now(), base_generation);
            assert_eq!(app.snapshot().expect("Draw last-valid survives staged cancel").id, base_id);
        }

        let mut app = draw_app();
        let handle = admit_draw_envelope(&mut app, &draw_envelope_wire());
        assert_eq!(drive_draw_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
        let stale = semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle { operation: handle.operation, generation: semio_framework_job::Generation(handle.generation.0 + 1) };
        assert!(app.acknowledge_artifact_store_replacement(stale).is_err(), "stale ACK cannot retire the exact committed owner");
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("exact staged Draw ACK"));
        assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate staged Draw ACK is idempotent"));
    }

    #[semio_framework_async_macros::async_test]
    async fn draw_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate() {
        for final_edit in [false, true] {
            let mut value: serde_json::Value = serde_json::from_slice(&draw_envelope_wire()).expect("Draw retained fixture JSON");
            let edits = value.pointer_mut("/vcs/edits").and_then(serde_json::Value::as_array_mut).expect("Draw retained edits");
            if final_edit {
                let mut first = edits[0].clone();
                first["id"] = serde_json::Value::String("draw-retained-edit-first".into());
                edits.insert(0, first);
            }
            edits.last_mut().expect("Draw final edit")["id"] = serde_json::Value::String("x".repeat(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES + 1));
            let wire = serde_json::to_vec(&value).expect("hostile Draw edit fixture");
            let mut app = draw_app();
            let generation = app.artifact_generation_now();
            let handle = admit_draw_envelope(&mut app, &wire);
            assert_eq!(drive_draw_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
            assert_eq!(app.artifact_generation_now(), generation);
        }
    }

    async fn first_layer_id(app: &DrawApp) -> String {
        layer_id(&app.snapshot().expect("materialize projection").layers[0]).to_string()
    }

    async fn last_layer_id(app: &DrawApp) -> String {
        let projection = app.snapshot().expect("materialize projection");
        layer_id(projection.layers.last().expect("layer")).to_string()
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_canvas_scene_with_segments() {
        let mut app = draw_app();
        let example_json = semio_draw_example_json();
        let node = app.render(DRAW_PLAY_BODY_COMPOSITE, Some(example_json.as_str()), &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
        let value = serde_json::to_value(&node).unwrap();
        let layers_json = value.pointer("/canvas2d/layersJson").and_then(|v| v.as_str()).expect("layersJson string");
        assert!(layers_json.contains("segments"));
        let records: Vec<Value> = serde_json::from_str(layers_json).unwrap();
        assert!(records.iter().any(|record| record.get("role").and_then(|value| value.as_str()) == Some("meta")));
        assert!(records.iter().any(|record| record.get("id").and_then(|value| value.as_str()) == Some("artboard:frame")), "canvas must show the document artboard frame");
        assert!(
            records.iter().any(|record| { record.get("id").and_then(|value| value.as_str()) == Some("artboard:dimensions") && record.pointer("/text/content").and_then(|value| value.as_str()).is_some_and(|label| label.contains('×')) }),
            "canvas must show document dimension label"
        );
        assert!(layers_json.contains("200 × 200"), "example artboard dimensions must be visible");
    }

    #[semio_framework_async_macros::async_test]
    async fn default_document_exposes_artboard_dimensions_on_canvas() {
        let mut app = draw_app();
        let node = app.render(DRAW_PLAY_BODY_COMPOSITE, None, &ViewModel::default()).expect("render");
        let value = serde_json::to_value(&node).unwrap();
        let layers_json = value.pointer("/canvas2d/layersJson").and_then(|v| v.as_str()).expect("layersJson string");
        assert!(layers_json.contains("1024 × 1024"), "blank documents show default artboard dimensions");
    }

    #[semio_framework_async_macros::async_test]
    async fn layers_panel_lists_default_layer() {
        let mut app = draw_app();
        let node = app.render(DRAW_PLAY_BODY_LAYERS, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("draw-play-layers.add.path"));
        assert!(json.contains("Layer 1"));
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_panel_lists_boolean_operations() {
        let mut app = draw_app();
        let node = app.render(DRAW_PLAY_BODY_CATALOGUE, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("draw-play-catalogue.path"));
        assert!(json.contains("Boolean union"));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_layer_action_emits_op_and_appends_path() {
        let mut app = draw_app();
        let before = app.snapshot().unwrap().layers.len();
        let result = app.dispatch_typed(DrawCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &fw_testkit::meta("local")).expect("add layer");
        assert_eq!(result.mutations.len(), 1);
        let projection = app.snapshot().unwrap();
        assert_eq!(projection.layers.len(), before + 1);
        assert!(projection.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Shape(shape) if shape.shape_kind == "rect")));
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_layers_opacity_emits_granular_operation() {
        let mut app = draw_app();
        let id = first_layer_id(&app);
        let result = app.dispatch_typed(DrawCommand::PatchLayers(patch_layers::PatchLayers { layer_ids: vec![id], field: "opacity".into(), value: "0.5".into() }), &fw_testkit::meta("local")).expect("patch");
        assert_eq!(result.mutations.len(), 1);
        let projection = app.snapshot().unwrap();
        assert!((crate::artifacts::draw::schema::layer_base(&projection.layers[0]).opacity - 0.5).abs() < f64::EPSILON);
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_layer_name_emits_op_and_changes_projection() {
        let mut app = draw_app();
        let id = first_layer_id(&app);
        let result = app.dispatch_typed(DrawCommand::PatchLayer(patch_layer::PatchLayer { layer_id: id, field: "name".into(), value: "Renamed".into() }), &fw_testkit::meta("local")).expect("patch");
        assert_eq!(result.mutations.len(), 1);
        assert_eq!(crate::artifacts::draw::schema::layer_base(&app.snapshot().unwrap().layers[0]).name, "Renamed");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_utility_clears_scratch_and_emits_no_history_entry() {
        let mut app = draw_app_with_registry();
        set_utility(&mut app, "shapeRect");
        app.dispatch_typed(
            DrawCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
                x: 10.0,
                y: 10.0,
                width: 800.0,
                height: 600.0,
                shift: false,
                ctrl: false,
                meta: false,
                generation: None,
                checkpoint_completed_work: None,
                checkpoint_pending_work: None,
                ..Default::default()
            }),
            &fw_testkit::meta("local"),
        )
        .expect("down");
        let before = app.snapshot().unwrap();
        let result = app.dispatch_typed(DrawCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "pen".into() }), &fw_testkit::meta("local")).expect("switch utility");
        assert!(result.mutations.is_empty(), "utility switching never emits document operations");
        assert_eq!(app.snapshot().unwrap(), before, "utility switching does not mutate the document");
        let up = app.dispatch_typed(DrawCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 40.0, y: 40.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }), &fw_testkit::meta("local")).expect("up");
        assert!(up.mutations.is_empty(), "the in-progress shape draft was cleared on utility switch");
    }

    #[semio_framework_async_macros::async_test]
    async fn combine_boolean_creates_boolean_layer() {
        let mut app = draw_app();
        let first_id = first_layer_id(&app);
        app.dispatch_typed(DrawCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &fw_testkit::meta("local")).expect("add rect");
        let second_id = last_layer_id(&app);
        let result = app.dispatch_typed(DrawCommand::CombineBoolean(combine_boolean::CombineBoolean { operation: "union".into(), ids: vec![first_id, second_id] }), &fw_testkit::meta("local")).expect("combine");
        assert_eq!(result.mutations.len(), 1);
        assert!(app.snapshot().unwrap().layers.iter().any(|layer| matches!(layer, DrawLayerNode::Boolean(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn canvas_point_to_world_matches_host_formula() {
        let camera = crate::artifacts::draw::DrawCamera { x: 100.0, y: 50.0, zoom: 2.0 };
        let (world_x, world_y) = canvas_pointer_down::canvas_point_to_world(&camera, 420.0, 310.0, 800.0, 600.0);
        assert!((world_x - 110.0).abs() < 1e-9);
        assert!((world_y - 55.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn shape_rect_drag_commits_one_layer_and_requests_utility_reset() {
        let mut app = draw_app_with_registry();
        set_utility(&mut app, "shapeRect");
        app.dispatch_typed(
            DrawCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
                x: 500.0,
                y: 400.0,
                width: 1000.0,
                height: 800.0,
                shift: false,
                ctrl: false,
                meta: false,
                generation: None,
                checkpoint_completed_work: None,
                checkpoint_pending_work: None,
                ..Default::default()
            }),
            &fw_testkit::meta("local"),
        )
        .expect("down");
        app.dispatch_typed(DrawCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 600.0, y: 500.0, width: 1000.0, height: 800.0 }), &fw_testkit::meta("local")).expect("move");
        let result = app.dispatch_typed(DrawCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 600.0, y: 500.0, width: 1000.0, height: 800.0, shift: false, ctrl: false, meta: false }), &fw_testkit::meta("local")).expect("up");
        assert_eq!(result.mutations.len(), 1, "a shape drag commits as one edit adding exactly the layer");
        let projection = app.snapshot().unwrap();
        assert!(projection.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Shape(shape) if shape.shape_kind == "rect")));
        assert!(
            matches!(
                result.requested_effects.as_slice(),
                [Effect::SetActiveUtility { window_id, utility_id }] if window_id == DRAW_PLAY_WINDOW_CANVAS && utility_id == "selectDirect"
            ),
            "the canvas returns to select-direct via a host effect, not a document operation"
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn pen_draft_commits_path_layer_on_enter() {
        let mut app = draw_app();
        set_utility(&mut app, "pen");
        app.dispatch_typed(
            DrawCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
                x: 400.0,
                y: 300.0,
                width: 800.0,
                height: 600.0,
                shift: false,
                ctrl: false,
                meta: false,
                generation: None,
                checkpoint_completed_work: None,
                checkpoint_pending_work: None,
                ..Default::default()
            }),
            &fw_testkit::meta("local"),
        )
        .expect("p1");
        app.dispatch_typed(
            DrawCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
                x: 500.0,
                y: 300.0,
                width: 800.0,
                height: 600.0,
                shift: false,
                ctrl: false,
                meta: false,
                generation: None,
                checkpoint_completed_work: None,
                checkpoint_pending_work: None,
                ..Default::default()
            }),
            &fw_testkit::meta("local"),
        )
        .expect("p2");
        let result = app.dispatch_typed(DrawCommand::CanvasCommitDraft(canvas_commit_draft::CanvasCommitDraft {}), &fw_testkit::meta("local")).expect("commit");
        assert_eq!(result.mutations.len(), 1, "the draft commits as exactly one AddLayer edit");
        let projection = app.snapshot().unwrap();
        assert!(projection.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Path(path) if !path.segments.is_empty())));
        assert!(matches!(result.requested_effects.as_slice(), [Effect::SetActiveUtility { utility_id, .. }] if utility_id == "selectDirect"));
    }

    #[semio_framework_async_macros::async_test]
    async fn canvas_escape_cancels_draft_without_committing() {
        let mut app = draw_app();
        let before = app.snapshot().unwrap().layers.len();
        set_utility(&mut app, "pen");
        app.dispatch_typed(
            DrawCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
                x: 400.0,
                y: 300.0,
                width: 800.0,
                height: 600.0,
                shift: false,
                ctrl: false,
                meta: false,
                generation: None,
                checkpoint_completed_work: None,
                checkpoint_pending_work: None,
                ..Default::default()
            }),
            &fw_testkit::meta("local"),
        )
        .expect("p1");
        let result = app.dispatch_typed(DrawCommand::CanvasEscape(canvas_escape::CanvasEscape {}), &fw_testkit::meta("local")).expect("escape");
        assert!(result.mutations.is_empty());
        assert_eq!(app.snapshot().unwrap().layers.len(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn marquee_select_covers_contained_layer_only() {
        // 🔖 Built through dispatched commands (`add-layer` + `patch-layer` transform fields), never
        // a whole-document swap — `SetSnapshot` is banned vocabulary now (see
        // `🧬️mutations/🦀️component.rs`'s module doc); this exercises the same real semantic
        // `create-layer`/`update-layer-transform` mutations a live editor session would emit.
        let mut app = draw_app();
        set_utility(&mut app, "selectMarquee");
        let initial_id = layer_id(&app.snapshot().unwrap().layers[0]).to_string();
        app.dispatch_typed(DrawCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: initial_id }), &fw_testkit::meta("local")).expect("clear default layer");

        app.dispatch_typed(DrawCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &fw_testkit::meta("local")).expect("add rect");
        let rect_a_id = layer_id(app.snapshot().unwrap().layers.last().unwrap()).to_string();
        for (field, value) in [("transformX", "10"), ("transformY", "10"), ("transformScaleX", "0.15625"), ("transformScaleY", "0.208333")] {
            app.dispatch_typed(DrawCommand::PatchLayer(patch_layer::PatchLayer { layer_id: rect_a_id.clone(), field: field.into(), value: value.into() }), &fw_testkit::meta("local")).expect("position rect a");
        }

        app.dispatch_typed(DrawCommand::AddLayer(add_layer::AddLayer { kind: "shape:ellipse".into() }), &fw_testkit::meta("local")).expect("add ellipse");
        let ellipse_b_id = layer_id(app.snapshot().unwrap().layers.last().unwrap()).to_string();
        for (field, value) in [("transformX", "200"), ("transformY", "200")] {
            app.dispatch_typed(DrawCommand::PatchLayer(patch_layer::PatchLayer { layer_id: ellipse_b_id.clone(), field: field.into(), value: value.into() }), &fw_testkit::meta("local")).expect("position ellipse b");
        }

        app.dispatch_typed(DrawCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::draw::DrawCamera { x: 0.0, y: 0.0, zoom: 1.0 } }), &fw_testkit::meta("local")).expect("camera");
        app.dispatch_typed(
            DrawCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
                x: 400.0,
                y: 300.0,
                width: 800.0,
                height: 600.0,
                shift: false,
                ctrl: false,
                meta: false,
                generation: None,
                checkpoint_completed_work: None,
                checkpoint_pending_work: None,
                ..Default::default()
            }),
            &fw_testkit::meta("local"),
        )
        .expect("down");
        app.dispatch_typed(DrawCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 460.0, y: 360.0, width: 800.0, height: 600.0 }), &fw_testkit::meta("local")).expect("move");
        let result = app.dispatch_typed(DrawCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 460.0, y: 360.0, width: 800.0, height: 600.0, shift: false, ctrl: false, meta: false }), &fw_testkit::meta("local")).expect("up");
        // 🕹️ Selection is framework-owned now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
        // the marquee hit-test requests `interactionSelect` for exactly the contained rect via a
        // `Effect::ReplayShellCommand`, instead of writing a `DrawConfigMutation::SetSelection`.
        assert!(result.mutations.is_empty(), "a pure marquee-select gesture is not a document operation");
        assert_eq!(result.requested_effects, vec![canvas_pointer_down::interaction_select_effect(&[rect_a_id.clone()], "replace")], "only the contained rect is requested, not the outside ellipse");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_camera_writes_runtime_and_emits_no_operations() {
        let mut app = draw_app();
        let before = app.snapshot().expect("projection");
        let result = app.dispatch_typed(DrawCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::draw::DrawCamera { x: 5.0, y: 5.0, zoom: 2.0 } }), &fw_testkit::meta("local")).expect("camera");
        assert!(result.mutations.is_empty(), "camera is a view action and emits no operations");
        assert_eq!(app.snapshot().expect("projection"), before, "camera never mutates the document");
        let json = serde_json::to_string(&app.render(DRAW_PLAY_BODY_COMPOSITE, None, &ViewModel::default()).expect("render")).unwrap();
        assert!(json.contains(r#""zoom":2.0"#), "composite scene camera reflects runtime state: {json}");
        assert!(json.contains(r#""cameraX":5.0"#), "composite scene camera reflects runtime state: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_camera_zoom_updates_zoom_and_keeps_pan_via_runtime() {
        let mut app = draw_app();
        app.dispatch_typed(DrawCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::draw::DrawCamera { x: 4.0, y: 5.0, zoom: 1.0 } }), &fw_testkit::meta("local")).expect("set camera");
        let result = app.dispatch_typed(DrawCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { value: 3.0 }), &fw_testkit::meta("local")).expect("set camera zoom");
        assert!(result.mutations.is_empty(), "camera zoom is a view action and emits no operations");
        let json = serde_json::to_string(&app.render(DRAW_PLAY_BODY_COMPOSITE, None, &ViewModel::default()).expect("render")).unwrap();
        assert!(json.contains(r#""zoom":3.0"#), "zoom updated: {json}");
        assert!(json.contains(r#""cameraX":4.0"#), "pan preserved across zoom-only update: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn add_layer_undo_round_trip_through_wrapper() {
        let mut app = draw_app();
        let before = app.snapshot().unwrap().layers.len();
        fw_testkit::assert_undo_redo_round_trip(&mut app, DrawCommand::AddLayer(add_layer::AddLayer { kind: "path".into() }), |app| app.snapshot().unwrap().layers.len(), before, before + 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn utility_registry_declares_all_canvas_utilities_scoped_to_the_window() {
        let definition = create_draw_app();
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["selectMarquee", "selectLasso", "selectDirect", "pen", "shapeRect", "shapeEllipse", "shapeLine", "shapePolygon", "booleanCombine", "trace", "transformMove"],);
        let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
        assert_eq!(selects, ["selectMarquee", "selectLasso", "selectDirect"]);
        let scene = definition.window_kinds.iter().find(|window| window.id == DRAW_PLAY_WINDOW_CANVAS).expect("canvas window");
        assert_eq!(scene.utilities.len(), definition.utilities.len(), "every utility is scoped to the canvas window kind");
        assert!(scene.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID && matches!(action.kind, ActionKind::View)));
        assert!(!definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == "setActiveUtility" && !matches!(action.kind, ActionKind::View)));
    }

    #[semio_framework_async_macros::async_test]
    async fn strokes_interaction_domain_is_declared_flat_pick_rectangle_lasso_on_the_canvas_window() {
        let definition = create_draw_app();
        let domain = definition.interactions.iter().find(|interaction| interaction.id == DRAW_INTERACTION_DOMAIN).expect("strokes interaction domain declared");
        assert!(matches!(domain.hierarchy, HierarchyProvider::Flat));
        assert_eq!(domain.selection.methods, vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso]);
        let canvas_window = definition.window_kinds.iter().find(|window| window.id == DRAW_PLAY_WINDOW_CANVAS).expect("canvas window");
        assert!(canvas_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == DRAW_INTERACTION_DOMAIN));
    }

    #[semio_framework_async_macros::async_test]
    async fn canvas_pointer_up_direct_pick_requests_interaction_select() {
        let mut app = draw_app_with_registry();
        // 🔖 The default document's one layer is an empty-segment path (no bounds to hit-test against
        // — see `default_draw_document`), so a real shape is added first, mirroring
        // `marquee_select_covers_contained_layer_only`'s own setup.
        let initial_id = first_layer_id(&app);
        app.dispatch_typed(DrawCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: initial_id }), &fw_testkit::meta("local")).expect("clear default layer");
        app.dispatch_typed(DrawCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &fw_testkit::meta("local")).expect("add rect");
        let rect_id = last_layer_id(&app);
        app.dispatch_typed(DrawCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::draw::DrawCamera { x: 0.0, y: 0.0, zoom: 1.0 } }), &fw_testkit::meta("local")).expect("camera");
        set_utility(&mut app, "selectDirect");
        // 🎯️ Default `shape:rect` geometry is world (0,0)-(128,96); screen (110,110) on a 200x200
        // viewport with the identity camera above maps to world (10,10) — inside the rect.
        let result = app.dispatch_typed(DrawCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 110.0, y: 110.0, width: 200.0, height: 200.0, shift: false, ctrl: false, meta: false }), &fw_testkit::meta("local")).expect("pick");
        assert!(result.mutations.is_empty(), "a direct pick is not a document operation");
        assert_eq!(result.requested_effects, vec![canvas_pointer_down::interaction_select_effect(&[rect_id], "replace")]);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_selected_opacity_reads_the_framework_interaction_selection() {
        let mut app = draw_app_with_registry();
        let id = first_layer_id(&app);
        let targets = serde_json::to_string(&vec![serde_json::json!({ "granularity": DRAW_INTERACTION_GRANULARITY, "id": id })]).unwrap();
        app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&serde_json::json!({ "domainId": DRAW_INTERACTION_DOMAIN, "targets": targets, "merge": "replace" })), &fw_testkit::meta("local")).expect("select");
        let result = app.dispatch_typed(DrawCommand::SetSelectedOpacity(set_selected_opacity::SetSelectedOpacity { value: 0.25 }), &fw_testkit::meta("local")).expect("opacity");
        assert_eq!(result.mutations.len(), 1);
        assert!((crate::artifacts::draw::schema::layer_base(&app.snapshot().unwrap().layers[0]).opacity - 0.25).abs() < f64::EPSILON);
    }

    #[semio_framework_async_macros::async_test]
    async fn draw_labels_resolve_native_by_default() {
        let mut app = draw_app();
        let node = app.render(DRAW_PLAY_BODY_LAYERS, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Add Path"));
        assert!(json.contains("Add Rectangle"));
        assert!(!json.contains("Pfad hinzufügen"));
    }

    #[semio_framework_async_macros::async_test]
    async fn draw_labels_translate_panels_in_german() {
        let mut app = draw_app();
        app.dispatch_typed(DrawCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }), &fw_testkit::meta("local")).expect("set locale");
        let layers_node = app.render(DRAW_PLAY_BODY_LAYERS, None, &ViewModel::default()).expect("render");
        let layers_json = serde_json::to_string(&layers_node).unwrap();
        assert!(layers_json.contains("Pfad hinzufügen"));
        assert!(layers_json.contains("Rechteck hinzufügen"));
        assert!(!layers_json.contains("Add Path"));
        let catalogue_node = app.render(DRAW_PLAY_BODY_CATALOGUE, None, &ViewModel::default()).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue_node).unwrap();
        assert!(catalogue_json.contains("\"Ellipse\""));
        assert!(catalogue_json.contains("Nachzeichnung"));
    }

    #[semio_framework_async_macros::async_test]
    async fn draw_io_declares_vector_out_and_export_media_covers_both_ports() {
        let mut app = draw_app();
        app.dispatch_typed(DrawCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }), &fw_testkit::meta("local")).expect("add");
        let projection = app.snapshot().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&projection, &history);
        let vector = semio_framework_plugin::resolve_ready(DrawPlayApp::export_media("vector:out", &doc)).expect("vector:out");
        let MediaPayload::Structured { schema, json } = vector.payload else { panic!("expected structured svg payload") };
        assert_eq!(schema, "2d.drawing");
        assert!(json.starts_with("<svg"));
        assert!(semio_framework_plugin::resolve_ready(DrawPlayApp::export_media("document:out", &doc)).is_ok());
        assert!(matches!(semio_framework_plugin::resolve_ready(DrawPlayApp::export_media("unknown:out", &doc)), Err(MediaError::NotImplemented)));
    }

    //#region 🔖️GesturePreview
    #[semio_framework_async_macros::async_test]
    async fn gesture_preview_is_none_while_idle() {
        let session = DrawSession::default();
        assert_eq!(session.preview().phase, canvas_pointer_down::DrawGesturePreviewPhase::Idle, "idle has an empty fixed projection");
    }

    #[semio_framework_async_macros::async_test]
    async fn gesture_preview_reflects_live_shape_drag_and_clears_on_commit() {
        let mut session = DrawSession::default();
        let document = default_draw_document("empty", None);
        let config = DrawConfig { active_utility_id: "shapeRect".into(), ..Default::default() };

        let down = session.step_gesture(canvas_pointer_down::draw_gesture::Event::PointerDown { utility: "shapeRect".into(), world: [10.0, 10.0], shift: false, ctrl: false, meta: false }, &document, &config);
        assert!(down.artifact_mutations.is_empty(), "pointer-down starts a scratch drag, not a document operation");
        let preview = session.preview();
        let seq_after_down = preview.sequence;
        assert_eq!(preview.context.start, [10.0, 10.0]);
        assert_eq!(preview.context.cursor, [10.0, 10.0]);

        let moved = session.step_gesture(canvas_pointer_down::draw_gesture::Event::PointerMove { world: [40.0, 30.0], marquee_threshold_world: 4.0 }, &document, &config);
        assert!(moved.artifact_mutations.is_empty(), "mid-drag ticks emit zero operations (scratch-commit pattern)");
        let preview = session.preview();
        assert_eq!(preview.context.cursor, [40.0, 30.0], "preview tracks the live cursor, not the drag start");
        assert!(preview.sequence > seq_after_down, "seq is monotone per tick, for staleness detection on the receiving end");

        let up = session.step_gesture(canvas_pointer_down::draw_gesture::Event::PointerUp { utility: "shapeRect".into(), world: [40.0, 30.0], shift: false, ctrl: false, meta: false }, &document, &config);
        assert_eq!(up.artifact_mutations.len(), 1, "pointer-up commits the shape as one real DrawMutation");
        assert_eq!(session.preview().phase, canvas_pointer_down::DrawGesturePreviewPhase::Idle, "the committed projection is terminal idle");
    }

    #[semio_framework_async_macros::async_test]
    async fn gesture_preview_is_a_pure_read_never_mutating_gesture_context() {
        let mut session = DrawSession::default();
        let document = default_draw_document("empty", None);
        let config = DrawConfig { active_utility_id: "shapeRect".into(), ..Default::default() };
        session.step_gesture(canvas_pointer_down::draw_gesture::Event::PointerDown { utility: "shapeRect".into(), world: [1.0, 2.0], shift: false, ctrl: false, meta: false }, &document, &config);
        let context_before = session.gesture.context.clone();
        let _ = session.preview();
        let _ = session.preview();
        assert_eq!(session.gesture.context, context_before, "preview must never mutate the live gesture scratch it reads");
    }
    //#endregion 🔖️GesturePreview

    //#region 🔖️WireGuards
    /// 🔖️ One `DrawCommand` value per row, in binary-variant-ordinal order — feeds both the
    /// op-text/binary equivalence loop and the "printed line starts with the row's wire keyword"
    /// assertion. Permanent wire guard: appending a variant is safe, reordering breaks the format.
    async fn every_command() -> Vec<DrawCommand> {
        vec![
            DrawCommand::SetSnapshot(set_snapshot::SetSnapshot { snapshot: default_draw_document("cmd-doc", None) }),
            DrawCommand::CommitDocument(commit_document::CommitDocument { snapshot: default_draw_document("cmd-doc-2", None) }),
            DrawCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: "{}".into() }),
            DrawCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "semio".into() }),
            DrawCommand::SetSelectedOpacity(set_selected_opacity::SetSelectedOpacity { value: 0.5 }),
            DrawCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("Renamed \"layer\"".into()) }),
            DrawCommand::AddLayer(add_layer::AddLayer { kind: "shape:rect".into() }),
            DrawCommand::DropLayerKind(drop_layer_kind::DropLayerKind { kind: "path".into(), target_row_id: "draw-play-layers".into(), drop_position: "inside".into() }),
            DrawCommand::MoveLayer(move_layer::MoveLayer { layer_id: "layer-1".into(), target_row_id: "draw-play-layers".into(), drop_position: "after".into() }),
            DrawCommand::DeleteLayer(delete_layer::DeleteLayer { layer_id: "layer-1".into() }),
            DrawCommand::DuplicateLayer(duplicate_layer::DuplicateLayer { layer_id: "layer-1".into() }),
            DrawCommand::ToggleLayerVisible(toggle_layer_visible::ToggleLayerVisible { layer_id: "layer-1".into() }),
            DrawCommand::CombineBoolean(combine_boolean::CombineBoolean { operation: "union".into(), ids: vec!["a".into(), "b".into()] }),
            DrawCommand::PatchLayer(patch_layer::PatchLayer { layer_id: "layer-1".into(), field: "opacity".into(), value: "0.4".into() }),
            DrawCommand::PatchLayers(patch_layers::PatchLayers { layer_ids: vec!["a".into(), "b".into()], field: "blendMode".into(), value: "\"multiply\"".into() }),
            DrawCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "pen".into() }),
            DrawCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::draw::DrawCamera { x: 1.0, y: 2.0, zoom: 1.5 } }),
            DrawCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { value: 2.0 }),
            DrawCommand::EngagementInput(engagement_input::EngagementInput { value: "typing".into() }),
            DrawCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            DrawCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {
                x: 1.0,
                y: 2.0,
                width: 800.0,
                height: 600.0,
                shift: true,
                ctrl: false,
                meta: false,
                generation: None,
                checkpoint_completed_work: None,
                checkpoint_pending_work: None,
                ..Default::default()
            }),
            DrawCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 1.0, y: 2.0, width: 800.0, height: 600.0 }),
            DrawCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x: 1.0, y: 2.0, width: 800.0, height: 600.0, shift: false, ctrl: true, meta: false }),
            DrawCommand::CanvasDoubleClick(canvas_double_click::CanvasDoubleClick {}),
            DrawCommand::CanvasCommitDraft(canvas_commit_draft::CanvasCommitDraft {}),
            DrawCommand::CanvasEscape(canvas_escape::CanvasEscape {}),
        ]
    }

    #[semio_framework_async_macros::async_test]
    async fn draw_command_op_text_round_trips_every_variant() {
        for command in every_command() {
            store::os_store::test_support::assert_op_line_round_trip(&command);
        }
        // The `None`-field variant missing from `every_command` (kept distinct from its `Some`
        // counterpart above, matching the pre-migration wire-baseline capture).
        store::os_store::test_support::assert_op_line_round_trip(&DrawCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None }));
    }

    #[semio_framework_async_macros::async_test]
    async fn draw_command_op_binary_round_trips_every_variant() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🔖️ Pins the exact pre-migration hex for the two rows whose `Option` fields make `None`/`Some`
    /// distinct wire cases — copied verbatim from the `wire-baseline-before.txt` capture taken from
    /// the OLD `draw_protocol` crate before this migration. A byte-for-byte diff, not just a
    /// round-trip law, since round-trip alone would happily pass on a changed-but-consistent format.
    #[semio_framework_async_macros::async_test]
    async fn optional_field_rows_keep_their_pre_migration_bytes() {
        use protocol::OpBinary;
        let engagement_submit_some = DrawCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("Renamed \"layer\"".into()) });
        assert_eq!(engagement_submit_some.encode_op().expect("encode"), hex_bytes("0105010f52656e616d656420226c617965722201000600"));
        let engagement_submit_none = DrawCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None });
        assert_eq!(engagement_submit_none.encode_op().expect("encode"), hex_bytes("01050000"));
    }

    async fn hex_bytes(hex: &str) -> Vec<u8> {
        (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("valid hex")).collect()
    }

    #[semio_framework_async_macros::async_test]
    async fn every_command_row_prints_starting_with_its_wire_keyword() {
        use protocol::OpText;
        let expected_keywords = [
            "set-snapshot",
            "commit-document",
            "fixture-json",
            "active-example",
            "selected-opacity",
            "engagement-submit",
            "add-layer",
            "drop-layer-kind",
            "move-layer",
            "delete-layer",
            "duplicate-layer",
            "toggle-layer-visible",
            "combine-boolean",
            "patch-layer",
            "patch-layers",
            "active-utility",
            "camera",
            "camera-zoom",
            "engagement-input",
            "locale",
            "canvas-pointer-down",
            "canvas-pointer-move",
            "canvas-pointer-up",
            "canvas-double-click",
            "canvas-commit-draft",
            "canvas-escape",
        ];
        for (command, keyword) in every_command().into_iter().zip(expected_keywords) {
            let printed = command.print_op();
            assert!(printed.starts_with(keyword), "expected '{printed}' to start with '{keyword}'");
        }
    }
    //#endregion 🔖️WireGuards
}
//#endregion 🧪️Tests
