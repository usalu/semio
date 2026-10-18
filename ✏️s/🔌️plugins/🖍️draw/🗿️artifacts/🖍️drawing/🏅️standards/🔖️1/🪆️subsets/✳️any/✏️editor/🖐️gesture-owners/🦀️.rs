//! 🖐️ Drawing gesture owners: the gesture tool ids and limits and the fixed operation owners of the gesture and instance operations.

use super::*;

//#region 🧵️GestureOperationJobs
pub const DRAWING_GESTURE_TOOL_IDS: &[&str] = &["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "canvasCommitDraft", "canvasEscape"];

pub const DRAWING_GESTURE_RAW_BYTES: usize = 8_192;

pub const DRAWING_GESTURE_RETAINED_BYTES: usize = 131_072;

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
pub(super) const DRAWING_GESTURE_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasDoubleClick", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasCommitDraft", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasEscape", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
];

pub struct DrawingGestureOperationOwner {
    pub session: Option<DrawingSession>,
    pub(super) closing: bool,
    /// 🧾️ The declared `DRAWING_GESTURE_RETAINED_BYTES` budget still to be handed back to the
    /// registry. The session itself is dropped on the first granted close step; its budget is
    /// released one grant-sized page per step, because every framework close/maintenance pump grants
    /// at most `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` (4 KiB) — demanding the whole 32 KiB in one step
    /// meant no gesture owner ever closed outside a hand-rolled test (ticket
    /// 26/09/05/DRAW-PLUGIN-END-TO-END, 2026-09-17).
    unreleased_bytes: usize,
}

impl DrawingGestureOperationOwner {
    pub fn new(active_utility_id: &str, authoring_seed: &str) -> Self {
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

pub(super) const DRAWING_GESTURE_OPERATION_SLOTS: usize = 64;

pub struct DrawingInstanceOperationOwner {
    pub operations: semio_framework_job_registry::FixedOperationRegistry<DrawingGestureOperationOwner, DRAWING_GESTURE_OPERATION_SLOTS>,
    pub active: Option<(semio_framework_job_registry::FixedOperationKey, SceneIdentity)>,
    pub(super) closing: bool,
    /// 🚪️ Every slot has been marked closing. `FixedOperationRegistry::begin_close_step` and
    /// `close_step` share one cursor and each advance it by one, so a close that calls both per step
    /// only ever begins the close of every OTHER slot — an owner on the wrong parity was never marked
    /// and its close spun `Pending { 0, 0 }` until the app's Drop witness fired (ticket
    /// 26/09/05/DRAW-PLUGIN-END-TO-END, 2026-09-17). The first close step now sweeps all slots once.
    close_begun: bool,
}

impl DrawingInstanceOperationOwner {
    pub fn new() -> Self {
        Self { operations: semio_framework_job_registry::FixedOperationRegistry::new(DRAWING_GESTURE_OPERATION_SLOTS * DRAWING_GESTURE_RETAINED_BYTES), active: None, closing: false, close_begun: false }
    }

    pub(super) fn dispatch(&mut self, payload: &DrawingGestureOperationPayload, context:&mut semio_framework_job::StepContext<'_>) -> Result<Option<(Emit<DrawingMutation, NoConfigMutation, NoDraftMutation>, DrawingCanvasWindowTransient)>, Fault> {
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

    pub fn preview_projection(&mut self, source_identity: SceneIdentity, active_utility: &str) -> Option<DrawingGesturePreview> {
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
//#endregion 🧵️GestureOperationJobs
