//! 🖥️ Frame presentation: frame preparation, presenter state and the presenter implementation.
use super::*;

//#region 📮️RuntimeMailbox
#[derive(Clone,Copy,PartialEq,Eq)]
pub(crate) enum FramePreparationStep { Pending, Complete, Cancelled, Fault }

pub(crate) struct AppFramePreparation {
    pub(crate) retained: semio_framework_job::RetainedCloneGrant,
    pub(crate) params: Option<semio_framework_job_worker_authority::BatchJobParams>,
    pub(crate) params_cancel_retirement: Option<semio_framework_async::CancelTokenRetirement>,
    pub(crate) worker_semantic_pending: bool,
    pub(crate) outcome_pending: Option<FramePreparationStep>,
    pub(crate) outcome_acknowledged: bool,
    pub(crate) refused_receipt: Option<(RetainedCloneGrant, RetainedCloneProgress)>,
    pub(crate) job: Option<ui_wgpu::wgpu::PreparedRenderJob>,
    pub(crate) job_rejected: Option<ui_wgpu::wgpu::PreparedRenderJobRejected>,
    pub(crate) input_candidate: Option<shell::PresentedInputCandidateWitness>,
    pub(crate) session: Option<semio_framework_job_worker::BatchJobSession<ui_wgpu::wgpu::PreparedRenderJob>>,
    pub(crate) engine_packets: Option<FrameEnginePackets>,
    pub(crate) generation: semio_framework_trace::Generation,
    pub(crate) cursor: SemioCursor,
    pub(crate) theme_dark: bool,
    pub(crate) fullscreen: Option<bool>,
    pub(crate) cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) job_progress: Option<kernel_runtime::JobProgressPresentationLease>,
    pub(crate) terminal: bool,
    pub(crate) fault: Option<&'static str>,
}

/// 🪙️ Compares actual child demand with the five unchanged incoming frame axes.
fn frame_retirement_permits(grant: semio_framework_job::RetainedCloneGrant, demand: semio_framework_value::RetirementDemand) -> bool {
    grant.maximum_items != 0 && demand.copy_bytes <= grant.maximum_copy_bytes && demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.release_bytes <= grant.maximum_release_bytes && demand.depth <= grant.maximum_depth
}

impl AppFramePreparation {
    /// 🩺️ Why this preparation refused, for the ONE caller that must not swallow it.
    ///
    /// 🩸️ Every refusal here became an empty `StepOutcome::Fault`/`Cancelled`, and `ActiveFrameBuild`
    /// answered it by cancelling its own token — no fault recorded, no log, no quarantine. A single
    /// unbound raster producer therefore ended every frame build after the reference image decoded and
    /// the shell presented ONE frame per boot with a completely silent console (ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w7b-presenter-one-frame-per-boot.md`).
    pub(crate) fn fault(&self) -> Option<&'static str> {
        self.fault
    }

    pub(crate) fn callback_verdict(&self) -> Option<&semio_framework_trace::CallbackVerdict> {
        self.session.as_ref().and_then(semio_framework_job_worker::BatchJobSession::callback_verdict)
    }

    pub(crate) fn drive_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<FramePreparationStep, semio_framework_value::ValueError> {
        use semio_framework_job::{JobOutcomeView};
use semio_framework_job_worker::{WorkerJobPoll};
        use semio_framework_value::{ValueError,ValueRefusalKind};
        let refuse=|message|ValueError::literal(ValueRefusalKind::InvariantViolated,message);
        if self.refused_receipt.is_some() { return Err(refuse("original prepared worker receipt remains refused")); }
        if self.job_rejected.is_some() { return Err(refuse("original prepared render admission remains rejected")); }
        if self.job.is_some() && self.params.is_none() {
            let Some(cancel)=cx.admit_original_cancel_alias()? else { return Ok(FramePreparationStep::Pending); };
            self.params=Some(semio_framework_job_worker_authority::BatchJobParams { operation:cx.operation(),generation:cx.generation(),cancel,config:semio_framework_job_worker_authority::BatchDriveConfig { retained:self.retained,site:"os_renderer.prepare.worker",stage:semio_framework_job::InteractiveStage::BackgroundStep,fuel_per_step:1,step_budget_us:1000 },now_us:cx.clock_source() });
            return Ok(FramePreparationStep::Pending);
        }
        if self.job.is_some() {
            if let Some((session,_already_received))=semio_framework_job_worker::BatchJobSession::try_admit_owned(&mut self.job,&mut self.params,cx)? { self.session=Some(session); }
            return Ok(FramePreparationStep::Pending);
        }
        let session=self.session.as_mut().ok_or_else(||refuse("original prepared render session is absent"))?;
        if let Some(outcome)=self.outcome_pending {
            if !self.outcome_acknowledged {
                let step=session.acknowledge_outcome(cx.retained_grant());
                cx.consume_retained(step.progress())?;
                self.outcome_acknowledged=matches!(step,semio_framework_value::RetainedCloneStep::Complete(_));
                return Ok(FramePreparationStep::Pending);
            }
            if outcome==FramePreparationStep::Pending {
                session.resume().map_err(|_|refuse("original prepared worker resume was refused"))?;
                self.worker_semantic_pending=false;self.outcome_pending=None;self.outcome_acknowledged=false;
                return Ok(FramePreparationStep::Pending);
            }
            self.terminal=true;
            return Ok(outcome);
        }
        if !self.worker_semantic_pending {
            if !matches!(session.step(cx.retained_grant()),Ok(WorkerJobPoll::Outcome|WorkerJobPoll::Terminal)) || !session.checkout_outcome() { return Ok(FramePreparationStep::Pending); }
            if let Some((issued,progress))=session.take_checked_out_retained_step_receipt() {
                let remaining=cx.retained_grant();
                let expected=RetainedCloneGrant { maximum_items:remaining.maximum_items.min(self.retained.maximum_items),maximum_copy_bytes:remaining.maximum_copy_bytes.min(self.retained.maximum_copy_bytes),maximum_capacity_bytes:remaining.maximum_capacity_bytes.min(self.retained.maximum_capacity_bytes),maximum_release_bytes:remaining.maximum_release_bytes.min(self.retained.maximum_release_bytes),maximum_depth:remaining.maximum_depth.min(self.retained.maximum_depth) };
                self.worker_semantic_pending=true;
                let received=cx.consume_retained(progress);
                if issued!=expected || !progress.fits(issued) || received.is_err() { self.refused_receipt=Some((issued,progress));return Err(received.err().unwrap_or_else(||refuse("original prepared worker tuple differs from issued authority"))); }
                return Ok(FramePreparationStep::Pending);
            }
            return Err(refuse("original prepared worker omitted its checked-out physical receipt"));
        }
        self.outcome_pending=Some(match session.checked_out_outcome()? {
            None|Some(JobOutcomeView::Yield { .. })=>FramePreparationStep::Pending,
            Some(JobOutcomeView::Complete { .. })=>FramePreparationStep::Complete,
            Some(JobOutcomeView::Cancelled { .. })=>{self.fault=Some("prepared render job was cancelled");FramePreparationStep::Cancelled},
            Some(JobOutcomeView::Fault { .. }|JobOutcomeView::PreviewReady { .. }|JobOutcomeView::CheckpointReady { .. })=>{self.fault=Some("prepared render job refused its terminal candidate");FramePreparationStep::Fault},
        });
        Ok(FramePreparationStep::Pending)
    }

    pub(crate) fn take_presentation(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<AppFramePresentation>,semio_framework_value::ValueError> {
        if !self.terminal || !self.outcome_acknowledged {
            return Ok(None);
        }
        let grant=cx.retained_grant();
        if grant.maximum_items==0 || grant.maximum_depth==0 { return Ok(None); }
        if self.engine_packets.is_none() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original frame engine packets are absent")); }
        let Some(packet)=self.session.as_mut().and_then(|session|session.checked_out_job_mut()).and_then(|job|job.take_packet())else{return Ok(None)};
        cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;
        if let Some(session)=self.session.as_mut(){session.begin_close();}
        Ok(Some(AppFramePresentation {
            retained: self.retained,
            packet: Some(packet),
            input_candidate: self.input_candidate.take(),
            engine_packets: self.engine_packets.take().expect("original frame engine packets validated before publication"),
            generation: self.generation,
            cursor: self.cursor,
            theme_dark: self.theme_dark,
            fullscreen: self.fullscreen,
            cursor_wake: self.cursor_wake.take(),
            #[cfg(not(target_arch = "wasm32"))]
            job_progress: self.job_progress.take(),
        }))
    }

    pub(crate) fn discard_presented_input_candidate(&mut self, runtime: &RuntimeMailbox) -> bool {
        discard_frame_input_candidate(runtime, &mut self.input_candidate)
    }

    pub(crate) fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>, witness: Option<&semio_framework_job::CancelToken>) -> Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure> {
        let grant=cx.retained_grant();
        if cx.should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        if self.input_candidate.is_some() { return Ok(InteractiveJobCloseStep::Blocked); }
        if self.job_rejected.is_some() { return frame_receive_close(frame_unpriced_child(),cx); }
        if self.params.is_some() {
            if let Some(step)=frame_inline_gate(grant,0).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            let params=self.params.take().expect("original prepared params selected before paid transfer");
            self.params_cancel_retirement=Some(semio_framework_async::CancelTokenRetirement::from_token(params.cancel));
            return frame_receive_close(frame_inline_progress(0),cx);
        }
        if let Some(original)=self.params_cancel_retirement.as_mut() {
            if !original.terminal_is_empty() {
                let Some(witness)=witness else{return Ok(InteractiveJobCloseStep::Blocked)};
                return frame_receive_close(match original.return_alias_step(witness,grant) {
                    Ok(step)=>Ok(InteractiveJobCloseStep::Pending{progress:step.progress()}),
                    Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>Ok(InteractiveJobCloseStep::Blocked),
                    Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>Err(error),
                },cx);
            }
            if let Some(step)=frame_inline_gate(grant,0).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            self.params_cancel_retirement=None;
            return frame_receive_close(frame_inline_progress(0),cx);
        }
        if let Some(job) = self.job.as_mut() {
            use semio_framework_job::InteractiveJob;
            job.begin_close();
            if !job.terminal_is_empty() {
                return frame_received_child(job.close_step(cx), grant, job.terminal_is_empty());
            }
            let bytes = std::mem::size_of::<Option<ui_wgpu::wgpu::PreparedRenderJob>>();
            if let Some(step) = frame_inline_gate(grant, bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))? { return Ok(step); }
            self.job = None;
            return frame_receive_close(frame_inline_progress(bytes),cx);
        }
        if let Some(session) = self.session.as_mut() {
            if !matches!(session.poll(), semio_framework_job_worker::WorkerJobPoll::Closing | semio_framework_job_worker::WorkerJobPoll::TerminalEmpty) {
                session.begin_close();
                return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
            }
            if let Some(witness)=witness {
                match session.has_original_cancel_alias_witness(witness) {
                    Ok(true)=>return frame_receive_close(match session.return_original_cancel_alias_step(witness,grant) {
                        Ok(Some(step))=>Ok(InteractiveJobCloseStep::Pending{progress:step.progress()}),
                        Ok(None)|Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_))=>Ok(InteractiveJobCloseStep::Blocked),
                        Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error))=>Err(error),
                    },cx),
                    Ok(false)=>{},
                    Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_))=>return Ok(InteractiveJobCloseStep::Blocked),
                    Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error))=>return Err(semio_framework_job::OriginalTurnFailure::debit_original(error,cx)),
                }
            }
            if !session.terminal_is_empty() { let step=session.close_step(cx)?;return frame_worker_child_step(step, grant, session.terminal_is_empty()).map_err(semio_framework_job::OriginalTurnFailure::from); }
            let bytes = std::mem::size_of::<Option<semio_framework_job_worker::BatchJobSession<ui_wgpu::wgpu::PreparedRenderJob>>>();
            if let Some(step) = frame_inline_gate(grant, bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))? { return Ok(step); }
            self.session = None;
            return frame_receive_close(frame_inline_progress(bytes),cx);
        }
        if self.cursor_wake.is_some() {
            let bytes = std::mem::size_of::<Option<infinite_world::world::WorldCursorWakeToken>>();
            if let Some(step) = frame_inline_gate(grant, bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))? { return Ok(step); }
            self.cursor_wake = None;
            return frame_receive_close(frame_inline_progress(bytes),cx);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.job_progress.is_some() { return frame_receive_close(frame_unpriced_child(),cx); }
        if let Some(packets) = self.engine_packets.as_mut() {
            if !packets.terminal_is_empty() {
                if !packets.is_empty() { return frame_receive_close(frame_unpriced_child(),cx); }
                return frame_receive_close(frame_child_step(packets.close_backing_step(grant), grant, packets.terminal_is_empty()),cx);
            }
            let bytes = std::mem::size_of::<Option<FrameEnginePackets>>();
            if let Some(step) = frame_inline_gate(grant, bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))? { return Ok(step); }
            self.engine_packets = None;
            return frame_receive_close(frame_inline_progress(bytes),cx);
        }
        if self.worker_semantic_pending || self.outcome_pending.is_some() || self.outcome_acknowledged || self.refused_receipt.is_some() {
            if let Some(step)=frame_inline_gate(grant,0).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?{return Ok(step)}
            self.worker_semantic_pending=false;self.outcome_pending=None;self.outcome_acknowledged=false;self.refused_receipt=None;
            return frame_receive_close(frame_inline_progress(0),cx);
        }
        Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() })
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.params.is_none() && self.params_cancel_retirement.is_none() && !self.worker_semantic_pending && self.outcome_pending.is_none() && !self.outcome_acknowledged && self.refused_receipt.is_none() && self.job.is_none() && self.job_rejected.is_none() && self.session.is_none() && self.engine_packets.is_none() && self.input_candidate.is_none() && self.cursor_wake.is_none() && {
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.job_progress.is_none()
            }
            #[cfg(target_arch = "wasm32")]
            {
                true
            }
        }
    }
}

pub(crate) struct AppPresenter {
    pub(crate) gpu: GpuContext,
    pub(crate) engine: engine_canvas::EngineCanvasPresenter,
    pub(crate) gate: ui_wgpu::wgpu::PreparedRenderGate,
    pub(crate) runtime: RuntimeMailbox,
    pub(crate) presentation_authority: RuntimePresentationAuthority,
    pub(crate) raster_operation_authority: RuntimeRasterOperationAuthority,
    pub(crate) window: Option<Arc<Window>>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) offscreen_token: Option<ui_wgpu::wgpu::OffscreenPresentToken>,
    pub(crate) last_cursor: Option<(SemioCursor, bool)>,
    pub(crate) pending: Option<AppPresentCursor>,
    pub(crate) retirement: Option<AppPresentedRetirement>,
    pub(crate) retained_fault: Option<String>,
    pub(crate) surface_resize: Option<AppSurfaceResizeCursor>,
    pub(crate) stall: AppPresentStallWatch,
}

/// 🐕️ The presentation ladder's own watchdog: the last progress signature [`AppPresenter::present_step`]
/// answered `Pending` on, and how many consecutive steps have answered it unchanged.
///
/// ⚖️ `admit_next_frame` refuses to build a frame while `has_pending_presentation()`, so a cursor that
/// stops advancing stops the whole host — no frame transaction, no runtime mailbox pump, no world
/// snapshot apply, no retained document ingress, no accessibility projection. Measured on 6118 as
/// `os_host frame gate blocked=true pending=true phase=Some(Engine)` held from t≈85 s to t≈200 s, and
/// as `phase=Some(Aborted) retained-fault=true` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-wheel-zoom-a11y-live-2026-09-14.md` §3.4, `📓️wgpu-host-settle-pump-2026-09-14.md` §7).
///
/// The signature is every index a HEALTHY step moves — the phase, the engine packet, the upload page,
/// and whether a GPU cursor is held — so a legitimately long presentation (many upload pages, many
/// engine packets) resets the count on every one of them and only a genuinely frozen cursor
/// accumulates.
#[derive(Default)]
pub(crate) struct AppPresentStallWatch {
    signature: Option<AppPresentProgress>,
    pub(crate) steps: u32,
}

/// 🐕️ Every index a healthy presentation step moves: the ladder phase, the engine packet, the upload
/// page, and — because `AppPresentPhase::Render` holds ONE `gpu_cursor` for a whole composite pass —
/// that cursor's own `(phase, command, clip piece, blur mip)`. Without the inner four the outer
/// shape is frozen for as many steps as the scene has commands, and a watchdog reading the outer
/// shape alone aborts a healthy present: measured on 6118 as
/// `os_host present stalled phase=Render engine=1 upload=1 gpu-cursor=true` at t≈4.4 s on EVERY
/// example boot (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum AppPresentInputWait {
    #[default]
    None,
    RuntimeLock,
    InteractionCheckout {
        site: &'static str,
        request: &'static str,
    },
    SceneIntent,
}

impl AppPresentInputWait {
    fn awaiting_runtime(self, phase: AppPresentPhase) -> bool {
        matches!(phase, AppPresentPhase::Render | AppPresentPhase::Acknowledge) && matches!(self, Self::InteractionCheckout { .. })
    }
}

pub(crate) type AppPresentProgress = (AppPresentPhase, usize, usize, Option<(u8, usize, usize, u32)>, (u32, u32, usize), usize, AppPresentInputWait, u64);

/// 🐕️ Consecutive non-advancing `Pending` answers after which a pending presentation is aborted
/// rather than waited on. `present_step` is driven several times per frame transaction turn, so this
/// is well under a second of wall clock and three orders of magnitude above the longest healthy
/// non-advancing run this ladder has (`Uploads`/`Render` answering `Ok(false)` while the GPU queue
/// drains).
pub(crate) const APP_PRESENT_STALL_STEPS: u32 = 4_096;

/// 🐕️ The watchdog's whole arithmetic, over the progress signature alone: `Some(shape)` on exactly the
/// step a signature has repeated [`APP_PRESENT_STALL_STEPS`] times, `None` on every other step, and a
/// reset the moment any term of the signature moves.
pub(crate) fn note_present_stall_signature(watch: &mut AppPresentStallWatch, signature: AppPresentProgress) -> Option<String> {
    if signature.6.awaiting_runtime(signature.0) {
        watch.signature = None;
        watch.steps = 0;
        return None;
    }
    if watch.signature != Some(signature) {
        watch.signature = Some(signature);
        watch.steps = 0;
        return None;
    }
    watch.steps = watch.steps.saturating_add(1);
    if watch.steps != APP_PRESENT_STALL_STEPS {
        return None;
    }
    let mut shape = format!("phase={:?} engine={} upload={} gpu-cursor={:?} upload-progress={:?}", signature.0, signature.1, signature.2, signature.3, signature.4);
    if signature.6 != AppPresentInputWait::None {
        shape.push_str(&format!(" input-wait={:?}", signature.6));
    }
    Some(shape)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AppSurfaceResizePhase {
    Apply,
    InvalidateEngine,
    ApplyGpu,
    Retire,
    Complete,
}

struct AppSurfaceResizeCursor {
    candidate: Option<surface_lane::PreparedSurfaceResize>,
    phase: AppSurfaceResizePhase,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AppPresentPhase {
    Fullscreen,
    Engine,
    BeginGpu,
    Ownership,
    Uploads,
    Stage,
    Render,
    CloseGpu,
    Acknowledge,
    Media,
    ProgressAcknowledge,
    Aborted,
    Directives,
}

impl AppPresentPhase {
    pub(crate) fn latency_stage(self) -> frame_latency::FrameLatencyStage {
        match self {
            Self::Fullscreen => frame_latency::FrameLatencyStage::PresenterFullscreen,
            Self::Engine => frame_latency::FrameLatencyStage::PresenterEngine,
            Self::BeginGpu => frame_latency::FrameLatencyStage::PresenterBeginGpu,
            Self::Ownership => frame_latency::FrameLatencyStage::PresenterOwnership,
            Self::Uploads => frame_latency::FrameLatencyStage::PresenterUploads,
            Self::Stage => frame_latency::FrameLatencyStage::PresenterStage,
            Self::Render => frame_latency::FrameLatencyStage::QueueSubmit,
            Self::CloseGpu => frame_latency::FrameLatencyStage::PresenterCloseGpu,
            Self::Acknowledge => frame_latency::FrameLatencyStage::PresenterAcknowledge,
            Self::Media => frame_latency::FrameLatencyStage::PresenterAcknowledge,
            Self::ProgressAcknowledge => frame_latency::FrameLatencyStage::PresenterProgressAcknowledge,
            Self::Aborted => frame_latency::FrameLatencyStage::PresenterAborted,
            Self::Directives => frame_latency::FrameLatencyStage::PresenterDirectives,
        }
    }
}

struct AppPresentCursor {
    media_collect:crate::media_slots::original_collect::OriginalPresentedRoster,
    media_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<Vec<crate::media_slots::PresentedMediaSlot>>>,
    media_slots: Vec<crate::media_slots::PresentedMediaSlot>,
    has_animated_primitives: bool,
    frame: AppFramePresentation,
    phase: AppPresentPhase,
    engine: usize,
    upload: usize,
    witness: Option<ui_wgpu::wgpu::PreparedPresenterWitness>,
    raster_witness: Option<ui_wgpu::wgpu::RasterTextureWitness>,
    raster_keep: ui_wgpu::wgpu::PreparedRasterKeepCursorV1,
    raster_keep_steps: usize,
    input_wait: AppPresentInputWait,
    input_progress: u64,
    gpu_cursor: Option<ui_wgpu::wgpu::PreparedGpuPresentCursor>,
    retained_control_deadline: Option<f64>,
    shell_clock_deadline: Option<f64>,
}

struct AppPresentedRetirement {
    previous: Option<ui_wgpu::wgpu::PreparedRenderPacket>,
    completed_frame: Option<AppFramePresentation>,
    raster: RasterCandidateRetirement,
    acknowledged_eviction: usize,
    acknowledged_upload_scan: usize,
    acknowledged_versions: [u64; ui_wgpu::wgpu::MESH_GPU_KEEP_VERSION_CAPACITY],
    acknowledged_version_count: usize,
    release_previous_raster_ownership: bool,
    unowned_rasters_retired: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RasterCandidateRetirement {
    Infer,
    Abort(ui_wgpu::wgpu::RasterTextureWitness),
    Commit(ui_wgpu::wgpu::RasterTextureWitness),
    Complete,
}

pub(crate) enum AppPresentStep {
    Idle,
    Pending,
    AwaitingRuntime,
    RetryRuntime,
    Complete {
        generation: semio_framework_trace::Generation,
        cursor: SemioCursor,
        theme_dark: bool,
        fullscreen: Option<bool>,
        cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>,
        retained_control_deadline: Option<f64>,
        shell_clock_deadline: Option<f64>,
        has_animated_primitives: bool,
        media_slots: Vec<crate::media_slots::PresentedMediaSlot>,
    },
}

impl AppFramePresentation {
    pub(crate) fn discard_presented_input_candidate(&mut self, runtime: &RuntimeMailbox) -> bool {
        discard_frame_input_candidate(runtime, &mut self.input_candidate)
    }

    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep,semio_framework_value::ValueError> {
        if self.input_candidate.is_some() { return Ok(InteractiveJobCloseStep::Blocked); }
        if let Some(packet) = self.packet.as_mut() {
            if !packet.retirement_is_empty() { return frame_child_step(packet.close_step(grant), grant, packet.retirement_is_empty()); }
            let bytes = std::mem::size_of::<Option<ui_wgpu::wgpu::PreparedRenderPacket>>();
            if let Some(step) = frame_inline_gate(grant, bytes)? { return Ok(step); }
            self.packet = None;
            return frame_inline_progress(bytes);
        }
        if !self.engine_packets.terminal_is_empty() {
            if !self.engine_packets.is_empty() { return frame_unpriced_child(); }
            return frame_child_step(self.engine_packets.close_backing_step(grant), grant, self.engine_packets.terminal_is_empty());
        }
        if self.cursor_wake.is_some() {
            let bytes = std::mem::size_of::<Option<infinite_world::world::WorldCursorWakeToken>>();
            if let Some(step) = frame_inline_gate(grant, bytes)? { return Ok(step); }
            self.cursor_wake = None;
            return frame_inline_progress(bytes);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.job_progress.is_some() { return frame_unpriced_child(); }
        Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() })
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.packet.is_none() && self.input_candidate.is_none() && self.engine_packets.terminal_is_empty() && self.cursor_wake.is_none() && {
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.job_progress.is_none()
            }
            #[cfg(target_arch = "wasm32")]
            {
                true
            }
        }
    }
}

impl AppPresentedRetirement {
    fn new(previous: Option<ui_wgpu::wgpu::PreparedRenderPacket>) -> Self {
        Self {
            previous,
            completed_frame: None,
            raster: RasterCandidateRetirement::Infer,
            acknowledged_eviction: 0,
            acknowledged_upload_scan: 0,
            acknowledged_versions: [0; ui_wgpu::wgpu::MESH_GPU_KEEP_VERSION_CAPACITY],
            acknowledged_version_count: 0,
            release_previous_raster_ownership: false,
            unowned_rasters_retired: false,
        }
    }

    fn abort(previous: ui_wgpu::wgpu::PreparedRenderPacket, witness: ui_wgpu::wgpu::RasterTextureWitness) -> Self {
        let raster = RasterCandidateRetirement::Abort(witness);
        Self {
            previous: Some(previous),
            completed_frame: None,
            raster,
            acknowledged_eviction: 0,
            acknowledged_upload_scan: 0,
            acknowledged_versions: [0; ui_wgpu::wgpu::MESH_GPU_KEEP_VERSION_CAPACITY],
            acknowledged_version_count: 0,
            release_previous_raster_ownership: false,
            unowned_rasters_retired: false,
        }
    }

    fn commit(previous: Option<ui_wgpu::wgpu::PreparedRenderPacket>, witness: ui_wgpu::wgpu::RasterTextureWitness) -> Self {
        Self {
            previous,
            completed_frame: None,
            raster: RasterCandidateRetirement::Commit(witness),
            acknowledged_eviction: 0,
            acknowledged_upload_scan: 0,
            acknowledged_versions: [0; ui_wgpu::wgpu::MESH_GPU_KEEP_VERSION_CAPACITY],
            acknowledged_version_count: 0,
            release_previous_raster_ownership: true,
            unowned_rasters_retired: false,
        }
    }

    fn step(&mut self, gpu: &mut GpuContext, gate: &ui_wgpu::wgpu::PreparedRenderGate) -> Result<bool, String> {
        if self.raster == RasterCandidateRetirement::Infer {
            self.raster = RasterCandidateRetirement::Complete;
            return Ok(false);
        }
        match self.raster {
            RasterCandidateRetirement::Abort(witness) => {
                if !gpu.abort_presented_rasters_step(witness)? {
                    return Ok(false);
                }
                self.raster = RasterCandidateRetirement::Complete;
                return Ok(false);
            }
            RasterCandidateRetirement::Commit(witness) => {
                if !gpu.commit_presented_rasters_step(witness)? {
                    return Ok(false);
                }
                self.raster = RasterCandidateRetirement::Complete;
                return Ok(false);
            }
            RasterCandidateRetirement::Infer | RasterCandidateRetirement::Complete => {}
        }
        if !gpu.close_mesh_upload_step() {
            return Ok(false);
        }
        if let Some(packet) = gate.last_valid() {
            if self.acknowledged_eviction < packet.evictions().len() {
                let Some(eviction) = packet.evictions().get(self.acknowledged_eviction) else { return Err("prepared eviction cursor exceeded its fixed list".to_string()) };
                let key = match eviction {
                    ui_wgpu::wgpu::PreparedRenderEviction::Mesh { key } => key,
                };
                if let Some(upload) = packet.uploads().get(self.acknowledged_upload_scan) {
                    if let ui_wgpu::wgpu::PreparedRenderUpload::Mesh { key: upload_key, version, .. } = upload {
                        if upload_key == key {
                            let Some(slot) = self.acknowledged_versions.get_mut(self.acknowledged_version_count) else {
                                return Err("prepared mesh eviction keep-version credits exhausted".to_string());
                            };
                            *slot = *version;
                            self.acknowledged_version_count += 1;
                        }
                    }
                    self.acknowledged_upload_scan += 1;
                    return Ok(false);
                }
                if gpu.apply_prepared_eviction_step(packet, self.acknowledged_eviction, &self.acknowledged_versions[..self.acknowledged_version_count])? {
                    self.acknowledged_eviction += 1;
                    self.acknowledged_upload_scan = 0;
                    self.acknowledged_version_count = 0;
                }
                return Ok(false);
            }
        }
        if let Some(previous) = self.previous.as_mut() {
            if !previous.retire_step() {
                return Ok(false);
            }
            self.previous = None;
            return Ok(false);
        }
        if self.release_previous_raster_ownership {
            gpu.release_previous_raster_ownership();
            self.release_previous_raster_ownership = false;
            return Ok(false);
        }
        if !self.unowned_rasters_retired {
            if !gpu.retire_unowned_raster_step()? {
                return Ok(false);
            }
            self.unowned_rasters_retired = true;
            return Ok(false);
        }
        if let Some(frame) = self.completed_frame.as_mut() {
            if !matches!(frame.close_step(frame.retained), InteractiveJobCloseStep::Complete { .. }) {
                return Ok(false);
            }
            self.completed_frame = None;
            return Ok(false);
        }
        Ok(true)
    }

    fn terminal_is_empty(&self) -> bool {
        self.raster == RasterCandidateRetirement::Complete && self.previous.is_none() && !self.release_previous_raster_ownership && self.unowned_rasters_retired && self.completed_frame.as_ref().is_none_or(AppFramePresentation::terminal_is_empty)
    }
}

impl AppPresenter {
    pub(crate) fn surface_resize_available(&self) -> bool {
        self.surface_resize.is_none()
    }

    pub(crate) fn begin_surface_resize(&mut self, candidate: surface_lane::PreparedSurfaceResize) -> Result<(), surface_lane::PreparedSurfaceResize> {
        if self.surface_resize.is_some() {
            return Err(candidate);
        }
        self.surface_resize = Some(AppSurfaceResizeCursor { candidate: Some(candidate), phase: AppSurfaceResizePhase::Apply });
        Ok(())
    }

    pub(crate) fn surface_resize_step(&mut self) -> bool {
        let Some(cursor) = self.surface_resize.as_mut() else { return true };
        match cursor.phase {
            AppSurfaceResizePhase::Apply => {
                let Some(candidate) = cursor.candidate else {
                    cursor.phase = AppSurfaceResizePhase::Complete;
                    return false;
                };
                if !self.engine.observe_primary_metrics_generation(candidate.metrics_generation()) {
                    return false;
                }
                cursor.phase = AppSurfaceResizePhase::InvalidateEngine;
                false
            }
            AppSurfaceResizePhase::InvalidateEngine => {
                if !self.engine.invalidate_primary_metrics_step() {
                    return false;
                }
                cursor.phase = AppSurfaceResizePhase::ApplyGpu;
                false
            }
            AppSurfaceResizePhase::ApplyGpu => {
                let Some(candidate) = cursor.candidate else {
                    cursor.phase = AppSurfaceResizePhase::Complete;
                    return false;
                };
                if !candidate.suspended() {
                    self.gpu.resize(candidate.logical_width(), candidate.logical_height(), candidate.scale_factor());
                }
                cursor.phase = AppSurfaceResizePhase::Retire;
                false
            }
            AppSurfaceResizePhase::Retire => {
                cursor.candidate.take();
                cursor.phase = AppSurfaceResizePhase::Complete;
                false
            }
            AppSurfaceResizePhase::Complete => {
                self.surface_resize = None;
                true
            }
        }
    }

    pub(crate) fn close_surface_resize_step(&mut self) -> bool {
        if let Some(cursor) = self.surface_resize.as_mut() {
            if cursor.candidate.take().is_some() {
                cursor.phase = AppSurfaceResizePhase::Retire;
                return false;
            }
            cursor.phase = AppSurfaceResizePhase::Complete;
            self.surface_resize = None;
            return false;
        }
        self.engine.invalidate_primary_metrics_step()
    }

    pub(crate) fn has_pending_presentation(&self) -> bool {
        self.pending.is_some() || self.retirement.is_some() || self.retained_fault.is_some() || self.gate.has_pending_acknowledgement()
    }

    pub(crate) fn awaiting_runtime(&self) -> bool {
        self.pending.as_ref().is_some_and(|cursor| cursor.input_wait.awaiting_runtime(cursor.phase))
    }

    pub(crate) fn holds_presented_input_publication(&self) -> bool {
        self.pending.as_ref().is_some_and(|cursor| matches!(cursor.phase, AppPresentPhase::Render | AppPresentPhase::CloseGpu | AppPresentPhase::Acknowledge | AppPresentPhase::Media))
    }

    /// 🩺️ Which of the four owners is holding the presentation gate shut — the gate that decides
    /// whether `admit_next_frame` runs a frame build at all, and therefore whether the runtime mailbox
    /// is pumped this tick.
    pub(crate) fn presentation_gate_shape(&self) -> String {
        format!(
            "pending={} phase={:?} retirement={} retained-fault={:?} gate-ack={} stall-steps={}",
            self.pending.is_some(),
            self.pending.as_ref().map(|cursor| cursor.phase),
            self.retirement.is_some(),
            self.retained_fault.as_deref(),
            self.gate.has_pending_acknowledgement(),
            self.stall.steps
        )
    }

    /// 🐕️ One watchdog tick over the pending presentation: `Some(shape)` exactly once, on the step the
    /// cursor has answered [`APP_PRESENT_STALL_STEPS`] consecutive non-advancing `Pending`s. See
    /// [`AppPresentStallWatch`].
    fn note_present_stall(watch: &mut AppPresentStallWatch, cursor: &AppPresentCursor, upload_progress: (u32, u32, usize)) -> Option<String> {
        note_present_stall_signature(
            watch,
            (cursor.phase, cursor.engine, cursor.upload, cursor.gpu_cursor.as_ref().map(ui_wgpu::wgpu::PreparedGpuPresentCursor::progress), upload_progress, cursor.raster_keep_steps, cursor.input_wait, cursor.input_progress),
        )
    }

    pub(crate) fn close_cursor_wake_step(&mut self) -> bool {
        let Some(pending) = self.pending.as_mut() else { return true };
        if pending.frame.cursor_wake.take().is_some() {
            return false;
        }
        true
    }

    pub(crate) fn close_original_media_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let Some(cursor)=self.pending.as_mut()else{return Ok(true)};
        if !cursor.media_collect.terminal_is_empty(){if !crate::interpreter::close_presented_media_slots(&mut cursor.media_collect,cx)?{return Ok(false)}}
        if let Some(owner)=cursor.media_close.as_mut(){if owner.terminal_is_empty(){let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}cursor.media_close=None;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}let demand=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;let _=semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok(false)}
        if !cursor.media_slots.is_empty(){let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}let original=std::mem::take(&mut cursor.media_slots);match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){Ok(owner)=>cursor.media_close=Some(owner),Err((original,source))=>{cursor.media_slots=source;return Err(original.into())}}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}Ok(true)
    }

    pub(crate) fn close_frame_owners_step(&mut self) -> Result<bool, String> {
        if !self.engine.close_active_candidate_step(&mut self.gpu)? {
            return Ok(false);
        }
        if let Some(mut cursor) = self.pending.take() {
            if let Some(input_candidate) = cursor.frame.input_candidate {
                let runtime = self.runtime.clone();
                let Ok(mut runtime) = runtime.try_lock() else {
                    self.pending = Some(cursor);
                    return Ok(false);
                };
                let Some(interaction) = runtime.interaction.as_mut() else {
                    self.pending = Some(cursor);
                    return Ok(false);
                };
                if !interaction.shell.discard_presented_input_candidate(input_candidate) {
                    self.pending = Some(cursor);
                    return Err("presenter close lost its staged input witness".to_string());
                }
                cursor.frame.input_candidate = None;
                self.pending = Some(cursor);
                return Ok(false);
            }
            if let Some(gpu_cursor) = cursor.gpu_cursor.as_mut() {
                gpu_cursor.begin_close();
                if !gpu_cursor.close_step() {
                    self.pending = Some(cursor);
                    return Ok(false);
                }
                cursor.gpu_cursor = None;
                self.pending = Some(cursor);
                return Ok(false);
            }
            if cursor.frame.packet.is_none() {
                cursor.frame.packet = self.gate.abort_pending();
            }
            let retirement = self.retirement.get_or_insert_with(|| AppPresentedRetirement::new(None));
            if retirement.completed_frame.is_some() {
                self.pending = Some(cursor);
                return Err("presenter close retirement capacity exhausted".to_string());
            }
            if let Some(witness) = cursor.raster_witness.take() {
                if let Err(error) = self.raster_operation_authority.release(witness) {
                    cursor.raster_witness = Some(witness);
                    self.pending = Some(cursor);
                    return Err(error.to_string());
                }
                retirement.raster = RasterCandidateRetirement::Abort(witness);
            }
            retirement.completed_frame = Some(cursor.frame);
            return Ok(false);
        }
        if let Some(packet) = self.gate.abort_pending() {
            if self.retirement.is_some() {
                return Err("presenter pending packet cannot overtake retirement".to_string());
            }
            let witness = self.raster_operation_authority.current().ok_or_else(|| "presenter raster operation witness was missing during close".to_string())?;
            self.raster_operation_authority.release(witness).map_err(str::to_owned)?;
            self.retirement = Some(AppPresentedRetirement::abort(packet, witness));
            return Ok(false);
        }
        if let Some(retirement) = self.retirement.as_mut() {
            if !retirement.step(&mut self.gpu, &self.gate)? {
                return Ok(false);
            }
            self.retirement = None;
            return Ok(false);
        }
        if let Some(fault) = self.retained_fault.as_mut() {
            if fault.pop().is_some() {
                return Ok(false);
            }
            self.retained_fault = None;
            return Ok(false);
        }
        if let Some(packet) = self.gate.take_last_valid() {
            self.retirement = Some(AppPresentedRetirement::new(Some(packet)));
            return Ok(false);
        }
        if !self.gate.close_step() {
            return Ok(false);
        }
        Ok(self.pending.is_none() && self.retirement.as_ref().is_none_or(AppPresentedRetirement::terminal_is_empty) && self.retained_fault.is_none() && self.gate.terminal_is_empty() && self.raster_operation_authority.current().is_none())
    }

    pub(crate) fn close_world_owners_step(&mut self) -> Result<bool, String> {
        if !self.close_frame_owners_step()? || !self.engine.terminal_is_empty() {
            return Ok(false);
        }
        if !self.gpu.close_mesh_upload_step() {
            return Ok(false);
        }
        if !self.gpu.close_mesh_table_step() {
            return Ok(false);
        }
        if !self.gpu.close_raster_table_step()? {
            return Ok(false);
        }
        Ok(self.world_owners_terminal_is_empty())
    }

    pub(crate) fn world_owners_terminal_is_empty(&self) -> bool {
        self.pending.is_none()
            && self.retirement.as_ref().is_none_or(AppPresentedRetirement::terminal_is_empty)
            && self.gate.terminal_is_empty()
            && self.gpu.mesh_upload_terminal_is_empty()
            && self.gpu.mesh_table_terminal_is_empty()
            && self.gpu.raster_table_terminal_is_empty()
            && self.raster_operation_authority.current().is_none()
            && self.engine.terminal_is_empty()
            && self.retained_fault.is_none()
    }

    pub(crate) fn admit_next_frame(&mut self, produce: impl FnOnce() -> Option<AppFramePresentation>) -> Option<SemioCursor> {
        if self.has_pending_presentation() {
            return None;
        }
        let frame = produce()?;
        let cursor = frame.cursor;
        self.pending = Some(AppPresentCursor {
            media_collect:Default::default(),
            media_close:None,
            media_slots: Vec::new(),
            has_animated_primitives: false,
            frame,
            phase: AppPresentPhase::BeginGpu,
            engine: 0,
            upload: 0,
            witness: None,
            raster_witness: None,
            raster_keep: ui_wgpu::wgpu::PreparedRasterKeepCursorV1::default(),
            raster_keep_steps: 0,
            input_wait: AppPresentInputWait::None,
            input_progress: 0,
            gpu_cursor: None,
            retained_control_deadline: None,
            shell_clock_deadline: None,
        });
        Some(cursor)
    }

    /// 🎬️ Advances the pending presentation by exactly one phase.
    ///
    /// **Freshness is decided ONCE, by the BUILD.** `AppPresentPhase::BeginGpu` reads
    /// `presentation_authority.admitted()` — the pair the frame build declared when it minted its
    /// `FrameBuildCursor`, never the live `current()` — and hands it to
    /// `begin_prepared`/`begin_prepared_offscreen`, whose `PreparedRenderGate::validate` refuses a
    /// packet whose `scene_revision`/`preview_generation` does not match that admitted pair; the same
    /// pair is then frozen into the cursor's `RasterTextureWitness`. Every later phase therefore
    /// compares against THAT witness and never against the authority again: it is a moving target — `OsHost`
    /// re-publishes `observe_presentation_input_generation(frame_generation)` on every
    /// `build_and_publish_snapshot`, and `🌐️browser-worker`'s `enqueueBatch` additionally rebases
    /// `frame_generation` onto the UI isolate's input generation — so a presentation that legitimately
    /// spans more than one host tick (the browser's own interactive-ceiling loop guarantees it will)
    /// would observe a different authority at acknowledgement than the one that admitted it and fault a
    /// frame that is ALREADY submitted to the GPU. That is what took the wgpu browser surface down on
    /// the first pointer event (`worker-present-failed: prepared frame authority was stale before
    /// acknowledgement`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END): a superseded frame must be
    /// acknowledged and replaced by the next one, never turned into a surface fault.
    pub(crate) fn present_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<AppPresentStep, semio_framework_job::OriginalTurnFailure> {
        if cx.should_yield(){return Ok(AppPresentStep::Pending)}
        if let Some(cursor)=self.pending.as_mut().filter(|cursor|cursor.phase==AppPresentPhase::Media){
            let Ok(runtime)=self.runtime.try_lock()else{return Ok(AppPresentStep::RetryRuntime)};let Some(interaction)=runtime.interaction.as_ref()else{return Ok(AppPresentStep::AwaitingRuntime)};if !interaction.shell.drive_presented_media_slots(&mut cursor.media_collect,&mut cursor.media_slots,cx)?{return Ok(AppPresentStep::Pending)}cursor.phase=AppPresentPhase::ProgressAcknowledge;return Ok(AppPresentStep::Pending)
        }
        let owner = self.pending.as_ref().map(|cursor| (cursor.frame.generation.0, cursor.phase.latency_stage())).or_else(|| {
            self.has_pending_presentation().then(|| {
                let generation = self.retirement.as_ref().and_then(|retirement| retirement.completed_frame.as_ref()).map(|frame| frame.generation.0).unwrap_or_else(frame_latency::latest_frame_generation);
                (generation, frame_latency::FrameLatencyStage::PresenterRetirement)
            })
        });
        let _latency = owner.map(|(generation, stage)| frame_latency::FrameLatencyTimer::start(frame_latency::FrameLatencyAuthority::renderer_frame(generation), stage, 1));
        self.present_step_inner(cx).map_err(|source|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,source).into())
    }

    /// 🩺️ The stored string is only ever surfaced by this fn's `Err` return,
    /// and that return is unreachable while `pending` is still held by the
    /// `Aborted` phase this line just entered — so an abort that never
    /// finishes closing its candidate reported NOTHING at all, leaving the
    /// host's own `frame gate blocked=true phase=Some(Aborted)
    /// retained-fault=true` as the only trace and the REASON invisible
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️wgpu-wheel-zoom-a11y-live-2026-09-14.md` §3.4a).
    ///
    /// 🎟️ The pair the BUILD was admitted under, never `current()` — see
    /// `RuntimePresentationAuthority::admit_build` for the surface fault that re-reading a
    /// moving authority here produced.
    ///
    /// 🧷️ A prepared world draw whose mesh is not resident is SKIPPED, never fatal — the
    /// parity behaviour of React, which renders nothing until its loader resolves. It is
    /// still a residency-law breach on the build side, so it is reported on the transition
    /// rather than swallowed (`retain_ensured_world_draws`, `📓️w11a-…`).
    fn present_step_inner(&mut self,_cx:&mut semio_framework_job::StepContext<'_>) -> Result<AppPresentStep, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = RENDERER_IO_WAKE.swap(false, Ordering::AcqRel);
            let _ = pump_renderer_io_sessions(1);
        }
        let _ = semio_framework_job_worker::pump_worker_job_retirements(1, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        if self.pending.is_none() {
            if let Some(retirement) = self.retirement.as_mut() {
                if retirement.step(&mut self.gpu, &self.gate)? {
                    self.retirement = None;
                }
                return Ok(AppPresentStep::Pending);
            }
            if let Some(fault) = self.retained_fault.take() {
                return Err(fault);
            }
            return Ok(AppPresentStep::Idle);
        }
        let upload_progress = self.gpu.prepared_upload_progress();
        let Some(cursor) = self.pending.as_mut() else {
            self.stall = AppPresentStallWatch::default();
            return Ok(AppPresentStep::Idle);
        };
        if let Some(shape) = Self::note_present_stall(&mut self.stall, cursor, upload_progress) {
            log_debug(&format!("[TRACE] os_host present stalled {shape} retained-fault={:?}", self.retained_fault));
            if !matches!(cursor.phase, AppPresentPhase::Aborted) {
                if self.retained_fault.is_none() {
                    self.retained_fault = Some(format!("presentation stalled: {shape}"));
                }
                cursor.phase = AppPresentPhase::Aborted;
                return Ok(AppPresentStep::Pending);
            }
        }
        match cursor.phase {
            AppPresentPhase::Aborted => {
                if !self.engine.close_active_candidate_step(&mut self.gpu)? {
                    return Ok(AppPresentStep::Pending);
                }
                if let Some(gpu_cursor) = cursor.gpu_cursor.as_mut() {
                    gpu_cursor.begin_close();
                    if !gpu_cursor.close_step() {
                        return Ok(AppPresentStep::Pending);
                    }
                    cursor.gpu_cursor = None;
                    return Ok(AppPresentStep::Pending);
                }
                if let Some(input_candidate) = cursor.frame.input_candidate {
                    let runtime = self.runtime.clone();
                    let Ok(mut runtime) = runtime.try_lock() else { return Ok(AppPresentStep::Pending) };
                    let Some(interaction) = runtime.interaction.as_mut() else { return Ok(AppPresentStep::Pending) };
                    if !interaction.shell.discard_presented_input_candidate(input_candidate) {
                        return Err("aborted presentation lost its staged input witness".to_string());
                    }
                    cursor.frame.input_candidate = None;
                }
                let Some(mut aborted) = self.pending.take() else { return Err("aborted presentation cursor was missing".to_string()) };
                let retirement = self.retirement.get_or_insert_with(|| AppPresentedRetirement::new(None));
                if retirement.completed_frame.is_some() {
                    self.pending = Some(aborted);
                    return Err("aborted presentation retirement capacity exhausted".to_string());
                }
                if let Some(witness) = aborted.raster_witness.take() {
                    if let Err(error) = self.raster_operation_authority.release(witness) {
                        aborted.raster_witness = Some(witness);
                        self.pending = Some(aborted);
                        return Err(error.to_string());
                    }
                    retirement.raster = RasterCandidateRetirement::Abort(witness);
                }
                retirement.completed_frame = Some(aborted.frame);
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Fullscreen => {
                if let Some(active) = cursor.frame.fullscreen {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if let Some(window) = self.window.as_ref() {
                            window.set_fullscreen(if active { Some(Fullscreen::Borderless(None)) } else { None });
                        }
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        use winit::platform::web::WindowExtWebSys;
                        if let Some(canvas) = self.window.as_ref().and_then(|window| window.canvas()) {
                            let document = canvas.owner_document();
                            if active {
                                if let Err(error) = canvas.request_fullscreen() {
                                    web_sys::console::error_2(&"Fullscreen request was rejected".into(), &error);
                                }
                            } else if let Some(document) = document {
                                document.exit_fullscreen();
                            }
                        }
                    }
                }
                cursor.phase = AppPresentPhase::Directives;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Engine => {
                if let Some(packet) = cursor.frame.engine_packets.get(cursor.engine) {
                    let candidate = cursor.raster_witness.ok_or_else(|| "engine raster operation witness was missing".to_string())?;
                    let expected = self.raster_operation_authority.current().ok_or_else(|| "engine raster operation authority was empty".to_string())?;
                    match self.engine.realize_step(&mut self.gpu, packet, candidate, expected) {
                        Ok(true) => cursor.engine += 1,
                        Ok(false) => return Ok(AppPresentStep::Pending),
                        Err(error) => {
                            cursor.phase = AppPresentPhase::Aborted;
                            if self.retained_fault.is_none() {
                                log_debug(&format!("[TRACE] os_host present aborted engine={} fault={error}", cursor.engine));
                                self.retained_fault = Some(format!("engine canvas present: {error}"));
                            }
                            return Ok(AppPresentStep::Pending);
                        }
                    }
                    return Ok(AppPresentStep::Pending);
                }
                cursor.phase = AppPresentPhase::Uploads;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::BeginGpu => {
                let packet = cursor.frame.packet.as_ref().ok_or_else(|| "prepared frame packet was transferred before admission".to_string())?;
                let expected = self.presentation_authority.admitted();
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let token = ui_wgpu::wgpu::UiPresentToken::mint_for_current_thread();
                    if let Err(error) = self.gpu.begin_prepared(&token, &self.gate, packet, expected.scene_revision, expected.input_generation) {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err(format!("prepared frame admission: {error}"));
                    }
                }
                #[cfg(target_arch = "wasm32")]
                {
                    let Some(token) = self.offscreen_token.as_ref() else {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err("browser presentation requires dedicated Worker authority".to_string());
                    };
                    if let Err(error) = self.gpu.begin_prepared_offscreen(token, &self.gate, packet, expected.scene_revision, expected.input_generation) {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err(format!("offscreen prepared frame admission: {error}"));
                    }
                }
                let witness = match self.raster_operation_authority.begin(expected.scene_revision, expected.input_generation) {
                    Ok(witness) => witness,
                    Err(error) => {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err(format!("raster operation admission: {error}"));
                    }
                };
                cursor.raster_witness = Some(witness);
                if let Err(error) = self.gpu.begin_raster_ownership(witness) {
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err(format!("raster ownership admission: {error}"));
                }
                cursor.phase = AppPresentPhase::Ownership;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Ownership => {
                let packet = cursor.frame.packet.as_ref().ok_or_else(|| "prepared frame packet was transferred before raster ownership publication".to_string())?;
                let witness = cursor.raster_witness.ok_or_else(|| "raster operation witness was missing during ownership publication".to_string())?;
                cursor.raster_keep_steps = cursor.raster_keep_steps.saturating_add(1);
                match packet.raster_keep_step(&mut cursor.raster_keep) {
                    ui_wgpu::wgpu::PreparedRasterKeepStepV1::Pending => {}
                    ui_wgpu::wgpu::PreparedRasterKeepStepV1::Key(key) => {
                        if let Err(error) = self.gpu.publish_raster_ownership(witness, key) {
                            cursor.phase = AppPresentPhase::Aborted;
                            return Err(error);
                        }
                    }
                    ui_wgpu::wgpu::PreparedRasterKeepStepV1::Complete => {
                        if let Err(error) = self.gpu.seal_raster_ownership(witness) {
                            cursor.phase = AppPresentPhase::Aborted;
                            return Err(error);
                        }
                        cursor.phase = AppPresentPhase::Engine;
                    }
                }
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Uploads => {
                let packet = cursor.frame.packet.as_ref().ok_or_else(|| "prepared frame packet was transferred before upload".to_string())?;
                if cursor.upload >= packet.uploads().len() {
                    cursor.phase = AppPresentPhase::Stage;
                    return Ok(AppPresentStep::Pending);
                }
                let candidate = cursor.raster_witness.ok_or_else(|| "prepared raster operation witness was missing".to_string())?;
                let expected = self.raster_operation_authority.current().ok_or_else(|| "prepared raster operation authority was empty".to_string())?;
                match self.gpu.apply_prepared_upload_step(packet, cursor.upload, candidate, expected) {
                    Ok(true) => cursor.upload += 1,
                    Ok(false) => {}
                    Err(error) => {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err(error);
                    }
                }
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Stage => {
                let Some(packet) = cursor.frame.packet.take() else { return Err("validated prepared frame packet was missing".to_string()) };
                cursor.witness = match self.gate.stage_presented(packet) {
                    Ok(witness) => Some(witness),
                    Err(packet) => {
                        cursor.frame.packet = Some(packet);
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err("prepared frame presenter witness was already occupied".to_string());
                    }
                };
                cursor.phase = AppPresentPhase::Render;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Render => {
                let Some(input_candidate) = cursor.frame.input_candidate else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame input candidate witness was missing before submit".to_string());
                };
                let runtime = self.runtime.clone();
                let Ok(mut runtime) = runtime.try_lock() else {
                    cursor.input_wait = AppPresentInputWait::RuntimeLock;
                    return Ok(AppPresentStep::RetryRuntime);
                };
                if runtime.interaction.is_none() {
                    drop(runtime);
                    match self.runtime.restore_presenter_interaction_step() {
                        PresenterInteractionStep::Ready | PresenterInteractionStep::Restored => {
                            cursor.input_wait = AppPresentInputWait::None;
                            cursor.input_progress = cursor.input_progress.saturating_add(1);
                            return Ok(AppPresentStep::Pending);
                        }
                        PresenterInteractionStep::Waiting { site, request } => {
                            cursor.input_wait = AppPresentInputWait::InteractionCheckout { site, request };
                            return Ok(AppPresentStep::AwaitingRuntime);
                        }
                        PresenterInteractionStep::RuntimeLock => {
                            cursor.input_wait = AppPresentInputWait::RuntimeLock;
                            return Ok(AppPresentStep::RetryRuntime);
                        }
                        PresenterInteractionStep::Abandoned => {
                            cursor.frame.packet = self.gate.abort_pending();
                            cursor.witness = None;
                            cursor.phase = AppPresentPhase::Aborted;
                            return Err("presenter interaction checkout had no return owner before submit".to_string());
                        }
                    }
                }
                let Some(interaction) = runtime.interaction.as_mut() else { return Ok(AppPresentStep::AwaitingRuntime) };
                let AppInteractionState { shell, input, .. } = interaction;
                match shell.progress_presented_input_candidate(input, input_candidate) {
                    interpreter::PresentedInputCandidateProgress::Ready => cursor.input_wait = AppPresentInputWait::None,
                    interpreter::PresentedInputCandidateProgress::Pending { advanced } => {
                        cursor.input_wait = AppPresentInputWait::SceneIntent;
                        if advanced {
                            cursor.input_progress = cursor.input_progress.saturating_add(1);
                        }
                        return Ok(AppPresentStep::Pending);
                    }
                    interpreter::PresentedInputCandidateProgress::Stale => {
                        drop(runtime);
                        cursor.frame.packet = self.gate.abort_pending();
                        cursor.witness = None;
                        cursor.phase = AppPresentPhase::Aborted;
                        return Ok(AppPresentStep::Pending);
                    }
                }
                if !shell.presented_input_candidate_matches(input_candidate) {
                    drop(runtime);
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.witness = None;
                    cursor.phase = AppPresentPhase::Aborted;
                    return Ok(AppPresentStep::Pending);
                }
                drop(runtime);
                let Some(witness) = cursor.witness.as_ref() else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame presenter witness was missing before submit".to_string());
                };
                let Some(packet) = self.gate.pending_presented(witness) else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.witness = None;
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame presenter witness was stale before submit".to_string());
                };
                let raster_witness = cursor.raster_witness.ok_or_else(|| "raster operation witness was missing before submit".to_string())?;
                if !self.raster_operation_authority.matches(raster_witness) {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.witness = None;
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("raster operation authority was stale before submit".to_string());
                }
                if cursor.gpu_cursor.is_none() {
                    match self.gpu.begin_prepared_present(packet, raster_witness) {
                        Ok(gpu_cursor) => cursor.gpu_cursor = Some(gpu_cursor),
                        Err(error) => {
                            cursor.frame.packet = self.gate.abort_pending();
                            cursor.witness = None;
                            cursor.phase = AppPresentPhase::Aborted;
                            return Err(format!("prepared frame submit admission: {error}"));
                        }
                    }
                    return Ok(AppPresentStep::Pending);
                }
                let Some(gpu_cursor) = cursor.gpu_cursor.as_mut() else { return Ok(AppPresentStep::Pending) };
                let outcome = self.gpu.prepared_present_step(packet, gpu_cursor);
                if let Some((key, version)) = self.gpu.take_missing_world_mesh() {
                    log_debug_once_per_transition("present-missing-world-mesh", true, &format!("[TRACE] os_host prepared world mesh was not resident, draw skipped: key={key} version={version}"));
                } else {
                    log_debug_once_per_transition("present-missing-world-mesh", false, "[TRACE] os_host prepared world mesh residency restored");
                }
                match outcome {
                    Ok(true) => cursor.phase = AppPresentPhase::CloseGpu,
                    Ok(false) => {}
                    Err(error) => {
                        gpu_cursor.begin_close();
                        cursor.frame.packet = self.gate.abort_pending();
                        cursor.witness = None;
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err(format!("prepared frame submit step: {error}"));
                    }
                }
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::CloseGpu => {
                let Some(gpu_cursor) = cursor.gpu_cursor.as_mut() else {
                    cursor.phase = AppPresentPhase::Acknowledge;
                    return Ok(AppPresentStep::Pending);
                };
                gpu_cursor.begin_close();
                if gpu_cursor.close_step() {
                    cursor.gpu_cursor = None;
                    cursor.phase = AppPresentPhase::Acknowledge;
                }
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Acknowledge => {
                let Some(witness) = cursor.witness.as_ref() else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame presenter witness was missing".to_string());
                };
                let Some(packet) = self.gate.pending_presented(&witness) else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame presenter witness was stale before acknowledgement".to_string());
                };
                let raster_witness = cursor.raster_witness.ok_or_else(|| "raster operation witness was missing before acknowledgement".to_string())?;
                if !self.raster_operation_authority.matches(raster_witness) || raster_witness.scene_revision != packet.scene_revision() || raster_witness.preview_generation != packet.preview_generation() {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("raster operation authority was stale before acknowledgement".to_string());
                }
                let runtime = self.runtime.clone();
                let Ok(mut runtime) = runtime.try_lock() else {
                    cursor.input_wait = AppPresentInputWait::RuntimeLock;
                    return Ok(AppPresentStep::RetryRuntime);
                };
                if runtime.interaction.is_none() {
                    drop(runtime);
                    match self.runtime.restore_presenter_interaction_step() {
                        PresenterInteractionStep::Ready | PresenterInteractionStep::Restored => {
                            cursor.input_wait = AppPresentInputWait::None;
                            cursor.input_progress = cursor.input_progress.saturating_add(1);
                            return Ok(AppPresentStep::Pending);
                        }
                        PresenterInteractionStep::Waiting { site, request } => {
                            cursor.input_wait = AppPresentInputWait::InteractionCheckout { site, request };
                            return Ok(AppPresentStep::AwaitingRuntime);
                        }
                        PresenterInteractionStep::RuntimeLock => {
                            cursor.input_wait = AppPresentInputWait::RuntimeLock;
                            return Ok(AppPresentStep::RetryRuntime);
                        }
                        PresenterInteractionStep::Abandoned => {
                            cursor.frame.packet = self.gate.abort_pending();
                            cursor.phase = AppPresentPhase::Aborted;
                            return Err("presenter interaction checkout had no return owner before acknowledgement".to_string());
                        }
                    }
                }
                let Some(interaction) = runtime.interaction.as_mut() else { return Ok(AppPresentStep::AwaitingRuntime) };
                let Some(input_candidate) = cursor.frame.input_candidate else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame input candidate witness was missing before acknowledgement".to_string());
                };
                if !interaction.shell.presented_input_candidate_matches(input_candidate) {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame input authority was stale before acknowledgement".to_string());
                }
                let AppInteractionState { shell, input, .. } = interaction;
                match shell.progress_presented_input_candidate(input, input_candidate) {
                    interpreter::PresentedInputCandidateProgress::Ready => cursor.input_wait = AppPresentInputWait::None,
                    interpreter::PresentedInputCandidateProgress::Pending { advanced } => {
                        cursor.input_wait = AppPresentInputWait::SceneIntent;
                        if advanced {
                            cursor.input_progress = cursor.input_progress.saturating_add(1);
                        }
                        return Ok(AppPresentStep::Pending);
                    }
                    interpreter::PresentedInputCandidateProgress::Stale => {
                        cursor.frame.packet = self.gate.abort_pending();
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err("prepared frame input authority was stale before acknowledgement".to_string());
                    }
                }
                if !shell.acknowledge_presented_input(input, input_candidate) {
                    return Ok(AppPresentStep::Pending);
                }
                cursor.has_animated_primitives = packet.has_animated_primitives();
                cursor.retained_control_deadline = shell.presented_retained_clock_deadline();
                if let Some(now_us) = semio_framework_job::default_now_us() {
                    cursor.shell_clock_deadline = shell.next_chrome_deadline(now_us as f64 / 1_000_000.0, app_now_ms());
                }
                cursor.frame.input_candidate = None;
                let Some(witness) = cursor.witness.take() else { return Err("prepared frame presenter witness was lost during acknowledgement".to_string()) };
                let mut replacement = match self.gate.acknowledge_presented(witness) {
                    Ok(replacement) => replacement,
                    Err(_) => {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err("prepared frame presenter witness was stale or duplicated".to_string());
                    }
                };
                drop(runtime);
                self.raster_operation_authority.release(raster_witness).map_err(str::to_owned)?;
                cursor.raster_witness = None;
                self.retirement = Some(AppPresentedRetirement::commit(replacement.take_previous(), raster_witness));
                cursor.phase = AppPresentPhase::Media;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Media => Ok(AppPresentStep::Pending),
            AppPresentPhase::ProgressAcknowledge => {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(progress) = cursor.frame.job_progress.as_mut() {
                    if !progress.acknowledge_presented() {
                        return Ok(AppPresentStep::Pending);
                    }
                }
                cursor.phase = AppPresentPhase::Fullscreen;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Directives => {
                if self.retirement.as_ref().is_some_and(|retirement|retirement.completed_frame.is_some()){return Ok(AppPresentStep::Pending)}
                if let Some(window) = self.window.as_ref() {
                    apply_window_cursor(window, cursor.frame.cursor, cursor.frame.theme_dark, &mut self.last_cursor);
                }
                let fullscreen = self.window.is_none().then_some(cursor.frame.fullscreen).flatten();
                let Some(mut completed) = self.pending.take() else { return Err("completed presentation cursor was missing".to_string()) };
                let accepted_cursor = completed.frame.cursor;
                let accepted_theme_dark = completed.frame.theme_dark;
                let accepted_generation = completed.frame.generation;
                let cursor_wake = completed.frame.cursor_wake.take();
                let retained_control_deadline = completed.retained_control_deadline;
                let shell_clock_deadline = completed.shell_clock_deadline;
                let has_animated_primitives = completed.has_animated_primitives;
                let retirement = self.retirement.get_or_insert_with(|| AppPresentedRetirement::new(None));
                if retirement.completed_frame.is_some() {
                    completed.frame.cursor_wake = cursor_wake;
                    self.pending = Some(completed);
                    return Err("completed presentation retirement capacity exhausted".to_string());
                }
                let media_slots = std::mem::take(&mut completed.media_slots);
                retirement.completed_frame = Some(completed.frame);
                Ok(AppPresentStep::Complete { generation: accepted_generation, cursor: accepted_cursor, theme_dark: accepted_theme_dark, fullscreen, cursor_wake, retained_control_deadline, shell_clock_deadline, has_animated_primitives, media_slots })
            }
        }
    }

    pub(crate) fn engine_surface_token_at(&self, index: usize) -> Option<engine_canvas::EngineSurfaceToken> {
        self.engine.surface_token_at(index)
    }

    pub(crate) fn begin_engine_surface_close(&mut self, token: engine_canvas::EngineSurfaceToken) -> bool {
        self.engine.begin_surface_close(token)
    }

    pub(crate) fn close_engine_surface_step(&mut self, token: engine_canvas::EngineSurfaceToken) -> Result<bool, String> {
        self.engine.close_surface_step(&mut self.gpu, token)
    }

    pub(crate) fn engine_surface_terminal_is_empty(&self, token: engine_canvas::EngineSurfaceToken) -> bool {
        self.engine.surface_terminal_is_empty(token)
    }

    pub(crate) fn begin_engine_raster_close(&mut self, host_id: &str) -> Result<bool, String> {
        let key = engine_canvas::engine_raster_key(host_id).ok_or_else(|| "engine raster host identifier exceeded fixed credits".to_string())?;
        self.gpu.begin_exact_raster_close(&key)
    }

    pub(crate) fn close_engine_raster_step(&mut self, host_id: &str) -> Result<bool, String> {
        let key = engine_canvas::engine_raster_key(host_id).ok_or_else(|| "engine raster host identifier exceeded fixed credits".to_string())?;
        self.gpu.close_exact_raster_step(&key)
    }

    pub(crate) fn engine_raster_terminal_is_empty(&self, host_id: &str) -> bool {
        engine_canvas::engine_raster_key(host_id).is_some_and(|key| self.gpu.exact_raster_terminal_is_empty(&key))
    }

    pub(crate) fn engine_surfaces_terminal_is_empty(&self) -> bool {
        self.engine.terminal_is_empty()
    }
}
//#endregion 📮️RuntimeMailbox
