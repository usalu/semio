//! 🖥️ Drawing editor surface — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum
//! and the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window
//! render in `🎭️modes/✏️edit/🪟️windows/🖼️canvas`, panel trees in `📌️panels/*`, labels in
//! `🦀️terminology.rs`, view state in `🦀️config.rs`.
//! This file is a routing table: `handle` → `DrawingCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
use semio_s_artifact_draw_drawing_editor_interaction::DRAWING_DEFAULT_UTILITY;
use semio_s_artifact_draw_drawing_editor_interaction::DRAWING_INTERACTION_DOMAIN;
use semio_s_artifact_draw_drawing_editor_interaction::DRAWING_INTERACTION_GRANULARITY;
use semio_s_artifact_draw_drawing_editor_interaction::DRAWING_POINT_DOMAIN;
use semio_s_artifact_draw_drawing_editor_interaction::drawing_manifest_action;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use semio_s_artifact_draw_drawing_editor_commands::nudge_selection::{nudge_selection_left, nudge_selection_left_fast, nudge_selection_right, nudge_selection_right_fast, nudge_selection_up, nudge_selection_up_fast, nudge_selection_down, nudge_selection_down_fast};
use semio_s_artifact_draw_drawing_editor_tool::{DrawingGesturePreview, DrawingSession};
use semio_s_artifact_draw_drawing_editor_commands::{import_image, add_layer, combine_boolean, commit_document, delete_layer, delete_selection, drop_layer_kind, duplicate_layer, engagement_input, engagement_submit, export_document, edit_selection, edit_path, edit_fill, move_layer, patch_layer, patch_layers, set_active_example, set_camera, set_camera_zoom, load_document_json, set_selected_opacity, set_snapshot, toggle_layer_visible};
use semio_s_artifact_draw_drawing_editor_tool::{canvas_commit_draft, canvas_double_click, canvas_escape, canvas_pointer_move, canvas_pointer_up};
use semio_s_artifact_draw_drawing_editor_tool as canvas_pointer_down;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas as canvas_window;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas::config::DrawingCanvasWindowConfig;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas::transient::DrawingCanvasWindowTransient;
use semio_s_artifact_draw_drawing_editor_interaction::panels::{catalogue as catalogue_panel, layers as layers_panel, properties as properties_panel};
use semio_s_artifact_draw_drawing_editor_interaction::presence::{DrawingPresence, DrawingPresenceMutation};
use semio_s_artifact_draw_drawing_editor_interaction::terminology::DrawingPlayLabels;
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_scene::identity::SceneIdentity;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::DRAWING_DOCUMENT_SCHEMA;
use semio_framework_job_registry::{FixedOperationOwner};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
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
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowEngagementInput;
use semio_framework_plugin::WindowEngagementStatus;
use pack::snapshot::ArtifactPack;
use semio_framework_2d::compute::EngineHandles;
use std::collections::HashMap;

use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas::DRAWING_PLAY_BODY_COMPOSITE;
use semio_s_artifact_draw_drawing_editor_tool::modes::edit::windows::canvas::DRAWING_PLAY_WINDOW_CANVAS;
use semio_s_artifact_draw_drawing_editor_interaction::panels::catalogue::DRAWING_PLAY_BODY_CATALOGUE;
use semio_s_artifact_draw_drawing_editor_interaction::panels::layers::DRAWING_PLAY_BODY_LAYERS;
use semio_s_artifact_draw_drawing_editor_interaction::panels::properties::DRAWING_PLAY_BODY_PROPERTIES;

//#region 🔖️Constants

/// 🧰️ The utility armed for THIS window. The React host keeps utilities per window
/// (`ViewModel::active_utility_by_window_id`, keyed by window instance id) and only mirrors the
/// ACTIVE window's utility into the flat `active_utility_id`; a gesture dispatched into a pane that
/// is not the shell's active window therefore carried `None` and every rectangle drag ran as a
/// marquee select (ticket 26/09/05/DRAW-PLUGIN-END-TO-END, 2026-09-17). Resolution order: the
/// addressed window's entry, the focused window's entry, the flat field, the default.
fn drawing_active_utility(view: &semio_framework_plugin::ViewModel) -> &str {
    view.window_id
        .as_deref()
        .and_then(|window| view.active_utility_by_window_id.get(window))
        .or_else(|| view.focused_window_id.as_deref().and_then(|window| view.active_utility_by_window_id.get(window)))
        .map(String::as_str)
        .filter(|utility| !utility.is_empty())
        .or(view.active_utility_id.as_deref())
        .unwrap_or(DRAWING_DEFAULT_UTILITY)
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `DrawingPlayApp::Command` — the SOLE dispatch surface for drawing's own behavior, covering every
    /// action `create_drawing_app` declares. Field shapes mirror each action's real `args` object.
    /// **Row order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum DrawingCommand for DrawingSnapshot, DrawingMutation, NoConfig, NoConfigMutation, ctx = DrawingSession {
        "setSnapshot" as "set-snapshot" => set_snapshot::SetSnapshot,
        "commitDocument" as "commit-document" => commit_document::CommitDocument,
        "loadDocumentJson" as "document-json" => load_document_json::LoadDocumentJson,
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
        "exportDocument" as "export-document" => export_document::ExportDocument,
        "editSelection" as "edit-selection" => edit_selection::EditSelection,
        "editPath" as "edit-path" => edit_path::EditPath,
        "editFill" as "edit-fill" => edit_fill::EditFill,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "nudgeSelectionLeft" as "nudge-selection-left" => nudge_selection_left::NudgeSelectionLeft,
        "nudgeSelectionLeftFast" as "nudge-selection-left-fast" => nudge_selection_left_fast::NudgeSelectionLeftFast,
        "nudgeSelectionRight" as "nudge-selection-right" => nudge_selection_right::NudgeSelectionRight,
        "nudgeSelectionRightFast" as "nudge-selection-right-fast" => nudge_selection_right_fast::NudgeSelectionRightFast,
        "nudgeSelectionUp" as "nudge-selection-up" => nudge_selection_up::NudgeSelectionUp,
        "nudgeSelectionUpFast" as "nudge-selection-up-fast" => nudge_selection_up_fast::NudgeSelectionUpFast,
        "nudgeSelectionDown" as "nudge-selection-down" => nudge_selection_down::NudgeSelectionDown,
        "nudgeSelectionDownFast" as "nudge-selection-down-fast" => nudge_selection_down_fast::NudgeSelectionDownFast,
        "importImage" as "import-image" => import_image::ImportImage,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.

//#endregion 🔖️Commands

//#region 🌉️ActionBridge
/// 🌉️ Host action `{action, args}` → typed `DrawingCommand` (ticket 26/09/05/DRAW-PLUGIN-END-TO-END,
/// after forms' bridge). The React/wgpu shells send camelCase argument keys and JSON numbers; every
/// `🎮️commands/*` payload derives `FromValue` over its own snake_case field names, so this boundary
/// folds each key into both spellings, restores integral floats, prints structured `value`/`json`
/// arguments into the `String` wire fields that carry JSON text, and decodes. Without it the trait
/// default refuses every app action (`setActiveExample`, `addLayer`, …) as "not framework-reserved".
//#endregion 🌉️ActionBridge

//#region 🧵️GestureOperationJobs
const DRAWING_GESTURE_TOOL_IDS: &[&str] = &["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "canvasCommitDraft", "canvasEscape"];
const DRAWING_GESTURE_RAW_BYTES: usize = 8_192;
const DRAWING_GESTURE_RETAINED_BYTES: usize = 131_072;

/// 🎟️ One release turn that can retire a gesture owner's declared retained budget.
fn drawing_gesture_release_grant() -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant::one_release_turn(DRAWING_GESTURE_RETAINED_BYTES, 32)
}

/// 🛣️ One publication lane row per gesture route, read off each route's real `Emit` construction, not
/// off its `ActionKind`: every gesture that reaches a commit does so through the canvas tool's one
/// `Emit::commit_transaction` (`drawing_tool_emit`, artifact lane), and the two routes that
/// additionally write the exact Canvas `WindowTransient` snapshot — `canvasPointerDown` through
/// `advance_trace_pointer`, `canvasEscape` through its own trace cancellation — carry the config lane
/// as well. Under-declaring a lane that is actually emitted faults at publication time.
const DRAWING_GESTURE_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasDoubleClick", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasCommitDraft", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasEscape", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
];

struct DrawingGestureOperationOwner {
    session: Option<DrawingSession>,
    closing: bool,
    /// 🧾️ The declared `DRAWING_GESTURE_RETAINED_BYTES` budget still to be handed back to the
    /// registry. The session itself is dropped on the first granted close step; its budget is
    /// released one grant-sized page per step, because every framework close/maintenance pump grants
    /// at most `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` (4 KiB) — demanding the whole 32 KiB in one step
    /// meant no gesture owner ever closed outside a hand-rolled test (ticket
    /// 26/09/05/DRAW-PLUGIN-END-TO-END, 2026-09-17).
    unreleased_bytes: usize,
}

impl DrawingGestureOperationOwner {
    fn new(active_utility_id: &str, authoring_seed: &str) -> Self {
        Self { session: Some(DrawingSession::new(active_utility_id, authoring_seed)), closing: false, unreleased_bytes: DRAWING_GESTURE_RETAINED_BYTES }
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

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_release_bytes;
        if !self.closing || maximum_items == 0 || maximum_bytes == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        let released_items = usize::from(self.session.take().is_some());
        let released_bytes = self.unreleased_bytes.min(maximum_bytes);
        self.unreleased_bytes -= released_bytes;
        if released_items == 0 && released_bytes == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Complete {progress:Default::default()};
        }
        semio_framework_job::InteractiveJobCloseStep::Pending { progress:semio_framework_value::retained_clone::RetainedCloneProgress {copied_items:released_items,released_bytes,..Default::default()} }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.session.is_none() && self.unreleased_bytes == 0
    }
}

const DRAWING_GESTURE_OPERATION_SLOTS: usize = 64;

struct DrawingInstanceOperationOwner {
    operations: semio_framework_job_registry::FixedOperationRegistry<DrawingGestureOperationOwner, DRAWING_GESTURE_OPERATION_SLOTS>,
    active: Option<(semio_framework_job_registry::FixedOperationKey, SceneIdentity)>,
    closing: bool,
    /// 🚪️ Every slot has been marked closing. `FixedOperationRegistry::begin_close_step` and
    /// `close_step` share one cursor and each advance it by one, so a close that calls both per step
    /// only ever begins the close of every OTHER slot — an owner on the wrong parity was never marked
    /// and its close spun `Pending { 0, 0 }` until the app's Drop witness fired (ticket
    /// 26/09/05/DRAW-PLUGIN-END-TO-END, 2026-09-17). The first close step now sweeps all slots once.
    close_begun: bool,
}

impl DrawingInstanceOperationOwner {
    fn new() -> Self {
        Self { operations: semio_framework_job_registry::FixedOperationRegistry::new(DRAWING_GESTURE_OPERATION_SLOTS * DRAWING_GESTURE_RETAINED_BYTES), active: None, closing: false, close_begun: false }
    }

    fn dispatch(&mut self, payload: &DrawingGestureOperationPayload, context:&mut semio_framework_job::StepContext<'_>) -> Result<Option<(Emit<DrawingMutation, NoConfigMutation, NoDraftMutation>, DrawingCanvasWindowTransient)>, Fault> {
        let key = semio_framework_job_registry::FixedOperationKey::new(semio_framework_job::OperationId(payload.operation_context.operation_id), semio_framework_job::Generation(payload.operation_context.generation));
        let source_identity = payload.source_identity;
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
            if !observed_revision.matches(source_identity) || !same_utility {
                self.operations.cancel(active);
                self.active = None;
            }
        }
        let live_key = self.active.map_or(key, |(active, _)| active);
        if self.operations.get(live_key).is_none() {
            // 🧹️ The registry is direct-mapped by `operation_id % slots` and the framework mints every
            // tool operation id INTO the first vacant residue class of its own 64-slot table — so the
            // next gesture reuses residue 0 the moment the previous one settled, and lands on the slot
            // still held by that gesture's cancelled-but-not-yet-retired owner. Retire the retiring
            // owners now (bounded: one cursor sweep, each owner closes within its declared grant)
            // instead of answering `saturated` (every browser gesture failed so, 2026-09-17).
            if !self.operations.can_admit(live_key, DRAWING_GESTURE_RETAINED_BYTES) {
                for _ in 0..DRAWING_GESTURE_OPERATION_SLOTS * 2 {
                    if self.operations.can_admit(live_key, DRAWING_GESTURE_RETAINED_BYTES) {
                        break;
                    }
                    let _ = self.operations.close_step(drawing_gesture_release_grant());
                }
            }
            self.operations.admit(live_key, DrawingGestureOperationOwner::new(active_utility_id, &operation.authoring_seed)).map_err(|mut rejected| {
                rejected.owner.cancel();
                rejected.owner.begin_close();
                let _ = rejected.owner.close_step(drawing_gesture_release_grant());
                Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.saturated"), "the fixed Drawing gesture operation authority is saturated")
            })?;
            self.active = Some((live_key, source_identity));
        }
        let retained = self.operations.get_mut(live_key).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.owner"), "the exact Drawing gesture owner changed before its bounded reducer step"))?;
        let session = retained.session.as_mut().ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.owner"), "the Drawing gesture session is already closing"))?;
        session.identity_cancel=Some(context.cancel_token());
        session.source_identity=Some(source_identity);
        session.window_config = payload.window_config.clone();
        session.window_transient = payload.window_transient.clone();
        session.base = Some(canvas_pointer_down::DrawingToolBase { document: payload.snapshot.clone(), operation: Some(operation.clone()) });
        if session.tool.context().points_overflowed {
            self.operations.cancel(live_key);
            self.active = None;
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.point-capacity"), "the fixed Drawing gesture point capacity was exceeded"));
        }
        if let Some(query) = session.point_query.as_mut() {
            if query.command_id != command.command_id() {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.query-owner"), "a retained Drawing point query rejects a different command owner"));
            }
            semio_s_artifact_draw_drawing_editor_interaction::geometry_session::prepare_query(source_identity,snapshot);
            let admitted=semio_s_artifact_draw_drawing_editor_interaction::geometry_session::with_query(source_identity,|status,borrowed|match status {
                semio_s_artifact_draw_drawing_scene::identity::admission::SceneAdmissionStatus::Pending=>Ok(None),
                semio_s_artifact_draw_drawing_scene::identity::admission::SceneAdmissionStatus::Ready=>{
                    let borrowed=borrowed.unwrap();
                    if query.traversal_complete{query.cursor.validate_cache(&borrowed)?;Ok(Some(true))}
                    else{Ok(Some(query.cursor.advance(snapshot,&borrowed)))}
                },
                _=>Err(Fault::from("Drawing pointer geometry is unavailable or changed")),
            });
            let admitted=match admitted{Ok(admitted)=>admitted,Err(fault)=>{self.operations.cancel(live_key);self.active=None;return Err(fault)}};
            let Some(complete)=admitted else{return Ok(None)};
            if !query.traversal_complete {
                if !complete {
                    return Ok(None);
                }
                if query.cursor.overflowed {
                    let error=query.cursor.failure.clone().unwrap_or_else(||"Drawing query exceeds result capacity".into());
                    session.point_query = None;
                    self.operations.cancel(live_key);
                    self.active = None;
                    return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.query"), error));
                }
                query.traversal_complete = true;
                return Ok(None);
            }
            let ids=payload.interaction_state.selection.get(DRAWING_INTERACTION_DOMAIN).map(|selection|selection.ids.as_slice()).unwrap_or(&[]);
            let point_ids=payload.interaction_state.selection.get(DRAWING_POINT_DOMAIN).map(|selection|selection.ids.as_slice()).unwrap_or(&[]);
            match session.prepare_grab(snapshot,ids,point_ids) {
                Ok(false)=>return Ok(None),
                Err(fault)=>{ self.operations.cancel(live_key); self.active=None; return Err(fault); },
                Ok(true)=>{},
            }
            if session.point_query.as_ref().is_some_and(|query|query.constrained) && session.tool.matches("pressing") && session.node_marquee.is_none() {
                session.escape()?;
            }
            let query=session.point_query.as_mut().expect("retained point query");
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
            let mut emit = Emit::default();
            if query.node_selection.is_some() {emit.effects.push(canvas_pointer_down::point_selection_effect_from_targets(targets));}
            else if !query.preserve_selection {
                emit.effects.push(if query.hover {canvas_pointer_down::interaction_hover_effect_from_targets(targets)}else {canvas_pointer_down::interaction_select_effect_from_targets(targets,&query.merge)});
            }
            let window_transient = session.window_transient.clone();
            if session.tool.at_rest() && session.trace_pointer.is_none() {
                self.operations.cancel(live_key);
                self.active = None;
            }
            return Ok(Some((emit, window_transient)));
        }
        if let DrawingCommand::CanvasPointerDown(pointer) = command {
            if (active_utility_id=="editNodes" || active_utility_id=="selectDirect" && !pointer.ctrl && !pointer.meta) && pointer.generation.is_none() {
                let (x,y) = canvas_pointer_down::canvas_point_to_world(&session.window_config.viewport,pointer.x,pointer.y,pointer.width,pointer.height);
                let world = [x,y];
                session.press(canvas_pointer_down::DrawingPointer { utility:active_utility_id.into(),world,shift:pointer.shift,alt:pointer.alt,ctrl:pointer.ctrl,meta:pointer.meta })?;
                let tolerance = canvas_pointer_down::DRAWING_PICK_TOLERANCE_PX/session.window_config.viewport.zoom.max(1e-6);
                let mut query = canvas_pointer_down::DrawingPointQuery::new(command.command_id(),canvas_pointer_down::TracePointerJob::new_query(snapshot,world,tolerance,false),false,"replace".into(),false);
                query.cursor.node_editing=active_utility_id=="editNodes";
                query.point_pick_mode=if pointer.shift {semio_s_artifact_draw_drawing_editor_interaction::points::PointPickMode::Toggle} else if pointer.ctrl || pointer.meta {semio_s_artifact_draw_drawing_editor_interaction::points::PointPickMode::Add} else {semio_s_artifact_draw_drawing_editor_interaction::points::PointPickMode::Replace};
                query.constrained=pointer.shift;
                query.centered=pointer.alt;
                query.cursor.retain_selection_bounds(payload.interaction_state.selection.get(DRAWING_INTERACTION_DOMAIN).map(|selection|selection.ids.as_slice()).unwrap_or(&[]))?;
                query.drag_start = Some(world);
                session.point_query = Some(query);
                return Ok(None);
            }
        }
        if let DrawingCommand::CanvasPointerMove(payload) = command {
            if session.tool.at_rest() {
                // 🧵️ Idle hover hit-tests the LAST sample of a batch only (design L4 / §2 D).
                let [x, y] = payload.last_sample();
                let (world_x, world_y) = canvas_pointer_down::canvas_point_to_world(&session.window_config.viewport, x, y, payload.width, payload.height);
                let tolerance = canvas_pointer_down::DRAWING_PICK_TOLERANCE_PX / session.window_config.viewport.zoom.max(1e-6);
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
        let document_owner = payload.snapshot.clone();
        let retained_emit = match command {
            DrawingCommand::CanvasPointerMove(pointer) if session.tool.matches("pressing") || session.tool.matches("dragging") => {
                let [x,y] = pointer.last_sample();
                let (x,y) = canvas_pointer_down::canvas_point_to_world(&session.window_config.viewport,x,y,pointer.width,pointer.height);
                Some(Some(session.sample([x,y],pointer.shift,pointer.alt)?))
            }
            DrawingCommand::CanvasPointerMove(payload) if session.tool.matches("marqueeing") && session.tool.context().method == "lasso" => Some(session.advance_lasso_move(payload)?),
            // 🚫️ A cancelled release aborts a live gesture with zero trace and selects/commits nothing.
            DrawingCommand::CanvasPointerUp(payload) if payload.cancelled => Some(Some(session.cancel())),
            DrawingCommand::CanvasPointerUp(payload) => {
                let (world_x, world_y) = canvas_pointer_down::canvas_point_to_world(&session.window_config.viewport, payload.x, payload.y, payload.width, payload.height);
                let pointer = canvas_pointer_down::DrawingPointer { utility: session.active_utility_id.clone(), world: [world_x, world_y], shift: payload.shift, alt: payload.alt, ctrl: payload.ctrl, meta: payload.meta };
                let base = canvas_pointer_down::DrawingToolBase { document: document_owner, operation: Some(operation.clone()) };
                Some(session.release(command.command_id(), pointer, base)?)
            }
            DrawingCommand::CanvasDoubleClick(_) | DrawingCommand::CanvasCommitDraft(_) => {
                let base = canvas_pointer_down::DrawingToolBase { document: document_owner, operation: Some(operation.clone()) };
                Some(Some(session.finish_draft(base)?))
            }
            _ => None,
        };
        if let Some(retained_emit) = retained_emit {
            let Some(emit) = retained_emit else { return Ok(None) };
            let window_transient = session.window_transient.clone();
            if session.tool.at_rest() && session.trace_pointer.is_none() && session.point_query.is_none() {
                self.operations.cancel(live_key);
                self.active = None;
            }
            return Ok(Some((emit, window_transient)));
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
        if session.tool.context().points_overflowed {
            self.operations.cancel(live_key);
            self.active = None;
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.point-capacity"), "the fixed Drawing gesture point capacity was exceeded"));
        }
        let window_transient = session.window_transient.clone();
        if session.tool.at_rest() && session.trace_pointer.is_none() {
            self.operations.cancel(live_key);
            self.active = None;
        }
        Ok(Some((emit, window_transient)))
    }

    fn preview_projection(&mut self, source_identity: SceneIdentity, active_utility: &str) -> Option<DrawingGesturePreview> {
        let (key, observed_revision) = self.active?;
        if !observed_revision.matches(source_identity) {
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
        if session.tool.at_rest() && session.trace_pointer.is_none() && session.point_query.is_none() {
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

    fn retirement_demands(&self, _body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(if self.operations.is_empty() { Default::default() } else { semio_framework_value::RetirementDemand { release_bytes: DRAWING_GESTURE_RAW_BYTES, depth: 4, ..Default::default() } })
    }

    fn maintenance_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        for _ in 0..DRAWING_GESTURE_OPERATION_SLOTS {
            match self.operations.close_step(grant) {
                semio_framework_job::InteractiveJobCloseStep::Pending { progress } if progress == Default::default() && !self.operations.is_empty() => continue,
                semio_framework_job::InteractiveJobCloseStep::Blocked => return Ok(semio_framework_plugin::PluginLifecycleStep::Blocked { reason: "Drawing gesture close owner awaits a non-empty grant" }),
                semio_framework_job::InteractiveJobCloseStep::Pending { progress } => return Ok(semio_framework_plugin::PluginLifecycleStep::Progress(progress)),
                semio_framework_job::InteractiveJobCloseStep::Complete { progress } => return Ok(semio_framework_plugin::PluginLifecycleStep::Complete(progress)),
                semio_framework_job::InteractiveJobCloseStep::Refused { kind, .. } => return Err(Fault::from(format!("Drawing gesture retirement refused: {kind:?}"))),
            }
        }
        Ok(semio_framework_plugin::PluginLifecycleStep::Progress(Default::default()))
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        self.closing = true;
        if !self.close_begun {
            for _ in 0..DRAWING_GESTURE_OPERATION_SLOTS {
                self.operations.begin_close_step();
            }
            self.close_begun = true;
        }
        self.maintenance_step(cx,grant)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.operations.is_empty()
    }
}

struct DrawingGestureOperationPayload {
    source_identity: SceneIdentity,
    command: DrawingCommand,
    snapshot: std::sync::Arc<DrawingSnapshot>,
    config: std::sync::Arc<NoConfig>,
    window_config: DrawingCanvasWindowConfig,
    window_transient: DrawingCanvasWindowTransient,
    view_state: semio_framework_plugin::ViewModel,
    history: std::sync::Arc<semio_framework_plugin::HistoryView>,
    interaction_state: std::sync::Arc<protocol::InteractionState>,
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    operation_context: semio_framework_plugin::AppOperationContext,
    active_utility_id: String,
    completion: semio_framework_plugin::ArtifactToolCompletion<semio_framework_plugin::EditorApp<DrawingPlayApp>>,
}

/// 📦️ Validates the retained wire pages of one gesture against the typed command the host already
/// decoded: the pages are the command's OWN binary op encoding (`app_commands!` `OpBinary`, the same
/// bytes `ArtifactRetainedCommandJob` decodes for the bounded lane — since 09-15 the host crosses a
/// pack, not `["verb", {...}]` JSON text). Fail-closed exactly like the framework lane: the bytes
/// must decode to a command whose id is the registered verb, so truncation and a variant swap reject
/// before dispatch. No byte-equal re-encode: the host's pack encoder is not obliged to emit the
/// Rust encoder's canonical layout (float/int spelling, defaulted fields).
struct DrawingRetainedCommandDecoder {
    expected_verb: &'static str,
    raw: Vec<u8>,
    overflow: bool,
}

impl DrawingRetainedCommandDecoder {
    fn new(expected_verb: &'static str) -> Self {
        Self { expected_verb, raw: Vec::new(), overflow: false }
    }

    /// 📄️ Appends one wire page within `DRAWING_GESTURE_RAW_BYTES`; an oversized owner fails closed.
    fn feed_page(&mut self, page: &[u8]) -> bool {
        if self.overflow || self.raw.len().checked_add(page.len()).is_none_or(|end| end > DRAWING_GESTURE_RAW_BYTES) || self.raw.try_reserve_exact(page.len()).is_err() {
            self.overflow = true;
            return false;
        }
        self.raw.extend_from_slice(page);
        true
    }

    fn finish(&self) -> bool {
        if self.overflow {
            return false;
        }
        let Ok(decoded) = <DrawingCommand as ::protocol::OpBinary>::decode_op(&self.raw) else { return false };
        decoded.command_id() == self.expected_verb
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
    outcomes: semio_s_artifact_draw_drawing_host::outcome::DrawingJobOutcomes,
}

impl DrawingGestureOperationJob {
    fn turn(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_s_artifact_draw_drawing_host::outcome::DrawingJobTurn {
        use semio_s_artifact_draw_drawing_host::outcome::DrawingJobTurn;
        if self.outcomes.faulted() {
            return DrawingJobTurn::Fault;
        }
        if context.is_cancelled() {
            return DrawingJobTurn::Cancelled;
        }
        if context.should_yield() || context.fuel_remaining() == 0 {
            return DrawingJobTurn::Yield;
        }
        if !self.raw_validated {
            let Some(input) = self.raw_input.as_ref() else { return self.outcomes.fault("drawing.gesture-wire-input-missing") };
            if let Some(page) = input.page(self.raw_page_cursor) {
                let Some(decoder) = self.decoder.as_mut() else {
                    return self.outcomes.fault("drawing.gesture-wire-decoder-missing");
                };
                if !decoder.feed_page(page) {
                    return self.outcomes.fault("drawing.gesture-wire-oversized");
                }
                self.raw_page_cursor += 1;
                self.raw_byte_cursor = 0;
                context.consume_fuel(page.len().max(1) as u64);
                return DrawingJobTurn::Yield;
            }
            let exact = self.decoder.as_ref().is_some_and(DrawingRetainedCommandDecoder::finish);
            if !exact {
                return self.outcomes.fault("drawing.gesture-wire-command-mismatch");
            }
            self.raw_validated = true;
            context.consume_fuel(1);
            return DrawingJobTurn::Yield;
        }
        if self.pending_completion_rejection.is_some() {
            return self.outcomes.fault("drawing.gesture-completion-rejected");
        }
        if !self.completed {
            let Some(payload) = self.payload.as_ref() else { return self.outcomes.fault("drawing.gesture-payload-missing") };
            let emit = payload.instance_owner.with_mut::<DrawingInstanceOperationOwner, _>(|owner| owner.dispatch(payload,context));
            let (emit, transient) = match emit {
                Ok(Some(output)) => output,
                Ok(None) => {
                    context.consume_fuel(1);
                    return DrawingJobTurn::Yield;
                }
                Err(error) => {
                    if let Err(rejected) = payload.completion.complete(Err(error), semio_framework_plugin::EphemeralEmit::default()) {
                        self.pending_completion_rejection = Some(rejected);
                    }
                    return self.outcomes.fault("drawing.gesture-dispatch-refused");
                }
            };
            let window_transient = (transient != payload.window_transient)
                .then(|| canvas_window::transient::addressed(&payload.view_state, transient))
                .transpose();
            let ephemeral = match window_transient {
                Ok(Some(mutation)) => semio_framework_plugin::EphemeralEmit { window_transient: vec![mutation], ..Default::default() },
                Ok(None) => semio_framework_plugin::EphemeralEmit::default(),
                Err(error) => {
                    if let Err(rejected) = payload.completion.complete(Err(error), semio_framework_plugin::EphemeralEmit::default()) {
                        self.pending_completion_rejection = Some(rejected);
                    }
                    return self.outcomes.fault("drawing.gesture-transient-refused");
                }
            };
            if let Err(rejected) = payload.completion.complete(Ok(emit), ephemeral) {
                self.pending_completion_rejection = Some(rejected);
                return self.outcomes.fault("drawing.gesture-completion-rejected");
            }
            self.completed = true;
            context.consume_fuel(1);
        }
        DrawingJobTurn::Complete
    }

    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let unit = semio_framework_value::RetirementDemand { depth: 1, ..Default::default() };
        if !self.outcomes.terminal_is_empty() {
            return self.outcomes.retirement_demands();
        }
        if let Some(rejected) = self.pending_completion_rejection.as_ref() {
            if let Ok(emit) = rejected.emit.as_ref() {
                if let Some(demand) = emit.child_close_demands(body)? {
                    return Ok(demand);
                }
            }
            return Ok(unit);
        }
        if let Some(input) = self.raw_input.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: input.next_close_copy_byte_demand()?, capacity_bytes: input.next_close_capacity_byte_demand(body)?, release_bytes: input.next_close_release_byte_demand()?, depth: input.next_close_depth_demand()? });
        }
        if self.payload.is_some() {
            return Ok(unit);
        }
        if let Some(decoder) = self.decoder.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: decoder.raw.capacity(), depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }
}

impl semio_framework_job::InteractiveJob for DrawingGestureOperationJob {
    fn step<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        let turn = self.turn(context);
        self.outcomes.lend(turn, context)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outcomes.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(input) = self.raw_input.as_mut() {
            input.begin_close();
        }
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_release_bytes;
        let pending=|copied_items,released_bytes|semio_framework_job::InteractiveJobCloseStep::Pending {progress:semio_framework_value::retained_clone::RetainedCloneProgress {copied_items,released_bytes,..Default::default()}};
        if !self.closing {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if !self.outcomes.terminal_is_empty() {
            return self.outcomes.close_job_step(grant);
        }
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            if let Ok(emit) = rejected.emit.as_mut() {
                match emit.close_child_one(grant) {
                    Ok(Some(semio_framework_plugin::PluginLifecycleStep::Progress(progress) | semio_framework_plugin::PluginLifecycleStep::Complete(progress))) => return semio_framework_job::InteractiveJobCloseStep::Pending { progress },
                    Ok(Some(semio_framework_plugin::PluginLifecycleStep::Blocked { .. } | semio_framework_plugin::PluginLifecycleStep::AwaitingInput { .. })) => return semio_framework_job::InteractiveJobCloseStep::Blocked,
                    Ok(None) => {}
                    Err(_) => return semio_framework_job::InteractiveJobCloseStep::Refused { kind: semio_framework_value::ValueRefusalKind::InvariantViolated, progress: Default::default() },
                }
            }
            if maximum_items == 0 {
                return pending(0,0);
            }
            self.pending_completion_rejection = None;
            return pending(1,0);
        }
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if let Some(input) = self.raw_input.as_mut() {
            let step = input.close_step(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,..grant});
            if input.terminal_is_empty() {
                self.raw_input = None;
            }
            return match step {
                semio_framework_job::InteractiveJobCloseStep::Complete {progress} => semio_framework_job::InteractiveJobCloseStep::Pending {progress},
                other => other,
            };
        }
        if self.payload.take().is_some() {
            return pending(1,0);
        }
        if let Some(decoder)=self.decoder.as_mut() {
            let bytes=decoder.raw.capacity();
            if bytes>maximum_bytes {return pending(0,0);}
            drop(std::mem::take(&mut decoder.raw));self.decoder=None;
            return pending(1,bytes);
        }
        semio_framework_job::InteractiveJobCloseStep::Complete {progress:Default::default()}
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,semio_framework_value::ValueError> {Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(self.close_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(self.close_demands(0)?.depth)}

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.pending_completion_rejection.is_none() && self.payload.is_none() && self.raw_input.is_none() && self.decoder.is_none() && self.outcomes.terminal_is_empty()
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
        Ok(DrawingGestureOperationJob { payload: Some(payload), pending_completion_rejection: None, raw_input: None, raw_page_cursor: 0, raw_byte_cursor: 0, decoder: None, raw_validated: true, completed: false, closing: false, outcomes: semio_s_artifact_draw_drawing_host::outcome::DrawingJobOutcomes::new() })
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
    "deleteSelection",
    "nudgeSelectionLeft",
    "nudgeSelectionLeftFast",
    "nudgeSelectionRight",
    "nudgeSelectionRightFast",
    "nudgeSelectionUp",
    "nudgeSelectionUpFast",
    "nudgeSelectionDown",
    "nudgeSelectionDownFast",
    "setSnapshot",
    "commitDocument",
    "loadDocumentJson",
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
    "editSelection",
    "editFill",
];
const DRAWING_EXPORT_TOOL_IDS: &[&str] = &["exportDocument"];
const DRAWING_CLIPBOARD_RESERVED_IDS: &[&str] = &["copy","cut","paste"];
const DRAWING_EXPORT_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "exportDocument", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
];
#[path="🎮️commands/📤️export-document/🧵️export/🦀️.rs"]
mod export_job;
use export_job::DrawingExportCommandJobFactory;
const DRAWING_IMAGE_RAW_BYTES:usize=89_481_584;
const DRAWING_IMAGE_TOOL_IDS: &[&str] = &["importImage"];
const DRAWING_IMAGE_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importImage", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Interaction] },
];
#[path="🎮️commands/📥️import-image/🧵️admission/🦀️.rs"]
mod image_job;
use image_job::DrawingImageCommandJobFactory;
const DRAWING_PATH_TOOL_IDS: &[&str] = &["editPath"];
const DRAWING_PATH_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "editPath", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
];
#[path="🎮️commands/✏️edit-path/🧵️simplify/🦀️.rs"]
mod path_job;
use path_job::DrawingPathCommandJobFactory;
const DRAWING_BOUNDED_PAYLOAD_SCHEMA: &str = "drawing.tool-command.v1";
const DRAWING_BOUNDED_RAW_BYTES: usize = 65_536;
const DRAWING_BOUNDED_WORK_ITEMS: usize = 4_096;

/// 🛣️ One publication lane row per bounded route, read off each handler's real `Emit` construction
/// (`🎮️commands/*/🦀️.rs`). The four whole-document routes emit nothing but an `Effect::LoadDocument`
/// — effects are not a store lane, so those are honestly `HostOnly`; the layer routes emit
/// `DrawingMutation`s; the view routes emit exact Canvas window mutations.
const DRAWING_BOUNDED_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionLeft", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionLeftFast", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionRight", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionRightFast", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionUp", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionUpFast", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionDown", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nudgeSelectionDownFast", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSnapshot", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "commitDocument", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "loadDocumentJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSelectedOpacity", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "dropLayerKind", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "duplicateLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleLayerVisible", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "editSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "editFill", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "combineBoolean", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchLayers", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCameraZoom", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
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

struct DrawingWindowCommandWork {
    tool_id: &'static str,
    completed: bool,
}

impl DrawingWindowCommandWork {
    fn new(tool_id: &'static str) -> Self { Self { tool_id, completed: false } }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<DrawingPlayApp>> for DrawingWindowCommandWork {
    fn terminal_frame_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
    fn tool_id(&self) -> &'static str { self.tool_id }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<DrawingPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }

    fn extent(
        &self,
        command: &DrawingCommand,
        snapshot: &DrawingSnapshot,
        interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<DrawingPlayApp>>>,
    ) -> Option<usize> {
        if self.completed || command.command_id() != self.tool_id || drawing_bounded_extent(command, snapshot, interaction) != Some(1) { return None; }
        match command {
            DrawingCommand::SetCamera(_) | DrawingCommand::SetCameraZoom(_) | DrawingCommand::EngagementInput(_) => context?.view_state.as_ref().map(|_| 1),
            _ => Some(1),
        }
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<DrawingPlayApp>>,
    cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<DrawingPlayApp>>, Fault> {
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || input.command.command_id() != self.tool_id { return Err(Fault::from("drawing-window-work-terminal")); }
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: input.context.and_then(|context| context.window_config.as_ref()) };
        let active_utility = input.context.and_then(|context| context.view_state.as_ref()).map_or(DRAWING_DEFAULT_UTILITY, drawing_active_utility);
        let mut session = DrawingSession::new(active_utility, &input.operation.authoring_seed);
        session.identity_cancel=Some(cx.cancel_token());
        session.interaction.ids = input.interaction.selection.get(DRAWING_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default();
        session.interaction.points = input.interaction.selection.get(DRAWING_POINT_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default();
        session.window_config = canvas_window::config::from_snapshot(input.context.and_then(|context| context.window_config.as_ref()));
        session.window_transient = canvas_window::transient::from_snapshot(input.context.and_then(|context| context.window_transient.as_ref()));
        let mut emit = Emit::default();
        let mut transient = None;
        match input.command {
            DrawingCommand::SetCamera(payload) => {
                payload.camera.validate().map_err(|error| Fault::from(error.to_string()))?;
                let view = input.context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("drawing-canvas-window-required"))?;
                let mut config = session.window_config;
                config.viewport = payload.camera;
                config.framed = true;
                emit.window_config_mutations.push(canvas_window::config::addressed(view, config)?);
            }
            DrawingCommand::SetCameraZoom(payload) => {
                let view = input.context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("drawing-canvas-window-required"))?;
                let mut config = session.window_config;
                config.viewport.zoom = payload.value;
                config.framed = true;
                config.viewport.validate().map_err(|error| Fault::from(error.to_string()))?;
                emit.window_config_mutations.push(canvas_window::config::addressed(view, config)?);
            }
            DrawingCommand::EngagementInput(payload) => {
                let view = input.context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("drawing-canvas-window-required"))?;
                session.window_transient.engagement_input = payload.value.clone();
                transient = Some(canvas_window::transient::addressed(view, session.window_transient)?);
            }
            _ => emit = input.command.dispatch(&doc, &cfg, &mut session)?,
        }
        cx.consume_fuel(session.identity_checkpoints);
        session.identity_cancel=None;
        self.completed = true;
        Ok(match transient {
            Some(mutation) => ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit,
                ephemeral: semio_framework_plugin::EphemeralEmit { window_transient: vec![mutation], ..Default::default() },
            },
            None => ArtifactCommandWorkStep::Complete(emit),
        })
    }
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
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.tool-mismatch"), "bounded Drawing command does not match its exact registered tool or exceeds its declared extent"));
    }
    let tool_id = request.command.command_id();
    let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<DrawingPlayApp>>> = Box::new(DrawingWindowCommandWork::new(tool_id));
    let operation_context = semio_framework_plugin::AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id.clone(),
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
        retained: request.retained,
        authoring_seed: request.authoring_seed.clone(),
    };
    let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::new(
        semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
            command: *request.command,
            snapshot: request.snapshot,
            snapshot_read: request.snapshot_read,
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
    );
    Ok(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation))
}
//#endregion 🧵️BoundedCommands

//#region 📬️StorePreparation

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct DrawingArtifactStorePreparationFactory;

struct DrawingArtifactStorePreparation {
    base: std::mem::ManuallyDrop<Option<store::os_store::snapshot_read::SnapshotRead<DrawingSnapshot>>>,
    mutation: std::mem::ManuallyDrop<Option<DrawingMutation>>,
    inverse: std::mem::ManuallyDrop<Option<Vec<DrawingMutation>>>,
    refused: std::mem::ManuallyDrop<Option<(::protocol::Edit<DrawingMutation>, std::sync::Arc<DrawingSnapshot>)>>,
    apply_refusal: std::mem::ManuallyDrop<Option<::protocol::MutationApplyError>>,
    authority: std::mem::ManuallyDrop<Option<std::sync::Arc<store::os_store::one_item_publication::ArtifactStoreOneItemLiveAuthority>>>,
    prepared: std::mem::ManuallyDrop<Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<DrawingSnapshot, DrawingMutation>>>,
    mutation_retirement: std::mem::ManuallyDrop<Option<std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<DrawingMutation>>>>,
    snapshot_retirement: std::mem::ManuallyDrop<Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<DrawingSnapshot>>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    factories: std::mem::ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>; 2]>,
    checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl DrawingArtifactStorePreparation {
    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "preparation close depth overflow"))?; Ok(demand) };
        if let Some(active) = self.active.as_ref() { return nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?); }
        if self.prepared.is_some() { let birth = store::os_store::one_item_publication::ArtifactStoreOneItemPrepared::<DrawingSnapshot, DrawingMutation>::retirement_birth_demand(); return Ok(semio_framework_value::RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth + 1, ..Default::default() }); }
        if self.mutation.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.mutation)?); }
        if self.inverse.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.inverse)?); }
        if self.refused.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.refused)?); }
        if self.apply_refusal.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.apply_refusal)?); }
        if self.base.is_some() { return nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.base)?); }
        if let Some(authority) = self.authority.as_ref() { let birth = authority.retirement_birth_demand(); return Ok(semio_framework_value::RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth + 1, ..Default::default() }); }
        if self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<std::sync::Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        self.factories.iter().find_map(Option::as_ref).map_or(Ok(Default::default()), |factory| nested(factory.demands(body)?))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<DrawingSnapshot, DrawingMutation> for DrawingArtifactStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<::protocol::Edit<DrawingMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<DrawingMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &DrawingMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("drawing-artifact-lane".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &DrawingMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<DrawingArtifactStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<DrawingSnapshot, DrawingMutation, DrawingMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<DrawingSnapshot, DrawingMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<DrawingSnapshot, DrawingMutation, DrawingMutation>)> {
        let items = request.base.get().layers.len().saturating_add(request.base.get().assets.len());
        if request.lane != store::os_store::store_schema::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || items > DRAWING_BOUNDED_WORK_ITEMS
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "drawing preparation rejected original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        Ok((Box::new(DrawingArtifactStorePreparation {
            base: std::mem::ManuallyDrop::new(Some(request.base)),
            mutation: std::mem::ManuallyDrop::new(Some(request.mutation)),
            inverse: std::mem::ManuallyDrop::new(None),
            refused: std::mem::ManuallyDrop::new(None),
            apply_refusal: std::mem::ManuallyDrop::new(None),
            authority: std::mem::ManuallyDrop::new(Some(request.authority)),
            prepared: std::mem::ManuallyDrop::new(None),
            mutation_retirement: std::mem::ManuallyDrop::new(Some(request.mutation_retirement)),
            snapshot_retirement: std::mem::ManuallyDrop::new(Some(request.snapshot_retirement)),
            active: std::mem::ManuallyDrop::new(None),
            factories: std::mem::ManuallyDrop::new(Default::default()),
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<DrawingSnapshot, DrawingMutation> for DrawingArtifactStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use ::protocol::Mutation as _;
        use semio_framework_value::{ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.refused.is_some() || self.apply_refusal.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal"));
        }
        let progress = semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if self.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, progress));
        }
        let base = self.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "drawing-artifact-base-owner-missing"))?;
        let authority = self.authority.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "drawing-artifact-authority-missing"))?;
        let mutation = self.mutation.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "drawing-artifact-mutation-owner-missing"))?;
        let inverse = match mutation.inverse(base.get()) {
            Ok(inverse) => inverse,
            Err(error) => { *self.mutation = Some(mutation); return Err(error); }
        };
        let post = match ::protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()) {
            Ok(post) => post,
            Err(error) => {
                *self.mutation = Some(mutation);
                *self.inverse = Some(inverse);
                *self.apply_refusal = Some(error);
                return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "preparation retained the original mutation application refusal"));
            }
        };
        let edit = authority.next_edit(mutation, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => { *self.refused = Some((edit, post)); return Err(error); }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        *self.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, progress))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<DrawingSnapshot, DrawingMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<DrawingSnapshot, DrawingMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
        let empty = RetainedCloneProgress::default();
        let grant = grant.retained_grant();
        if !self.closing || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "preparation close exceeds original depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() { return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.prepared.is_some() {
            if self.mutation_retirement.is_none() || self.snapshot_retirement.is_none() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation retains its original installed issuers")); }
            let original = self.prepared.take().expect("observed original prepared candidate");
            let mutations = self.mutation_retirement.take().expect("original mutation issuer");
            let snapshots = self.snapshot_retirement.take().expect("original snapshot issuer");
            return match original.admit_retirement(mutations, snapshots, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "original prepared close birth")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation child changed its actual admitted birth")); } Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original, mutations, snapshots)) => { *self.prepared = Some(original); *self.mutation_retirement = Some(mutations); *self.snapshot_retirement = Some(snapshots); Err(error) },
            };
        }
        if self.mutation.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.mutation, &mut self.active, child); }
        if self.inverse.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.inverse, &mut self.active, child); }
        if self.refused.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.refused, &mut self.active, child); }
        if self.apply_refusal.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.apply_refusal, &mut self.active, child); }
        if self.base.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.base, &mut self.active, child); }
        if let Some(authority) = self.authority.take() { return match authority.retire(child) { Ok((owner, progress)) => { *self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "original preparation authority close birth")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation authority changed admitted birth")); } Ok(RetainedCloneStep::Progress(progress)) }, Err((error, original)) => { *self.authority = Some(original); Err(error) } }; }
        if let Some(factory) = self.mutation_retirement.take() { let factory: std::sync::Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factories[0] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
        if let Some(factory) = self.snapshot_retirement.take() { let factory: std::sync::Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factories[1] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
        if let Some(slot) = self.factories.iter_mut().find(|slot| slot.is_some()) { let factory = slot.as_mut().expect("original preparation factory alias"); let step = factory.step(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "original preparation factory close")?; if factory.terminal_is_empty() { *slot = None; } return Ok(RetainedCloneStep::Progress(step.progress())); }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }

    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.active.is_none() && self.factories.iter().all(Option::is_none) && self.mutation_retirement.is_none() && self.snapshot_retirement.is_none() && self.inverse.is_none() && self.refused.is_none() && self.apply_refusal.is_none() && self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}

impl Drop for DrawingArtifactStorePreparation {
    fn drop(&mut self) { assert!(std::thread::panicking() || (self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.prepared.is_none() && self.inverse.is_none() && self.refused.is_none() && self.apply_refusal.is_none() && self.mutation_retirement.is_none() && self.snapshot_retirement.is_none() && self.active.is_none() && self.factories.iter().all(Option::is_none)), "preparation must retain original owners until supplied-grant terminal closure"); }
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
        artifact_schema: "drawing.document",
        factory: "DrawingGestureOperationJobFactory",
        factory_type: DrawingGestureOperationJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(8_192, 32, 1, 16_384, 7_500, 1, 1),
        tools: ["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "canvasCommitDraft", "canvasEscape"]
    }
}

struct DrawingPathProofs;
impl DrawingPathProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        artifact_schema: "drawing.document",
        factory: "DrawingPathCommandJobFactory",
        factory_type: DrawingPathCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),
        tools: ["editPath"]
    }
}

struct DrawingImageProofs;
impl DrawingImageProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        artifact_schema: "drawing.document",
        factory: "DrawingImageCommandJobFactory",
        factory_type: DrawingImageCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(89_481_584, 4_096, 1, 262_144, 7_500, 1, 1),
        tools: ["importImage"]
    }
}

struct DrawingExportProofs;
impl DrawingExportProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        artifact_schema: "drawing.document",
        factory: "DrawingExportCommandJobFactory",
        factory_type: DrawingExportCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::resumable(4_096, 4_096, 1, 67_108_864, 7_500, 1, 1),
        tools: ["exportDocument"]
    }
}
struct DrawingBoundedProofs;
impl DrawingBoundedProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.draw.drawing@1/*#editor",
        artifact_schema: "drawing.document",
        factory: "DrawingBoundedCommandJobFactory",
        factory_type: DrawingBoundedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "setSnapshot", "commitDocument", "loadDocumentJson", "setActiveExample", "setSelectedOpacity", "engagementSubmit",
            "addLayer", "dropLayerKind", "moveLayer", "deleteLayer", "duplicateLayer", "toggleLayerVisible", "combineBoolean",
            "patchLayer", "patchLayers", "setCamera", "setCameraZoom", "engagementInput", "editSelection", "editFill",
            "deleteSelection",
    "nudgeSelectionLeft", "nudgeSelectionLeftFast", "nudgeSelectionRight", "nudgeSelectionRightFast", "nudgeSelectionUp", "nudgeSelectionUpFast", "nudgeSelectionDown", "nudgeSelectionDownFast",
        ]
    }
}
//#endregion 🧾️ProofCatalogs

//#region 🔖️DrawingPlayApp

/// 🧪️ Drawing editor owner; durable content stays in the document while concrete Canvas windows own
/// persisted navigation and ephemeral engagement/trace progress.
pub struct DrawingPlayApp {
    arena_boot_fault: Option<&'static str>,
}

impl DrawingPlayApp {
    pub fn arena_boot_fault(&self) -> Option<&'static str> {
        self.arena_boot_fault.or_else(semio_s_artifact_draw_drawing_host_arena::drawing_mutation_arena_pool_fault)
    }
}

fn render_drawing_body(
    body_key: &str,
    doc: &ArtifactView<'_ ,DrawingSnapshot>,
    config: &DrawingCanvasWindowConfig,
    preview: &DrawingGesturePreview,
    selection: &[String],
    point_selection: &[String],
    view_state: &semio_framework_plugin::ViewModel,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let document=doc.snapshot;
    let labels = semio_framework_plugin::resolve_labels::<DrawingPlayLabels>(view_state);
    let active_utility = drawing_active_utility(view_state);
    // 🪟️ One `TreeWindows` per panel body: the host's open/scroll state for exactly the containers
    // that body owns, plus the shared first-paint budget the panel spends in document order.
    let windows = semio_framework_plugin::TreeWindows::for_body(view_state, body_key);
    let root = match body_key {
        DRAWING_PLAY_BODY_COMPOSITE => semio_s_artifact_draw_drawing_editor_interaction::geometry_session::with_visual(doc.render_operation(),|plan,revision,fresh|{let idle=DrawingGesturePreview::default();canvas_window::render(plan,revision,document,config,if fresh{preview}else{&idle},active_utility,if fresh{selection}else{&[]},if fresh{point_selection}else{&[]})}),
        DRAWING_PLAY_BODY_LAYERS => layers_panel::render(document, labels, &windows),
        DRAWING_PLAY_BODY_CATALOGUE => catalogue_panel::render(document, labels, &windows),
        DRAWING_PLAY_BODY_PROPERTIES => properties_panel::render(document, &[], labels, &windows),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("drawing.body.label", "the fixed Drawing unknown-body label exceeds its UI bound")),
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(root))
}

impl Default for DrawingPlayApp {
    fn default() -> Self {
        let arena_boot_fault = match semio_s_artifact_draw_drawing_host_arena::request_drawing_mutation_arena_pool() {
            semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaPoolAvailability::Fault(error) => Some(error),
            semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaPoolAvailability::Ready | semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaPoolAvailability::NotReady | semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaPoolAvailability::Contended => None,
        };
        Self { arena_boot_fault }
    }
}

impl ArtifactEditor for DrawingPlayApp {
    fn completion_retirement_birth_bytes(value:&semio_framework_plugin::ArtifactToolCompletionValue<semio_framework_plugin::EditorApp<Self>>)->Option<usize>{completion::birth(value)}
    fn admit_completion_retirement(value:&mut Option<semio_framework_plugin::ArtifactToolCompletionValue<semio_framework_plugin::EditorApp<Self>>>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<Option<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,semio_framework_value::retained_clone::RetainedCloneProgress)>,semio_framework_value::ValueError>{completion::admit(value,grant)}
    /// 📢️ The localized notices of the retained gesture owner's refusals (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        drawing_fault_notices()
    }

    type Snapshot = DrawingSnapshot;
    type Mutation = DrawingMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = DrawingPresence;
    type PresenceMutation = DrawingPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = DrawingCommand;

    const DIALECT: semio_framework_artifact_reference::Dialect = semio_s_artifact_draw_drawing_model::DRAWING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DRAWING_DOCUMENT_SCHEMA;

    fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> Result<protocol::InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        protocol::InteractionTopology { domains: [(DRAWING_INTERACTION_DOMAIN.into(), semio_s_artifact_draw_drawing_editor_interaction::drawing_interaction_topology(doc.snapshot)),(DRAWING_POINT_DOMAIN.into(),semio_s_artifact_draw_drawing_editor_interaction::drawing_point_topology(doc.snapshot))].into() }

})())
}

    /// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields;
    /// without it every live envelope load faults `editor did not declare a loaded-parent child
    /// projection` before the decoded document can replace the store.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("drawing.child-projection"), error.to_string()))
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(semio_s_artifact_draw_drawing_host_arena::drawing_envelope_decode_owner_bundle())
    }

    fn build_document_store_initialization_job(
        envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_s_artifact_draw_drawing_host::owned::drawing_document_store_initialization_job(envelope, operation, generation, actor))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    /// 👥️ Returned local reads retain the actual Draw presence fields and original Arc backing.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(semio_s_artifact_draw_drawing_editor_interaction::presence::DrawingPresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(semio_s_artifact_draw_drawing_editor_interaction::presence::DrawingPresenceRetirementFactory))
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |value| value == &Self::Presence::default()).expect("default Drawing presence is the exact empty terminal")))
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(DrawingArtifactStorePreparationFactory))
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        canvas_window::config::register(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        canvas_window::transient::register(registry)
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        DrawingGestureProofs::bounded_first_step_tool_proofs().into_iter().chain(DrawingBoundedProofs::bounded_first_step_tool_proofs()).chain(DrawingPathProofs::bounded_first_step_tool_proofs()).chain(DrawingImageProofs::bounded_first_step_tool_proofs()).chain(DrawingExportProofs::bounded_first_step_tool_proofs()).collect()
    }

    fn mounted_job_prepare_snapshot_read(operation:semio_framework_plugin::AppRenderOperationContext,snapshot:&Self::Snapshot)->bool{semio_s_artifact_draw_drawing_editor_interaction::geometry_session::prepare(operation,snapshot)}
    fn mounted_job_maintenance_demands(instance:u32,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{semio_s_artifact_draw_drawing_editor_interaction::geometry_session::maintenance_demands(instance,body)}
    fn mounted_job_maintenance_step(instance:u32,cx:&mut semio_framework_job::StepContext<'_>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_plugin::PluginLifecycleStep,semio_framework_plugin::PluginLifecycleFailure>{semio_s_artifact_draw_drawing_editor_interaction::geometry_session::maintenance(instance,grant).map_err(semio_framework_plugin::PluginLifecycleFailure::from)}
    fn mounted_job_close_demands(instance:u32,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{semio_s_artifact_draw_drawing_editor_interaction::geometry_session::close_demands(instance,body)}
    fn mounted_job_close_step(instance:u32,cx:&mut semio_framework_job::StepContext<'_>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_plugin::PluginLifecycleStep,semio_framework_plugin::PluginLifecycleFailure>{semio_s_artifact_draw_drawing_editor_interaction::geometry_session::close(instance,grant).map_err(semio_framework_plugin::PluginLifecycleFailure::from)}
    fn mounted_jobs_terminal_is_empty(instance:u32)->bool{semio_s_artifact_draw_drawing_editor_interaction::geometry_session::terminal_is_empty(instance)}
    fn pending_effects(_owner:&semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,doc:&ArtifactView<'_,DrawingSnapshot>,_cfg:&ConfigView<'_,NoConfig>,_view:Option<&semio_framework_plugin::ViewModel>)->Vec<semio_framework::kernel::Effect>{semio_s_artifact_draw_drawing_editor_interaction::geometry_session::reconcile(doc)}

    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(DrawingInstanceOperationOwner::new())
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(DrawingGestureOperationJobFactory::new(&controller))?;
        registry.register(DrawingBoundedCommandJobFactory::new(&controller))?;
        registry.register(DrawingPathCommandJobFactory::new(&controller))?;
        registry.register(DrawingImageCommandJobFactory::new(&controller))?;
        registry.register(DrawingExportCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if DRAWING_EXPORT_TOOL_IDS.contains(&request.tool_id.as_str()) {return export_job::build(request).map(Some);}
        if DRAWING_IMAGE_TOOL_IDS.contains(&request.tool_id.as_str()) {return image_job::build(request).map(Some);}
        if DRAWING_PATH_TOOL_IDS.contains(&request.tool_id.as_str()) {return path_job::build(request).map(Some);}
        if DRAWING_BOUNDED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return drawing_bounded_tool_job(request).map(Some);
        }
        if !DRAWING_GESTURE_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.tool-mismatch"), "Drawing gesture command does not match its exact registered tool"));
        }
        let view_state = request.context.view_state.clone().ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("drawing.canvas.window-required"), "Drawing gesture commands require one concrete Canvas window instance"))?;
        let window_config = canvas_window::config::from_snapshot(request.context.window_config.as_ref());
        let window_transient = canvas_window::transient::from_snapshot(request.context.window_transient.as_ref());
        let operation_context = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = DrawingGestureOperationPayload {
            source_identity: SceneIdentity{instance:request.app_instance_id,base:request.operation.base_revision.0,generation:request.operation.generation.0,revision:request.canonical_base_revision},
            command: *request.command,
            snapshot: request.snapshot,
            config: request.config,
            window_config,
            window_transient,
            view_state,
            history: request.history,
            interaction_state: request.interaction_state,
            instance_owner: request.instance_operation_owner,
            operation_context,
            active_utility_id: request.context.view_state.as_ref().map_or(DRAWING_DEFAULT_UTILITY, drawing_active_utility).to_owned(),
            completion: request.completion,
        };
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(semio_s_artifact_draw_drawing_editor_interaction::config::app_schema_descriptor())
    }

    fn initial_snapshot() -> DrawingSnapshot {
        semio_s_artifact_draw_drawing_model::default_drawing_document("empty", None)
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
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    // 🖼️ No override: whole-document replacement has no `Mutation` vehicle any more (banned
    // vocabulary — see `🧬️mutations/🦀️.rs`'s module doc). The default `None` disables the
    // generic `import_media("artifact:in")` port for drawing; explicit whole-document load/replace
    // stays reachable through the `set_snapshot`/`commit_document`/`load_document_json`/
    // `set_active_example` commands, which now emit `Effect::LoadDocument` (the sanctioned
    // non-history reset path) instead.

    fn clipboard_media_type()->Option<MediaType>{Some(semio_s_artifact_draw_drawing_editor_interaction::clipboard::media_type())}

    fn copy_fragment(doc:&ArtifactView<'_,DrawingSnapshot>,_cfg:&ConfigView<'_,NoConfig>,interaction:&InteractionView<'_>)->Result<semio_framework_plugin::kernel::ClipboardFragment,semio_framework_plugin::kernel::ClipboardError>{
        semio_s_artifact_draw_drawing_editor_interaction::clipboard::copy(doc.snapshot,&interaction.selection(DRAWING_INTERACTION_DOMAIN).ids)
    }

    fn cut_operations(doc:&ArtifactView<'_,DrawingSnapshot>,_cfg:&ConfigView<'_,NoConfig>,interaction:&InteractionView<'_>)->Vec<DrawingMutation>{
        semio_s_artifact_draw_drawing_editor_interaction::clipboard::cut(doc.snapshot,&interaction.selection(DRAWING_INTERACTION_DOMAIN).ids).unwrap_or_default()
    }

    fn build_reserved_tool_job(request:semio_framework_plugin::ArtifactReservedToolJobRequest<semio_framework_plugin::EditorApp<Self>>)->Result<Option<semio_framework_plugin::ArtifactReservedToolJob>,Fault>{
        if !DRAWING_CLIPBOARD_RESERVED_IDS.contains(&request.tool_id.as_str()){return Ok(None);}
        Ok(Some(semio_framework_plugin::ArtifactReservedToolJob::new(crate::clipboard_job::DrawingClipboardJob::new(request)?)))
    }

    /// 🏷️ `app_commands!`'s generated `command_id()`.
    fn command_id(command: &DrawingCommand) -> &'static str {
        command.command_id()
    }

    /// 🌉️ Shell `{action, args}` → `DrawingCommand` (see `args_bridge`); the trait default refuses
    /// every app action, which left the whole ribbon/palette dead in the React shell.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        args_bridge::command_from_action(action, args)
    }

    fn handle(
        command: &DrawingCommand,
        doc: &ArtifactView<'_, DrawingSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<DrawingMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        if DRAWING_GESTURE_TOOL_IDS.contains(&command.command_id()) {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.retained-route"), "Drawing gesture commands are reachable only through their exact retained factory owner"));
        }
        let mut session = DrawingSession::new(view_state.map_or(DRAWING_DEFAULT_UTILITY, drawing_active_utility), doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str()));
        session.interaction.ids = interaction.selection(DRAWING_INTERACTION_DOMAIN).ids.clone();
        session.interaction.points = interaction.selection(DRAWING_POINT_DOMAIN).ids.clone();
        session.window_config = canvas_window::config::current(cfg);
        command.dispatch(doc, cfg, &mut session)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_drawing_body(body_key, doc, &canvas_window::config::current(cfg), &DrawingSession::default().preview(), &[], &[], view_state)
    }

    fn render_with_instance_operation_owner(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, DrawingSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let preview = match doc.render_operation() {
            Some(operation) => owner
                .with_mut::<DrawingInstanceOperationOwner, _>(|owner| Ok(owner.preview_projection(semio_s_artifact_draw_drawing_editor_interaction::geometry_session::identity(operation), drawing_active_utility(view_state))))
                .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("drawing.gesture.preview-owner", error.message))?
                .unwrap_or_default(),
            None => DrawingGesturePreview::default(),
        };
        render_drawing_body(body_key, doc, &canvas_window::config::current(cfg), &preview, &[], &[], view_state)
    }

    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, DrawingSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        if body_key == DRAWING_PLAY_BODY_PROPERTIES {
            return properties_panel::render(doc.snapshot, &interaction.selection(DRAWING_INTERACTION_DOMAIN).ids, semio_framework_plugin::resolve_labels::<DrawingPlayLabels>(view_state), &semio_framework_plugin::TreeWindows::for_body(view_state, body_key)).map(semio_framework_plugin::built_to_component_tree);
        }
        let preview=match doc.render_operation() {
            Some(operation)=>owner.with_mut::<DrawingInstanceOperationOwner,_>(|owner|Ok(owner.preview_projection(semio_s_artifact_draw_drawing_editor_interaction::geometry_session::identity(operation),drawing_active_utility(view_state))))
                .map_err(|error|semio_framework_plugin::PluginAssemblyError::new("drawing.gesture.preview-owner",error.message))?.unwrap_or_default(),
            None=>DrawingGesturePreview::default(),
        };
        render_drawing_body(body_key,doc,&canvas_window::config::current(cfg),&preview,&interaction.selection(DRAWING_INTERACTION_DOMAIN).ids,&interaction.selection(DRAWING_POINT_DOMAIN).ids,view_state)

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_engagements(doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        semio_s_artifact_draw_drawing_editor_interaction::status::engagements(doc.snapshot.layers.len(),0,String::new(),view_state)
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, DrawingSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        semio_s_artifact_draw_drawing_editor_interaction::status::engagements(doc.snapshot.layers.len(),interaction.selection(DRAWING_INTERACTION_DOMAIN).ids.len(),canvas_window::transient::current(transient).engagement_input,view_state)
    }
}
//#endregion 🔖️DrawingPlayApp

//#region 🔖️Io

/// 🔌️ Relocated verbatim from the `⚙️engine` directory (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 4: anything returning `AppIo` or
/// referencing an app type lives in `🎛️apps/<app>/`). This app's typed media I/O surface
/// (`AppDefinition.io`) — mirrors the `2d.drawing` `ArtifactKindSpec` literal `create_drawing_app`
/// already declares via `.artifact_kind(...)` (schema/media type/export+import formats copied
/// verbatim), plus the app-specific `vector:out` port (see `drawing_vector_out_port` below).
pub fn drawing_io() -> semio_framework::AppIo {
    semio_framework::AppIo {
        artifact_schema: DRAWING_DOCUMENT_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
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
/// reuses `crate::standards::v1::subsets::any::io::drawing_document_to_svg` (the same semio/drawing↔svg bridge the
/// export-svg shell path uses), so there is exactly one SVG renderer.
pub fn drawing_vector_media(doc: &DrawingSnapshot) -> Result<Media, MediaError> {
    let (svg, _width, _height) = semio_s_artifact_draw_drawing_write::drawing_document_to_svg(doc).map_err(|error| MediaError::Payload("vector:out".into(), error))?;
    Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "2d.drawing".into(), json: svg } })
}
//#endregion 🔖️Io

//#region 🔖️Manifest
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub mod unit_tests;
/// 🚪️ Host document-archive door law for the example load (`setActiveExample` → `LoadDocument`).
#[cfg(test)]
#[path = "🧪️tests/🔬️archive-load/🦀️.rs"]
mod archive_load_tests;
/// 🌉️ The action bridge folds camelCase keys and JSON values into typed drawing commands.
#[cfg(test)]
#[path = "🧪️tests/🌉️args-bridge/🦀️.rs"]
mod args_bridge_tests;
//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts
//#endregion 🪢️TaxonomyMounts

#[path="📬️completion/🦀️.rs"]
mod completion;

/// 📣️ The en/de notices of every `drawing.gesture.*` refusal code (design §20.12).
pub fn drawing_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 9]> = std::sync::LazyLock::new(|| {
        [
            ("drawing.gesture.retained-route", LocalizedLabel::native("This drawing gesture can only run in an open drawing window.", "Diese Zeichengeste läuft nur in einem geöffneten Zeichenfenster.")),
            ("drawing.gesture.closing", LocalizedLabel::native("The drawing is closing; the gesture was not applied.", "Die Zeichnung wird geschlossen; die Geste wurde nicht angewendet.")),
            ("drawing.gesture.saturated", LocalizedLabel::native("Too many drawing gestures are running at once.", "Zu viele Zeichengesten laufen gleichzeitig.")),
            ("drawing.gesture.owner", LocalizedLabel::native("The drawing gesture ended before it could continue.", "Die Zeichengeste endete, bevor sie fortgesetzt werden konnte.")),
            ("drawing.gesture.point-capacity", LocalizedLabel::native("The stroke has too many points.", "Der Strich hat zu viele Punkte.")),
            ("drawing.gesture.query-owner", LocalizedLabel::native("Another drawing gesture owns this query.", "Eine andere Zeichengeste besitzt diese Abfrage.")),
            ("drawing.gesture.query-capacity", LocalizedLabel::native("Too many shapes match this point.", "Zu viele Formen treffen diesen Punkt.")),
            ("drawing.gesture.query-output-capacity", LocalizedLabel::native("The interaction produced too many results.", "Die Interaktion erzeugte zu viele Ergebnisse.")),
            ("drawing.gesture.command", LocalizedLabel::native("This action cannot run during a drawing gesture.", "Diese Aktion kann während einer Zeichengeste nicht ausgeführt werden.")),
        ]
    });
    &*NOTICES
}

#[cfg(test)]
#[path="🧪️tests/🪪️identity/🦀️.rs"]
pub mod identity_test;

//#region 🪢️TaxonomyMounts
//#endregion 🪢️TaxonomyMounts

//#region 🪢️TaxonomyMounts
#[path = "📋️clipboard/🧵️job/🦀️.rs"]
pub mod clipboard_job;
//#endregion 🪢️TaxonomyMounts
