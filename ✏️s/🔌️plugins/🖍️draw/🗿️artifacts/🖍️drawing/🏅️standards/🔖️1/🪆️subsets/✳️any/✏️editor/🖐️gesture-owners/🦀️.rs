//! 🖐️ Drawing gesture budgets and per-instance registry.

use super::*;

//#region 🧵️GestureOperationJobs
pub const DRAWING_GESTURE_TOOL_IDS: &[&str] = &["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "canvasCommitDraft", "canvasEscape"];

pub const DRAWING_GESTURE_RAW_BYTES: usize = 8_192;

pub const DRAWING_GESTURE_RETAINED_BYTES: usize = 131_072;

/// 🎟️ One release turn that can retire a gesture owner's declared retained budget.
fn drawing_gesture_release_grant() -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant::one_release_turn(DRAWING_GESTURE_RETAINED_BYTES, 32)
}

/// 🛣️ Gesture publication lanes follow the artifact commit and Canvas transient.
pub(super) const DRAWING_GESTURE_PUBLICATION_CONTRACTS: &[semio_plugin::ArtifactToolPublicationContract] = &[
    semio_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[semio_plugin::ArtifactToolPublicationLane::Artifact, semio_plugin::ArtifactToolPublicationLane::WindowTransient] },
    semio_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[semio_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[semio_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_plugin::ArtifactToolPublicationContract { tool_id: "canvasDoubleClick", lanes: &[semio_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_plugin::ArtifactToolPublicationContract { tool_id: "canvasCommitDraft", lanes: &[semio_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_plugin::ArtifactToolPublicationContract { tool_id: "canvasEscape", lanes: &[semio_plugin::ArtifactToolPublicationLane::Artifact, semio_plugin::ArtifactToolPublicationLane::WindowTransient] },
];

pub struct DrawingGestureOperationOwner {
    pub session: Option<DrawingSession>,
    pub(super) closing: bool,
    /// 🧾️ The first close gives up the session. Later turns return one budget page.
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

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|semio_framework_value::RetainedWorkOutcome{result:(||->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
            let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_release_bytes;
            if !self.closing || maximum_items == 0 || maximum_bytes == 0 {
                return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked);
            }
            let released_items = usize::from(self.session.take().is_some());
            let released_bytes = self.unreleased_bytes.min(maximum_bytes);
            self.unreleased_bytes -= released_bytes;
            if released_items == 0 && released_bytes == 0 {
                return Ok(semio_framework_job::InteractiveJobCloseStep::Complete {progress:Default::default()});
            }
            Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress:semio_framework_value::retained_clone::RetainedCloneProgress {copied_items:released_items,released_bytes,..Default::default()} })
        })(),work:0})
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
    /// 🚪️ Sweep the slots before advancing the shared retirement cursor.
    close_begun: bool,
}

impl DrawingInstanceOperationOwner {
    pub fn new() -> Self {
        Self { operations: semio_framework_job_registry::FixedOperationRegistry::new(DRAWING_GESTURE_OPERATION_SLOTS * DRAWING_GESTURE_RETAINED_BYTES), active: None, closing: false, close_begun: false }
    }

    pub(super) fn dispatch(&mut self, payload: &DrawingGestureOperationPayload, context:&mut semio_framework_job::StepContext<'_>) -> Result<Option<(Emit<DrawingMutation, NoConfigMutation, NoDraftMutation>, DrawingCanvasWindowTransient)>, Fault> {
        let original = payload.original.as_ref().ok_or_else(|| Fault::from("drawing.gesture-payload-missing"))?;
        let key = semio_framework_job_registry::FixedOperationKey::new(original.operation.operation, original.operation.generation);
        let source_identity = payload.source_identity;
        let command = original.command.as_ref();
        let snapshot = original.snapshot.as_ref();
        let config = original.config.as_ref();
        let view = original.context.view_state.as_deref().expect("admitted original Drawing Canvas view");
        let active_utility_id = drawing_active_utility(view);
        let history = original.history.as_ref();
        let operation = semio_plugin::AppOperationContext {
            app_instance_id: original.app_instance_id,
            parent_document_id: original.parent_document_id.clone(),
            operation_id: original.operation.operation.0,
            generation: original.operation.generation.0,
            canonical_base_revision: original.canonical_base_revision,
            retained: original.retained,
            authoring_seed: original.authoring_seed.clone(),
        };
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
            if !self.operations.can_admit(live_key, DRAWING_GESTURE_RETAINED_BYTES) {
                for _ in 0..DRAWING_GESTURE_OPERATION_SLOTS * 2 {
                    if self.operations.can_admit(live_key, DRAWING_GESTURE_RETAINED_BYTES) {
                        break;
                    }
                    let _ = context.with_retained_loan(drawing_gesture_release_grant(), |same| self.operations.close_step(same));
                }
            }
            self.operations.admit(live_key, DrawingGestureOperationOwner::new(active_utility_id, &operation.authoring_seed)).map_err(|mut rejected| {
                rejected.owner.cancel();
                rejected.owner.begin_close();
                let _ = context.with_retained_loan(drawing_gesture_release_grant(), |same| rejected.owner.close_step(same));
                Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.saturated"), "the fixed Drawing gesture operation authority is saturated")
            })?;
            self.active = Some((live_key, source_identity));
        }
        let retained = self.operations.get_mut(live_key).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.owner"), "the exact Drawing gesture owner changed before its bounded reducer step"))?;
        let session = retained.session.as_mut().ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.owner"), "the Drawing gesture session is already closing"))?;
        session.identity_cancel=Some(context.cancel_token());
        session.source_identity=Some(source_identity);
        session.window_config = canvas_window::config::from_snapshot(original.context.window_config.as_ref());
        session.window_transient = canvas_window::transient::from_snapshot(original.context.window_transient.as_ref());
        session.base = Some(canvas_pointer_down::DrawingToolBase { document: original.snapshot.clone(), operation: Some(operation.clone()) });
        if session.tool.context().points_overflowed {
            self.operations.cancel(live_key);
            self.active = None;
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.point-capacity"), "the fixed Drawing gesture point capacity was exceeded"));
        }
        if let Some(query) = session.point_query.as_mut() {
            if query.command_id != command.command_id() {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.gesture.query-owner"), "a retained Drawing point query rejects a different command owner"));
            }
            semio_s_artifact_draw_drawing_scene_mounted::prepare_query(source_identity,snapshot);
            let admitted=semio_s_artifact_draw_drawing_scene_mounted::with_query(source_identity,|status,borrowed|match status {
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
            let ids=original.interaction_state.selection.get(DRAWING_INTERACTION_DOMAIN).map(|selection|selection.ids.as_slice()).unwrap_or(&[]);
            let point_ids=original.interaction_state.selection.get(DRAWING_POINT_DOMAIN).map(|selection|selection.ids.as_slice()).unwrap_or(&[]);
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
                query.cursor.retain_selection_bounds(original.interaction_state.selection.get(DRAWING_INTERACTION_DOMAIN).map(|selection|selection.ids.as_slice()).unwrap_or(&[]))?;
                query.drag_start = Some(world);
                session.point_query = Some(query);
                return Ok(None);
            }
        }
        if let DrawingCommand::CanvasPointerMove(payload) = command {
            if session.tool.at_rest() {
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
        let document_owner = original.snapshot.clone();
        let retained_emit = match command {
            DrawingCommand::CanvasPointerMove(pointer) if session.tool.matches("pressing") || session.tool.matches("dragging") => {
                let [x,y] = pointer.last_sample();
                let (x,y) = canvas_pointer_down::canvas_point_to_world(&session.window_config.viewport,x,y,pointer.width,pointer.height);
                Some(Some(session.sample([x,y],pointer.shift,pointer.alt)?))
            }
            DrawingCommand::CanvasPointerMove(payload) if session.tool.matches("marqueeing") && session.tool.context().method == "lasso" => Some(session.advance_lasso_move(payload)?),
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

impl semio_plugin::ArtifactInstanceOperationOwner for DrawingInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn retirement_demands(&self, _body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_value::RetirementDemand;
        if !self.closing {
            return Ok(RetirementDemand { copy_bytes: size_of::<bool>(), depth: 1, ..Default::default() });
        }
        Ok(if self.operations.is_empty() { Default::default() } else { RetirementDemand { release_bytes: DRAWING_GESTURE_RAW_BYTES, depth: 4, ..Default::default() } })
    }

    fn maintenance_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,_grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_plugin::PluginLifecycleStep, semio_plugin::PluginLifecycleFailure> {
        for _ in 0..DRAWING_GESTURE_OPERATION_SLOTS {
            match self.operations.close_step(cx)? {
                semio_framework_job::InteractiveJobCloseStep::Pending { progress } if progress == Default::default() && !self.operations.is_empty() => continue,
                semio_framework_job::InteractiveJobCloseStep::Blocked => return Ok(semio_plugin::PluginLifecycleStep::Blocked { reason: "Drawing gesture close owner awaits a non-empty grant" }),
                semio_framework_job::InteractiveJobCloseStep::Pending { progress } => return Ok(semio_plugin::PluginLifecycleStep::Progress(progress)),
                semio_framework_job::InteractiveJobCloseStep::Complete { progress } => return Ok(semio_plugin::PluginLifecycleStep::Complete(progress)),
            }
        }
        Ok(semio_plugin::PluginLifecycleStep::Progress(Default::default()))
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_plugin::PluginLifecycleStep, semio_plugin::PluginLifecycleFailure> {
        if !self.closing {
            let grant = semio_plugin::app::original_lifecycle_grant(cx, grant);
            let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
            if cx.retirement_should_yield() || !semio_plugin::app::plugin_grant_funds(demand, grant) {
                return Ok(semio_plugin::PluginLifecycleStep::AwaitingInput { reason: "Drawing instance close requires its original close flag grant" });
            }
            let progress = semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
            cx.consume_retained(progress)?;
            cx.consume_fuel(1);
            self.closing = true;
            return Ok(if self.operations.is_empty() { semio_plugin::PluginLifecycleStep::Complete(progress) } else { semio_plugin::PluginLifecycleStep::Progress(progress) });
        }
        if self.operations.is_empty() {
            return Ok(semio_plugin::PluginLifecycleStep::Complete(Default::default()));
        }
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
