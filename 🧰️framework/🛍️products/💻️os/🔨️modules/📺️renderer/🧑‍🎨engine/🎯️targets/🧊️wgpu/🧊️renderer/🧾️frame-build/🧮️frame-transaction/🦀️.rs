//! 🧮️ Frame transaction: the transaction phases and steps of one frame and `boot_runtime`; the frame-transaction crate of the renderer partition, above the presenter (plan section 5.2).
use super::*;

//#region 📮️RuntimeMailbox
pub(crate) struct FrameTransaction {
    pub(crate) retained: semio_framework_job::RetainedCloneGrant,
    directives: Option<FrameDirectives>,
    pub(crate) operation: semio_framework_trace::OperationId,
    pub(crate) generation: semio_framework_trace::Generation,
    base_witness: Option<RuntimePresentationWitness>,
    effect_opportunities: u32,
    pub(crate) stage: FrameTransactionStage,
    pub(crate) phase: AppFrameTransactionPhase,
    board_authority_cursor: usize,
    world3d_authority_cursor: usize,
    scene_camera_cursor: Option<scenes::SceneCameraDispatchCursor>,
    /// 🥽️ How many world surfaces this frame has already offered a `registerBrushMesh` step — a
    /// plain index into `world3d_states`, so the phase owns no closable authority.
    brush_mesh_cursor: usize,
    pub(crate) build_cursor: Option<FrameBuildCursor>,
    finish_cursor: Option<FrameFinishCursor>,
    pub(crate) after_chrome: Option<AppFrameAfterChrome>,
    raster_uploads: Option<scenes::PendingRasterUploadCursor>,
    raster_rejected: Option<ui_wgpu::wgpu::PreparedRasterProducer>,
}

pub(crate) enum AppFrameTransactionStep {
    Pending,
    Complete(AppFrameBuild),
    /// 🌀️ This opportunity's inputs moved under it — a newer input generation, a newer scene
    /// revision, or a cancelled operation. NOT a defect: the candidate is discarded and the next
    /// opportunity builds a fresh one from the current witness.
    ///
    /// 🩸️ Every one of those conditions used to answer [`Self::Fault`] AND record a runtime frame
    /// fault, which the browser worker turns into a surface quarantine. The scene revision is bumped
    /// by every runtime completion (`enqueue_runtime_completion`, `finish`), so a frame build long
    /// enough to ingest a retained document is superseded as a matter of course — and the whole shell
    /// died with `frame opportunity base revision was superseded` instead of simply rebuilding
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-blank-paint-2026-09-12.md`).
    Superseded,
    Fault,
    CloseFailure(semio_framework_job::OriginalTurnFailure),
}

/// 🧭️ The mounted frame's seven monotonic publication stages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FrameTransactionStage {
    DrainProjectionDeltas,
    RouteIntents,
    FlushEffects,
    PresentSurface,
    ReconcileTree,
    BuildRenderPackets,
    PublishSnapshot,
}

impl FrameTransactionStage {
    pub(crate) fn latency_stage(self) -> frame_latency::FrameLatencyStage {
        match self {
            Self::DrainProjectionDeltas => frame_latency::FrameLatencyStage::TransactionDrainProjectionDeltas,
            Self::RouteIntents => frame_latency::FrameLatencyStage::TransactionRouteIntents,
            Self::FlushEffects => frame_latency::FrameLatencyStage::TransactionFlushEffects,
            Self::PresentSurface => frame_latency::FrameLatencyStage::TransactionPresentSurface,
            Self::ReconcileTree => frame_latency::FrameLatencyStage::TransactionReconcileTree,
            Self::BuildRenderPackets => frame_latency::FrameLatencyStage::TransactionBuildRenderPackets,
            Self::PublishSnapshot => frame_latency::FrameLatencyStage::TransactionPublishSnapshot,
        }
    }
}

enum AppFrameTransactionPhase {
    SceneCamera,
    BrushMesh,
    Build,
    InputEvents,
    FrameDeferred,
    BoardAuthority,
    World3dSnapshot,
    World3dAuthority,
    RasterUploads,
    Finish,
    Terminal,
}

impl FrameTransaction {
    pub(crate) fn new(directives: FrameDirectives, operation: semio_framework_trace::OperationId, generation: semio_framework_trace::Generation, retained: semio_framework_job::RetainedCloneGrant) -> Self {
        Self {
            retained,
            directives: Some(directives),
            operation,
            generation,
            base_witness: None,
            effect_opportunities: 0,
            stage: FrameTransactionStage::DrainProjectionDeltas,
            phase: AppFrameTransactionPhase::SceneCamera,
            board_authority_cursor: 0,
            world3d_authority_cursor: 0,
            scene_camera_cursor: None,
            brush_mesh_cursor: 0,
            build_cursor: None,
            finish_cursor: None,
            after_chrome: None,
            raster_uploads: None,
            raster_rejected: None,
        }
    }

    /// 🎟️ No interaction state, no frame — and NO PARKING.
    ///
    /// 🩸️ Parking here (`Pending`) kept the build alive across the whole checkout, and a live build
    /// both freezes the frame generation and keeps asking for frames, so the worker isolate spun its
    /// whole interactive share on a transaction that could not move — starving the very suspended
    /// turn whose completion returns the state. Measured on 6118 as a third dispatched pointer move
    /// that never came home, after which the isolate answered nothing for thirty seconds
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md`).
    /// Ending the build instead retires the session in this same turn: the mailbox is pumped from
    /// every advance, the returning completion is applied at the next opportunity, and the build that
    /// needs the state is admitted fresh against it.
    ///
    /// 🥽️ ONE `registerBrushMesh` per world surface per frame — React's `BrushMeshRegistrar`
    /// announces a loaded GLB's collision geometry to the puzzle guest per window instance,
    /// and `drainPuzzle3dBrushMeshQueue` keeps exactly one page outstanding while a first
    /// upload runs. The wgpu host dispatched nothing at all, so the guest's brush and
    /// volume-brush utilities had no collision body on this renderer (ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w11a-…-missing.md` §6 family B). The row is
    /// `gesture` provenance like every other surface-derived action on this lane.
    ///
    /// 🌩️ ONE effect-flush ROUND, counted where the transaction actually enters the stage.
    ///
    /// 🩸️ The counter used to be charged on every `step` taken WHILE the stage was
    /// `FlushEffects`, which is not a storm measure at all: the stage covers the whole
    /// post-intent tail (deferred frame work, board authority, world snapshots, the prepared
    /// packet), and that tail legitimately needs far more than {@link EFFECT_STORM_BUDGET}
    /// one-cursor steps. It only stayed invisible while the wgpu browser frame never reached
    /// this stage at all; the moment it did, every frame faulted
    /// `frame effect storm budget exhausted` and quarantined the surface
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-blank-paint-2026-09-12.md`).
    ///
    /// 🔁️ A `Pending` draw rebuild does NOT end this phase's turn.
    ///
    /// 🩸️ `step_world3d_draw_rebuild` owns only the PUBLISH half of a rebuild and answers
    /// `Pending` for as long as its cursor is unsealed — and the only thing that can seal
    /// it is `step_world3d_snapshot`, below. Returning here therefore made the two steps
    /// wait on each other forever: measured on 6118 as `procedural-preview` frozen at
    /// `apply-page=0/3 apply-item=1 rebuild=true state-meshes=2 draws=0` across 3.8 M
    /// frame-transaction turns, with the solid `extrude@solid` sitting unconsumed in its
    /// mesh lane (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Both shared drivers —
    /// `♾️infinite/🌍️world`'s own `drive_scene_bridge` and
    /// `⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces` — already run the two steps in
    /// exactly this order, unconditionally; this host was the one place that diverged.
    ///
    /// 🧹️ `Stale` is NOT a fault: it means the rebuild cursor was begun for a revision
    /// or generation the state has since moved past, and the owning module's own rule
    /// (`♾️infinite/🌍️world` §`retained_draw_rebuild_stale_and_interrupted_close_never_publish`)
    /// is to CLOSE that cursor down its stepped ladder and let the next rebuild begin.
    /// The reference driver `⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:191`
    /// faults on `Fault` alone; this host was the one place that also faulted on `Stale`,
    /// and the fault code is fatal — `record_frame_fault` quarantines the surface for
    /// good. Measured on 6118: a hover on the ELEVENTH gesture of a session published
    /// `seq=3751 frame-credits: world3d retained draw rebuild faulted` and the frame
    /// wire froze at 3 751 batches for the remaining nine gestures, with the page
    /// showing "Surface: quarantined · input accepted: no"
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🚧️🚪️ A modal layer OPENING clears the hover the pane published, with no pointer
    /// move of its own. React's overlays take pointer events the instant they mount, so
    /// r3f raises `onPointerOut` there and then — its `context-menu` step journals the
    /// clear inside the step that opened the menu. This renderer only handed a surface its
    /// leave on the next MOVE (W12a), so the clear rode along to the next step that moved
    /// the pointer at all: the stray `interactionHover targets:[]` in
    /// `example-picker-open`, ten steps and six keyboard chords later
    /// (`🗑️generated/w12c-parity-run-19/steps.json` step 33, ticket 26/09/17 packet
    /// W13c §1). Idempotent by construction — the clear retires the published hover, so
    /// the predicate answers `false` on every later frame the layer stays open.
    ///
    /// 🖼️ One bitmap per step off `ui_wgpu`'s own image ledger — the drain
    /// `take_ui_image_upload`'s docstring names ("the host's per-frame drain") and never had.
    /// It answers `None` whenever a `SceneHost` is registered (the renderer's normal case:
    /// `render_ui_image_step` owns `UiNode::Image` then), so this is free here and is what
    /// makes the no-scene-host paint path in `ui_wgpu` actually show a decoded `data:` image
    /// instead of the `alt` placeholder.
    pub(crate) fn step(&mut self, runtime: &AppMailbox, handle: &AppHandle, context: &mut semio_framework_job::StepContext<'_>) -> AppFrameTransactionStep {
        let _latency = frame_latency::FrameLatencyTimer::start(frame_latency::FrameLatencyAuthority::renderer_frame(self.generation.0), self.stage.latency_stage(), 1);
        context.set_stage(self.stage_label());
        if context.operation() != self.operation || context.generation() != self.generation || context.is_cancelled() || context.deadline_exceeded() {
            self.phase = AppFrameTransactionPhase::Terminal;
            return AppFrameTransactionStep::Superseded;
        }
        let Some(current_witness) = runtime.presentation_witness_for(self.generation.0) else {
            self.phase = AppFrameTransactionPhase::Terminal;
            return AppFrameTransactionStep::Superseded;
        };
        if let Some(base_witness) = self.base_witness {
            if base_witness != current_witness {
                self.phase = AppFrameTransactionPhase::Terminal;
                return AppFrameTransactionStep::Superseded;
            }
        } else {
            self.base_witness = Some(current_witness);
        }
        let Some(directives) = self.directives.as_ref() else { return AppFrameTransactionStep::Pending };
        if context.should_yield() {
            return AppFrameTransactionStep::Pending;
        }
        let Ok(mut app) = runtime.try_lock() else { return AppFrameTransactionStep::Pending };
        if !app.interaction_available() {
            self.phase = AppFrameTransactionPhase::Terminal;
            return AppFrameTransactionStep::Superseded;
        }
        if !matches!(self.phase, AppFrameTransactionPhase::InputEvents) && app.frame_actions.staged_batch_remaining().is_some() {
            let transfer = {
                let app = &mut *app;
                let interaction = app.interaction.as_mut().expect("runtime interaction state is worker-owned");
                transfer_frame_input_action(&mut interaction.input, &mut app.frame_actions)
            };
            return match transfer {
                Ok(FrameInputActionStep::Pending | FrameInputActionStep::Transferred) => AppFrameTransactionStep::Pending,
                Ok(FrameInputActionStep::Empty | FrameInputActionStep::Deferred) => {
                    runtime.record_frame_fault("staged frame input action batch lost its runtime owner");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    AppFrameTransactionStep::Fault
                }
                Err(fault) => {
                    runtime.record_frame_fault(fault);
                    self.phase = AppFrameTransactionPhase::Terminal;
                    AppFrameTransactionStep::Fault
                }
            };
        }
        match self.phase {
            AppFrameTransactionPhase::SceneCamera => {
                let grant = context.retained_grant();
                if self.scene_camera_cursor.is_none() {
                    match scenes::SceneCameraDispatchCursor::begin(app_now_ms(), grant) {
                        Ok((cursor, progress)) => {
                            self.scene_camera_cursor = Some(cursor);
                            if context.consume_retained(progress).is_err() { runtime.record_frame_fault("camera original admission receipt exceeded frame authority"); return AppFrameTransactionStep::Fault; }
                            return AppFrameTransactionStep::Pending;
                        }
                        Err(error) => {
                            let _ = context.consume_retained(error.retained_progress());
                            runtime.record_frame_fault("camera original storage admission refused");
                            return AppFrameTransactionStep::Fault;
                        }
                    }
                }
                let (step, progress) = match self.scene_camera_cursor.as_mut().unwrap().step(grant) {
                    Ok(receipt) => receipt,
                    Err(error) => {
                        let _ = context.consume_retained(error.retained_progress());
                        runtime.record_frame_fault("camera original storage transition refused");
                        return AppFrameTransactionStep::Fault;
                    }
                };
                if context.consume_retained(progress).is_err() { runtime.record_frame_fault("camera original transition receipt exceeded frame authority"); return AppFrameTransactionStep::Fault; }
                match step {
                scenes::SceneCameraDispatchStep::Pending => AppFrameTransactionStep::Pending,
                scenes::SceneCameraDispatchStep::Action(action) => {
                    if let Err(_action) = app.frame_actions.try_push(action) {
                        runtime.record_frame_fault("frame deferred action credits exceeded");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                    AppFrameTransactionStep::Pending
                }
                scenes::SceneCameraDispatchStep::Complete => {
                    self.stage = FrameTransactionStage::RouteIntents;
                    self.phase = AppFrameTransactionPhase::BrushMesh;
                    AppFrameTransactionStep::Pending
                }
                scenes::SceneCameraDispatchStep::Fault(fault) => {
                    runtime.record_frame_fault(fault);
                    self.phase = AppFrameTransactionPhase::Terminal;
                    AppFrameTransactionStep::Fault
                }
                }
            },
            AppFrameTransactionPhase::BrushMesh => {
                let Some(surface) = app.shell.world3d_states.keys().nth(self.brush_mesh_cursor).cloned() else {
                    self.phase = AppFrameTransactionPhase::Build;
                    return AppFrameTransactionStep::Pending;
                };
                self.brush_mesh_cursor = self.brush_mesh_cursor.saturating_add(1);
                let action = app.shell.world3d_states.get_mut(surface.as_str()).and_then(|world| infinite_world::world::step_world3d_brush_mesh_announce(&mut world.session, &mut world.scene, &mut world.assets));
                if let Some(action) = action {
                    if let Err(_action) = app.frame_actions.try_push(action) {
                        runtime.record_frame_fault("frame brush mesh action credits exceeded");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                }
                AppFrameTransactionStep::Pending
            }
            AppFrameTransactionPhase::Build => {
                if self.build_cursor.is_none() {
                    let Some(presentation_witness) = runtime.presentation_witness_for(self.generation.0) else {
                        runtime.record_frame_fault("frame presentation input generation was stale before candidate construction");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    };
                    runtime.admit_build_presentation_witness(presentation_witness);
                    self.build_cursor = Some(FrameBuildCursor::new(presentation_witness, self.retained));
                    return AppFrameTransactionStep::Pending;
                }
                let Some(cursor) = self.build_cursor.as_mut() else { return AppFrameTransactionStep::Pending };
                match app.frame_before_input_step(handle, directives, cursor, context) {
                    FrameBuildBoundaryStep::Pending => AppFrameTransactionStep::Pending,
                    FrameBuildBoundaryStep::Complete(partial) => {
                        self.build_cursor = None;
                        self.after_chrome = Some(partial);
                        self.stage = FrameTransactionStage::RouteIntents;
                        self.phase = AppFrameTransactionPhase::InputEvents;
                        AppFrameTransactionStep::Pending
                    }
                    FrameBuildBoundaryStep::Fault(fault) => {
                        runtime.record_frame_fault(fault);
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                    FrameBuildBoundaryStep::CloseFailure(failure)=>AppFrameTransactionStep::CloseFailure(failure),
                }
            }
            AppFrameTransactionPhase::InputEvents => {
                let transfer = {
                    let app = &mut *app;
                    let interaction = app.interaction.as_mut().expect("runtime interaction state is worker-owned");
                    transfer_frame_input_action(&mut interaction.input, &mut app.frame_actions)
                };
                match transfer {
                    Ok(FrameInputActionStep::Pending | FrameInputActionStep::Transferred) => {
                        if self.after_chrome.is_none() {
                            runtime.record_frame_fault("frame input phase lost the retained chrome owner");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        }
                        return AppFrameTransactionStep::Pending;
                    }
                    Ok(FrameInputActionStep::Empty) => {
                        match app.input.drive_retained_action_step() {
                            Ok(Some(action)) => {
                                if app.frame_actions.try_push_queued(action).is_err() {
                                    runtime.record_frame_fault("retained input action credits exceeded");
                                    self.phase = AppFrameTransactionPhase::Terminal;
                                    return AppFrameTransactionStep::Fault;
                                }
                                return AppFrameTransactionStep::Pending;
                            }
                            Ok(None) if app.input.retained_action_pending() => return AppFrameTransactionStep::Pending,
                            Ok(None) => {}
                            Err(_) => {
                                runtime.record_frame_fault("retained input action transport failed");
                                self.phase = AppFrameTransactionPhase::Terminal;
                                return AppFrameTransactionStep::Fault;
                            }
                        }
                        match engine_canvas::drive_text_editor_retained_action_step() {
                            Ok(engine_canvas::TextEditorRetainedActionStep::Pending) => return AppFrameTransactionStep::Pending,
                            Ok(engine_canvas::TextEditorRetainedActionStep::Ready(actions)) => {
                                let admitted = match actions.second {
                                    Some(second) => app.frame_actions.try_push_queued_pair(actions.first, second).is_ok(),
                                    None => app.frame_actions.try_push_queued(actions.first).is_ok(),
                                };
                                if !admitted {
                                    runtime.record_frame_fault("retained text editor action credits exceeded");
                                    self.phase = AppFrameTransactionPhase::Terminal;
                                    return AppFrameTransactionStep::Fault;
                                }
                                return AppFrameTransactionStep::Pending;
                            }
                            Ok(engine_canvas::TextEditorRetainedActionStep::Idle) => {}
                            Err(_) => {
                                runtime.record_frame_fault("retained text editor action transport failed");
                                self.phase = AppFrameTransactionPhase::Terminal;
                                return AppFrameTransactionStep::Fault;
                            }
                        }
                        match engine_canvas::drive_text_editor_outbox_step(&mut app.input) {
                            Ok(true) => return AppFrameTransactionStep::Pending,
                            Ok(false) => {}
                            Err(fault) => {
                                log_debug(&format!("[TRACE] text editor outbox admission failed: {fault:?}"));
                                runtime.record_frame_fault("text editor outbox admission failed");
                                self.phase = AppFrameTransactionPhase::Terminal;
                                return AppFrameTransactionStep::Fault;
                            }
                        }
                    }
                    Ok(FrameInputActionStep::Deferred) => {}
                    Err(fault) => {
                        runtime.record_frame_fault(fault);
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                }
                if interpreter::drive_scene_interaction_step(&mut app.input) {
                    return AppFrameTransactionStep::Pending;
                }
                match interpreter::drive_clipboard_io_step(&mut app.input,context) { Ok(true)=>return AppFrameTransactionStep::Pending,Ok(false)=>{},Err(failure)=>return AppFrameTransactionStep::CloseFailure(failure) }
                let Some(next) = self.effect_opportunities.checked_add(1) else {
                    runtime.record_frame_fault("frame effect opportunity counter exhausted");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                if next > EFFECT_STORM_BUDGET {
                    runtime.record_frame_fault("frame effect storm budget exhausted");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                }
                self.effect_opportunities = next;
                self.stage = FrameTransactionStage::FlushEffects;
                self.phase = AppFrameTransactionPhase::FrameDeferred;
                AppFrameTransactionStep::Pending
            }
            AppFrameTransactionPhase::FrameDeferred => {
                let mut deferred = app.pending_frame_deferred.take();
                if deferred.is_some() && !RuntimeApply::start_frame_deferred(&mut deferred, &mut app, handle) {
                    app.pending_frame_deferred = deferred;
                    return AppFrameTransactionStep::Pending;
                }
                self.phase = AppFrameTransactionPhase::BoardAuthority;
                AppFrameTransactionStep::Pending
            }
            AppFrameTransactionPhase::BoardAuthority => {
                let Some(interaction) = app.interaction.as_mut() else { return AppFrameTransactionStep::Pending };
                let Some(surface_id) = interaction.shell.board2d_states.id_at(self.board_authority_cursor) else {
                    self.board_authority_cursor = 0;
                    self.phase = AppFrameTransactionPhase::World3dSnapshot;
                    return AppFrameTransactionStep::Pending;
                };
                if surface_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    runtime.record_frame_fault("board authority surface identifier exceeded fixed credits");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                }
                let surface_id = surface_id.to_owned();
                let Some(surface) = interaction.shell.board2d_states.get(&surface_id) else {
                    runtime.record_frame_fault("board authority surface order lost ownership");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                if surface.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    runtime.record_frame_fault("board authority controller identifier exceeded fixed credits");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                }
                let controller_id = surface.controller_id.clone();
                match engine_canvas::drive_board_authority_step(&surface_id, context) {
                    infinite_canvas::BoardAuthorityStep::Pending => AppFrameTransactionStep::Pending,
                    infinite_canvas::BoardAuthorityStep::Cancelled => {
                        if let Err(fault) = engine_canvas::release_board_pointer_claim(&surface_id, &mut app.input) {
                            app.input.record_action_fault(fault);
                            runtime.record_frame_fault("board cancelled claim release faulted");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        }
                        self.board_authority_cursor += 1;
                        AppFrameTransactionStep::Pending
                    }
                    infinite_canvas::BoardAuthorityStep::Fault => {
                        let _ = engine_canvas::release_board_pointer_claim(&surface_id, &mut app.input);
                        runtime.record_frame_fault("board retained authority faulted");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                    infinite_canvas::BoardAuthorityStep::Complete => match engine_canvas::publish_board_pointer_step(&surface_id, &mut app.input) {
                        Ok(true) => {
                            self.phase = AppFrameTransactionPhase::InputEvents;
                            AppFrameTransactionStep::Pending
                        }
                        Ok(false) => match engine_canvas::publish_board_event_step(&surface_id, &controller_id, &mut app.input) {
                            Ok(true) => {
                                self.phase = AppFrameTransactionPhase::InputEvents;
                                AppFrameTransactionStep::Pending
                            }
                            Ok(false) => {
                                self.board_authority_cursor += 1;
                                AppFrameTransactionStep::Pending
                            }
                            Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) => {
                                self.phase = AppFrameTransactionPhase::InputEvents;
                                AppFrameTransactionStep::Pending
                            }
                            Err(fault) => {
                                app.input.record_action_fault(fault);
                                runtime.record_frame_fault("board event publication faulted");
                                self.phase = AppFrameTransactionPhase::Terminal;
                                AppFrameTransactionStep::Fault
                            }
                        },
                        Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) => {
                            self.phase = AppFrameTransactionPhase::InputEvents;
                            AppFrameTransactionStep::Pending
                        }
                        Err(fault) => {
                            app.input.record_action_fault(fault);
                            runtime.record_frame_fault("board retained publication faulted");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            AppFrameTransactionStep::Fault
                        }
                    },
                }
            }
            AppFrameTransactionPhase::World3dSnapshot => {
                let Some(interaction) = app.interaction.as_mut() else {
                    return AppFrameTransactionStep::Pending;
                };
                let Some(surface_id) = interaction.shell.world3d_states.id_at(self.world3d_authority_cursor) else {
                    self.world3d_authority_cursor = 0;
                    self.phase = AppFrameTransactionPhase::World3dAuthority;
                    return AppFrameTransactionStep::Pending;
                };
                if surface_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    runtime.record_frame_fault("world3d snapshot surface identifier exceeded fixed credits");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                }
                let surface_id = surface_id.to_owned();
                let Some(state) = interaction.shell.world3d_states.get_mut(&surface_id) else {
                    runtime.record_frame_fault("world3d snapshot surface order lost ownership");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                world3d_ingest_trace(&surface_id, state, "phase-entry");
                let terrain_grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:16*1024*1024,maximum_depth:64};
                let terrain=infinite_world::world::step_world3d_terrain(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, &mut state.terrain,terrain_grant,context);
                if !terrain.ownership.fits(terrain_grant) {
                    runtime.record_frame_fault("world3d terrain ownership receipt exceeded caller grants");
                    self.phase=AppFrameTransactionPhase::Terminal;return AppFrameTransactionStep::Fault;
                }
                if let infinite_world::world::WorldTerrainMeshPublicationStep::Fault(fault)=terrain.step {
                    runtime.record_frame_fault(&format!("world3d terrain ownership faulted: {fault:?}"));
                    self.phase=AppFrameTransactionPhase::Terminal;return AppFrameTransactionStep::Fault;
                }

                if retire_cancelled_world3d_asset_step(&mut state.assets) {
                    context.consume_fuel(1);
                    return AppFrameTransactionStep::Pending;
                }
                match step_world3d_scene_bridge(state, context) {
                    World3dSceneBridgeStep::Pending => return AppFrameTransactionStep::Pending,
                    World3dSceneBridgeStep::Fault => {
                        log_debug(&format!("[TRACE] world3d bridge fault surface={surface_id} {}", state.ingest_census()));
                        runtime.record_frame_fault("world3d scene mesh-wire bridge faulted");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                    World3dSceneBridgeStep::Idle | World3dSceneBridgeStep::Complete => {}
                }
                match step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context) {
                    WorldDrawRebuildStep::Pending | WorldDrawRebuildStep::Complete => {}
                    WorldDrawRebuildStep::Stale => {
                        log_debug(&format!("[TRACE] world3d draw rebuild surface={surface_id} step=Stale {}", state.ingest_census()));
                        close_world3d_draw_rebuild_step(&mut state.scene, context);
                        return AppFrameTransactionStep::Pending;
                    }
                    WorldDrawRebuildStep::Fault => {
                        log_debug(&format!("[TRACE] world3d draw rebuild surface={surface_id} step=Fault {}", state.ingest_census()));
                        runtime.record_frame_fault("world3d retained draw rebuild faulted");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                }
                let snapshot_step = step_world3d_snapshot(state, context);
                world3d_ingest_trace(&surface_id, state, &format!("snapshot-{snapshot_step:?}"));
                if matches!(snapshot_step, World3dSnapshotApplyStep::Complete) {
                    log_debug(&format!("[TRACE] world3d delivery applied surface={surface_id} {}", state.ingest_census()));
                }
                match snapshot_step {
                    World3dSnapshotApplyStep::Idle | World3dSnapshotApplyStep::Complete => {
                        step_world3d_camera_fit_after_snapshot(state);
                        self.world3d_authority_cursor += 1;
                        AppFrameTransactionStep::Pending
                    }
                    World3dSnapshotApplyStep::Pending => AppFrameTransactionStep::Pending,
                    World3dSnapshotApplyStep::Stale | World3dSnapshotApplyStep::Fault => {
                        runtime.record_frame_fault("world3d typed snapshot apply faulted");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                }
            }
            AppFrameTransactionPhase::World3dAuthority => {
                let Some(interaction) = app.interaction.as_mut() else { return AppFrameTransactionStep::Pending };
                let Some(surface_id) = interaction.shell.world3d_states.id_at(self.world3d_authority_cursor) else {
                    self.world3d_authority_cursor = 0;
                    self.stage = FrameTransactionStage::ReconcileTree;
                    self.phase = AppFrameTransactionPhase::RasterUploads;
                    return AppFrameTransactionStep::Pending;
                };
                if surface_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    runtime.record_frame_fault("world3d authority surface identifier exceeded fixed credits");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                }
                let surface_id = surface_id.to_owned();
                let AppInteractionState { shell, input, .. } = interaction;
                let modal = shell.pointer_input_is_modal();
                let Some(state) = shell.world3d_states.get_mut(&surface_id) else {
                    runtime.record_frame_fault("world3d authority surface order lost ownership");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                if modal && world3d_hover_clear_is_owed(&state.selection, &state.authority) && enqueue_world3d_event(&mut state.selection, &mut state.authority, WorldInteractionIntent::pointer_leave(state.bounds.x - 1.0, state.bounds.y - 1.0)).is_err() {
                    input.record_action_fault(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                let Some(generation) = world3d_interaction_front_generation(&state.camera, &state.authority) else {
                    self.world3d_authority_cursor += 1;
                    return AppFrameTransactionStep::Pending;
                };
                world3d_interaction_trace(&surface_id, state, generation, None);
                let interaction_step = step_world3d_interaction(state, generation, input, context);
                world3d_interaction_trace(&surface_id, state, generation, Some(interaction_step));
                match interaction_step {
                    WorldInteractionAuthorityStep::Idle | WorldInteractionAuthorityStep::Complete | WorldInteractionAuthorityStep::Stale => {
                        self.world3d_authority_cursor += 1;
                        self.phase = AppFrameTransactionPhase::InputEvents;
                        AppFrameTransactionStep::Pending
                    }
                    WorldInteractionAuthorityStep::Pending => AppFrameTransactionStep::Pending,
                    WorldInteractionAuthorityStep::OutputBlocked => {
                        self.phase = AppFrameTransactionPhase::InputEvents;
                        AppFrameTransactionStep::Pending
                    }
                    WorldInteractionAuthorityStep::Fault => {
                        runtime.record_frame_fault("world3d retained interaction authority faulted");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                }
            }
            AppFrameTransactionPhase::RasterUploads => {
                if let Some(upload) = ui_wgpu::wgpu::take_ui_image_upload() {
                    scenes::queue_decoded_raster_upload("ui-image", upload.key, upload.width, upload.height, upload.pixels);
                    return AppFrameTransactionStep::Pending;
                }
                let cursor = self.raster_uploads.get_or_insert_with(Default::default);
                match cursor.step() {
                    scenes::PendingRasterUploadStep::Pending => AppFrameTransactionStep::Pending,
                    scenes::PendingRasterUploadStep::Upload(checked_out) => {
                        let Ok(mut producer) = checked_out.take() else {
                            runtime.record_frame_fault("frame raster producer checkout was stale");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        };
                        let Some(partial) = self.after_chrome.as_mut() else {
                            runtime.record_frame_fault("raster upload phase lost the retained chrome owner");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        };
                        let Some(input) = partial.resource_input.as_mut() else {
                            runtime.record_frame_fault("raster upload phase lost the retained resource input");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        };
                        let Some(upload_items) = input.raster_producers.len().checked_add(input.uploads.len()) else {
                            if let Err(mut returned) = cursor.retain_rejected(producer) {
                                returned.begin_close();
                                self.raster_rejected = Some(returned);
                            }
                            runtime.record_frame_fault("frame raster upload item credits exhausted");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        };
                        if upload_items >= input.limits.max_upload_items {
                            if let Err(mut returned) = cursor.retain_rejected(producer) {
                                returned.begin_close();
                                self.raster_rejected = Some(returned);
                            }
                            runtime.record_frame_fault("frame raster upload item credits exceeded");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        }
                        if !producer.bind_frame_generation(input.preview_generation) {
                            if let Err(mut returned) = cursor.retain_rejected(producer) {
                                returned.begin_close();
                                self.raster_rejected = Some(returned);
                            }
                            runtime.record_frame_fault("frame raster producer generation was stale");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        }
                        if let Err(producer) = input.try_push_raster_producer(producer) {
                            if let Err(mut returned) = cursor.retain_rejected(producer) {
                                returned.begin_close();
                                self.raster_rejected = Some(returned);
                            }
                            runtime.record_frame_fault("frame raster producer fixed admission was refused");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        }
                        AppFrameTransactionStep::Pending
                    }
                    scenes::PendingRasterUploadStep::Complete => {
                        self.raster_uploads = None;
                        self.stage = FrameTransactionStage::BuildRenderPackets;
                        self.phase = AppFrameTransactionPhase::Finish;
                        AppFrameTransactionStep::Pending
                    }
                    scenes::PendingRasterUploadStep::Fault(fault) => {
                        runtime.record_frame_fault(fault);
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                }
            }
            AppFrameTransactionPhase::Finish => {
                let Some(partial) = self.after_chrome.as_mut() else {
                    runtime.record_frame_fault("frame finish lost the retained chrome owner");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                let cursor = self.finish_cursor.get_or_insert_with(Default::default);
                match app.frame_after_input_step(runtime, partial, cursor, context) {
                    FrameFinishBoundaryStep::Pending => AppFrameTransactionStep::Pending,
                    FrameFinishBoundaryStep::Complete(frame) => {
                        self.after_chrome = None;
                        self.finish_cursor = None;
                        self.stage = FrameTransactionStage::PublishSnapshot;
                        self.phase = AppFrameTransactionPhase::Terminal;
                        self.directives = None;
                        AppFrameTransactionStep::Complete(frame)
                    }
                    FrameFinishBoundaryStep::Fault(fault) => {
                        runtime.record_frame_fault(fault);
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                }
            }
            AppFrameTransactionPhase::Terminal => AppFrameTransactionStep::Pending,
        }
    }

    /// ♻️ Preserves the camera child's physical receipts before any other frame child is considered.
    pub(crate) fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure> {
        if cx.should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let grant=cx.retained_grant();
        if let Some(camera) = self.scene_camera_cursor.as_mut() {
            if !camera.terminal_is_empty() { return frame_receive_close(frame_child_step(camera.close_step(grant),grant,camera.terminal_is_empty()),cx); }
            let bytes = std::mem::size_of::<Option<scenes::SceneCameraDispatchCursor>>();
            if let Some(step) = frame_inline_gate(grant,bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))? { return Ok(step); }
            self.scene_camera_cursor = None;
            return frame_receive_close(frame_inline_progress(bytes),cx);
        }
        if let Some(cursor) = self.finish_cursor.as_mut() {
            if !cursor.terminal_is_empty() { return frame_receive_close(frame_child_step(cursor.close_step(grant),grant,cursor.terminal_is_empty()),cx); }
            let bytes = std::mem::size_of::<Option<FrameFinishCursor>>();
            if let Some(step) = frame_inline_gate(grant,bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))? { return Ok(step); }
            self.finish_cursor = None;
            return frame_receive_close(frame_inline_progress(bytes),cx);
        }
        if let Some(cursor) = self.build_cursor.as_mut() {
            if !cursor.terminal_is_empty(){return frame_received_child(cursor.close_step(cx),grant,cursor.terminal_is_empty())}
        }
        if grant.maximum_items == 0 { return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }); }
        if let Some(owner)=self.after_chrome.as_mut(){if !owner.terminal_is_empty(){return frame_received_child(owner.close_step(cx),grant,owner.terminal_is_empty())}}
        frame_receive_close(frame_unpriced_child(),cx)
    }

    fn stage_label(&self) -> &'static str {
        match self.stage {
            FrameTransactionStage::DrainProjectionDeltas => "DrainProjectionDeltas",
            FrameTransactionStage::RouteIntents => "RouteIntents",
            FrameTransactionStage::FlushEffects => "FlushEffects",
            FrameTransactionStage::PresentSurface => "PresentSurface",
            FrameTransactionStage::ReconcileTree => "ReconcileTree",
            FrameTransactionStage::BuildRenderPackets => "BuildRenderPackets",
            FrameTransactionStage::PublishSnapshot => "PublishSnapshot",
        }
    }

    /// 🧭️ Other nested frame children still require their own physical retirement witnesses.
    pub(crate) fn terminal_is_empty(&self) -> bool {
        false
    }

    pub(crate) fn discard_presented_input_candidate(&mut self, runtime: &AppMailbox) -> bool {
        if let Some(cursor) = self.build_cursor.as_mut() {
            if !discard_frame_input_candidate(runtime, &mut cursor.input_candidate) {
                return false;
            }
        }
        if let Some(partial) = self.after_chrome.as_mut() {
            if !discard_frame_input_candidate(runtime, &mut partial.input_candidate) {
                return false;
            }
        }
        true
    }
}

/// 🧹️ P3c: this used to build a `PointerCallbacks` here (5 `Rc<RefCell<AppRuntime>>` clones, one
/// per input kind) and hand it back alongside `runtime`. `winit_app.rs`'s own `HostUserEvent` doc
/// comment records that its one caller stopped using it at the P3a enqueue-only
/// `WindowDelegate`/`dispatch_normalized_event` cutover -- `boot_runtime` was left constructing it
/// anyway because touching this signature wasn't that packet's job. It is
/// this packet's job (removing `self_weak`, see this crate's own `AppHandle` doc comment), and per
/// AGENTS.md's no-legacy-code rule, dead construction is deleted outright. Right-click remains a
/// lossless `DispatchEvent::PointerDown { button: Secondary }` in the enqueue-only contract;
/// `winit_app::dispatch_normalized_event` maps it to button `2` and calls the canonical
/// `handle_pointer_button`, whose Shell path opens the context menu. The redundant callbacks-only
/// `handle_context_menu` wrapper is deleted with its sole caller. See `📓️p3c-explicit-app-handle.md`.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn boot_runtime(
    window: Arc<Window>,
    gpu: impl std::future::Future<Output = Result<GpuContext, String>> + Send + 'static,
    plugin_filter: String,
    #[cfg(target_arch = "wasm32")] plugins: Option<JsValue>,
    #[cfg(not(target_arch = "wasm32"))] plugin_modules_root: std::path::PathBuf,
    services:Vec<semio_framework_os_kernel::os_directory::client::InstalledServiceContributionV1>,
) -> Result<(AppMailbox, AppPresenter), String> {
    let dpr = window.scale_factor() as f32;
    let size = window.inner_size();
    #[cfg(target_arch = "wasm32")]
    let (css_width, css_height, dpr) = {
        use winit::platform::web::WindowExtWebSys;
        let dpr = web_sys::window().map(|host| host.device_pixel_ratio() as f32).unwrap_or(dpr);
        if let Some(canvas) = window.canvas() {
            let css_width = canvas.client_width().max(1) as f32;
            let css_height = canvas.client_height().max(1) as f32;
            canvas.set_width((css_width * dpr) as u32);
            canvas.set_height((css_height * dpr) as u32);
            (css_width, css_height, dpr)
        } else {
            (size.width as f32 / dpr, size.height as f32 / dpr, dpr)
        }
    };
    #[cfg(not(target_arch = "wasm32"))]
    let css_width = size.width as f32 / dpr;
    #[cfg(not(target_arch = "wasm32"))]
    let css_height = size.height as f32 / dpr;

    const ANTA_LATIN: &[u8] = include_bytes!("../../../../../../../../../../🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf");
    let font_bytes = match fetch_font_bytes("/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf").await {
        Ok(bytes) if bytes.len() > 256 => bytes,
        _ => ANTA_LATIN.to_vec(),
    };
    let mut atlas = FontAtlas::from_bytes(&font_bytes).map_err(|err| format!("atlas failed: {err}"))?;
    atlas.set_raster_scale(dpr);
    let icons = icon_atlas::build_icon_atlas_scaled(dpr);
    let mut gpu = gpu.await.map_err(|err| format!("gpu init failed: {err}"))?;
    gpu.resize(css_width, css_height, dpr);
    gpu.upload_font_atlas(&atlas);
    gpu.upload_icon_atlas(&icons);

    #[cfg(target_arch = "wasm32")]
    let entries = {
        let plugins = plugins.ok_or("missing wasm programs")?;
        filter_plugins(parse_plugin_entries(plugins).map_err(|err| format!("program parse failed: {err}"))?, &plugin_filter)
    };
    #[cfg(not(target_arch = "wasm32"))]
    let entries = filter_plugins(load_wasm_plugins(&plugin_filter, &plugin_modules_root).await?, &plugin_filter);

    let (locale, terminology) = shell::shell_language_axes()?;
    let mut shell = ShellState::new(entries, plugin_filter.clone(), locale, terminology);
    shell.install_document_services(services);
    shell.screen_w = css_width;
    shell.screen_h = css_height;
    shell.boot().await.map_err(|err| format!("shell boot failed: {err}"))?;

    let runtime = RuntimeMailbox::new(AppRuntime {
        atlas,
        icons,
        icon_rebuild: None,
        icon_raster_scale: dpr,
        interaction: Some(AppInteractionState {
            shell,
            input: InputState::default(),
            theme: Theme::default(),
            theme_dark: appearance_is_dark("system"),
            last_pointer_x: 0.0,
            last_pointer_y: 0.0,
            pointer_down: false,
            pointer_button: 0,
            pointer_capture: PointerCapture::default(),
            modifiers: PointerModifiers::default(),
            space_pressed: false,
            wheel_zoom_deadline_ms: 0.0,
            text_streams: std::array::from_fn(|_| None),
            text_fault: None,
            frame_fault: None,
            text_cancel_pending: false,
            last_sync_pump_ms: 0.0,
        }),
        checkout: runtime_mailbox_core::InteractionCheckoutLedger::default(),
        draw: DrawList::default(),
        overlay: DrawList::default(),
        pending_frame_deferred: None,
        frame_actions: FrameActionOwners::default(),
        #[cfg(not(target_arch = "wasm32"))]
        pending_frame_maintenance_refusal: None,
        #[cfg(not(target_arch = "wasm32"))]
        plugin_modules_root: plugin_modules_root.clone(),
        #[cfg(not(target_arch = "wasm32"))]
        native_plugin_mtimes: std::collections::HashMap::new(),
        #[cfg(not(target_arch = "wasm32"))]
        native_hot_swap_scan: None,
        #[cfg(not(target_arch = "wasm32"))]
        native_hot_swap_modified: None,
        #[cfg(not(target_arch = "wasm32"))]
        native_hot_swap_cursor: 0,
        #[cfg(not(target_arch = "wasm32"))]
        native_reload_pending: false,
    });
    let presenter = AppPresenter {
        gpu,
        engine: engine_canvas::EngineCanvasPresenter::default(),
        gate: ui_wgpu::wgpu::PreparedRenderGate::default(),
        runtime: runtime.clone(),
        presentation_authority: runtime.presentation_authority(),
        raster_operation_authority: runtime.raster_operation_authority(),
        window: Some(window.clone()),
        #[cfg(target_arch = "wasm32")]
        offscreen_token: None,
        last_cursor: None,
        pending: None,
        retirement: None,
        retained_fault: None,
        surface_resize: None,
        stall: AppPresentStallWatch::default(),
    };

    log_debug("wgpu renderer booted");
    Ok((runtime, presenter))
}
//#endregion 📮️RuntimeMailbox
